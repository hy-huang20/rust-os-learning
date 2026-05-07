# rCore-N 串口逻辑

## 1. 概述

**os 部分**：`os/src/uart.rs` 中提供了 `BufferedSerial`

**user 部分**：`user/src/user_uart.rs` 中提供了 `BufferedSerial`, `PollingSerial` 和 [`AsyncSerial`](https://github.com/duskmoon314/rCore-N/commit/72bffef189ee4d0fce712ba714c0727f13b7bf45)

关于 user 部分的 `AsyncSerial`

- `Cargo.toml`：使用的 `Executor` 为外部库：
    ```
    executor = { git = "https://github.com/rcore-os/executor" }
    ```
- `Waker` 由 rust 提供：`core::task::Waker`

## 2. 串口逻辑

os 部分的串口逻辑位于 `os/src/uart.rs`，其中实现了 `BufferedSerial`。

`user/src/` 下的文件作为**用户程序库**提供给**用户程序**。用户程序库部分的串口逻辑位于 `user/src/user_uart.rs`，其中实现了 `BufferedSerial`, `PollingSerial` 和 `AsyncSerial`。林晨的[总结](https://github.com/BITcyman/Rust-os-learning/blob/main/rCore-N.md#user_uartrs-%E4%B8%AD%E7%9A%84%E5%90%84%E7%A7%8D%E4%B8%B2%E5%8F%A3%E5%88%86%E6%9E%90)。

#### BufferedSerial

TODO

#### PollingSerial

TODO

#### AsyncSerial

TODO

## 3. 用户程序

用户程序位于 `user/src/bin/` 下。

[林晨记录](https://github.com/BITcyman/Rust-os-learning/blob/main/driver/uart-crate.md#%E7%A7%BB%E5%87%BA-bufferedserial)截图中运行的是 `uart_benchmark.rs` 程序，其中会 spawn `cpu_load.rs` 和 `uart_load.rs`。

### 3.1. cpu_load.rs

作用是被 usoft 通知停止前，作为一个**计算密集型**任务跑满 CPU。

里面有一个 while 死循环，里面不断生成随机数。当收到 usoft 用户态软中断时会修改循环条件从而跳出循环，程序结束。

### 3.2. uart_load.rs

作用是提供了 `KERNEL_MODE`, `POLLING_MODE`, `INTR_MODE`, `ASYNC_MODE` 4 种模式进行测试，作为 **I/O 压力测试**任务跑满 UART。

**看串口是否工作正常应主要看 error bytes 是否为 0**。对于同一个 uart_load 进程，其接收字节数和发送字节数不相等应是正常的。因为同一个 uart_load 进程的收与发，对应的是两条方向相反且逻辑上独立的数据流。

而理论上，对于 UART3 和 UART4，应有 UART3 发出字节数等于 UART4 接收字节数，UART4 发出字节数等于 UART3 接受字节数。因为接收字节数在理论上确实应该等于对端发出的字节数。但是在当前 uart_load.rs 的粗略实现下，两者存在不相等的情况也是正常的。

```rust
bitflags! {
    struct UartLoadConfig: u32 {
        const KERNEL_MODE = 0b1;
        const POLLING_MODE = 0b10;
        const INTR_MODE = 0b100;
        const UART3 = 0b1000;
        const UART4 = 0b10000;
        const ASYNC_MODE = 0b10_0000;
        const ALL_MODE = Self::ASYNC_MODE.bits | Self::KERNEL_MODE.bits | Self::POLLING_MODE.bits | Self::INTR_MODE.bits;
    }
}
```

#### 执行流程

main 首先初始化当前被打断任务 `current_task()` 的 `UserTrapQueue`，然后使能用户态软中断和用户态时钟中断，用一个 while 死循环卡住当前任务进行同步，等待从别的任务到来的用户态软中断通知从而修改循环条件跳出循环继续运行。

```rust
let init_res = init_user_trap();
println!(
    "[uart load] trap init result: {:#x}, now waiting for config init...",
    init_res
);
unsafe {
    uie::set_usoft();
    uie::set_utimer();
}
while !IS_INITIALIZED.load(Relaxed) {}
```

从别的任务 `send_msg` 除了软中断当前任务通知跳出死循环继续运行，还会根据 msg 的值修改 `MODE` 原子变量的值：

```rust
mod user_trap {
    #[no_mangle]
    pub fn soft_intr_handler(_pid: usize, msg: usize) {
        if let Some(config) = UartLoadConfig::from_bits(msg as u32) {
            let mode = config & UartLoadConfig::ALL_MODE;
            MODE.store(mode.bits(), Relaxed);
            if config.contains(UartLoadConfig::UART3) {
                TX_SEED.store(20210821, Relaxed);
                RX_SEED.store(1000000007, Relaxed);
            } else if config.contains(UartLoadConfig::UART4) {
                RX_SEED.store(20210821, Relaxed);
                TX_SEED.store(1000000007, Relaxed);
            } else {
                println!("[uart load] UART config invalid!");
            }
            IS_INITIALIZED.store(true, Relaxed);
        } else {
            println!("[uart load] Invalid config {:#x}!", msg);
        }
    }
}
```


之后，上述提到的四种模式分别对应四个执行函数，根据被软中断修改后的 `MODE` 的值来决定：

```rust
let (rx_count, tx_count, error_count) = match UartLoadConfig::from_bits(MODE.load(Relaxed)) {
    Some(UartLoadConfig::KERNEL_MODE) => kernel_driver_test(),
    Some(UartLoadConfig::POLLING_MODE) => user_polling_test(),
    Some(UartLoadConfig::INTR_MODE) => user_intr_test(),
    Some(UartLoadConfig::ASYNC_MODE) => user_async_test(),
    _ => {
        println!("[uart load] Mode not supported!");
        (0, 0, 0)
    }
};
```

test 返回后，如果

```rust
if irq_to_serial_id(UART_IRQN.load(Relaxed)) == 3 {
    sleep(100);
}
```

这里是什么意思？可能和 UART4 对应 -serial 的 tcp client 端有关？

#### KERNEL_MODE: kernel_driver_test()

使用 os 提供的 `sys_read` `sys_write` 系统调用读写串口，最终会走到 os 的 BufferedSerial。读串口 `rx_fd` 和写串口 `tx_fd` 是同一个 fd, 但是由于 `UART` 是**全双工**的，所以即使读写同一个 fd, 读和写也是相互独立互不影响的。

使用 `uart_load.rs` 进行测试一般是 spawn 两个 `uart_load` 任务，并通过 `send_msg()` 设置两个任务的 `MODE` 为一个 `UART3` 一个 `UART4`。

之前在 justfile 里使用了 5 个 -serial：

```makefile
# machine, supervisor, user, echo1, echo2
SERIAL_FLAGS := "-serial /dev/pts/0 -serial /dev/pts/1 -serial /dev/pts/2 -serial tcp::23334,server,nowait -serial tcp:localhost:23334"
```

对应关系是：

|SERIAL_FLAGS 第 X 个 -serial|设置|对应|备注|
|---|---|---|---|
|1|/dev/pts/0|内核输出如 debug!()|内核输出直接走 sbi::console_putchar() 不受 uart.rs 管理|
|2|/dev/pts/1|Rust user shell|可以看到 uart.rs 里面的 serial_config::SERIAL_NUM 明确定义为 4，而且 serial_config::irq_to_serial_id() 函数里面只有 4 个 plic 中断号对应的串口号，**对应的就是后面这 4 项**。从 serial id 0 开始|
|3|/dev/pts/2||serial id 1|
|4|tcp::23334,server,nowait|uart_load 里的 UART3|UART_IRQN 设置为 14，对应 serial id 2|
|5|tcp:localhost:23334|uart_load 里的 UART4|UART_IRQN 设置为 15，对应 serial id 3|

所以这里的 UART3 和 UART4 实际上是接起来的，两个 uart_load 分别是全双工 uart 的两端。这样就明白了，UART3 把 tx_buf 里面的东西 write 给 UART4，从 UART4 去 read 数据到 rx_buf。至于正确性验证，则是基于两端用的对称交换的随机种子，相同的种子会生成相同的序列。

```rust
// 消耗掉可能的残余数据
while read(rx_fd, &mut rx_buf) > 0 {}
sleep(20);
let time_us = get_time() * 1000;
// 到时间后触发 utimer
// 执行 user_trap::timer_intr_handler()
// 修改下面的循环条件退出循环
set_timer(time_us + TEST_TIME_US);
while !(IS_TIMEOUT.load(Relaxed)) {
    // 开测 ...
    // write()
    // read()
    // 读逻辑实现为可以有限地处理串口丢字节问题
    // 如果是重复字节/多了字节/字节被污染问题
    // 这里的逻辑就处理不了了
}
```

#### POLLING_MODE: user_polling_test()

#### INTR_MODE: user_intr_test()

#### ASYNC_MODE: user_async_test()

### 3.3. uart_benchmark.rs

`uart_benchmark` 希望测试 cpu 繁忙时的 I/O 任务运行情况，于是会先 spawn 一个 `cpu_load` 进程。然后测试 4 种模式的串口逻辑，每个模式均通过开两个 uart_load 进程。

这里 send_msg 的作用简单来说就是**通知 uart_load 开测以及测试使用的模式**。send_msg 向目标进程 user trap queue 中插入一条 UserTrapRecord，当目标进程执行 os trap_handler 试图 trap_return 时执行真的用户态软中断。uart_load.rs 中用户定义了相应 soft_intr_handler，并且运行 LOG=DEBUG just run 时 soft_intr_handler 里面的 board_qemu 条件编译是被启用了的，可以去看 justfile。
