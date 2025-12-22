# rcoren 中的用户态中断

## 操作系统部分：os/

### os/src/trap/usertrap.rs

```rust
pub struct UserTrapRecord {
    /// 发起这条 record 的 task 的 pid
    /// 不过如果是“真的”用户态软中断则为 pid << 4（这样做的目的见下文）
    pub cause: usize,
    /// 如果是用户态时钟中断则设置为 get_time_us() 表示用户态中断从硬件触发的时间
    /// 如果是用户态外部中断则设置为 PLIC 的 irq 编号（标识外部设备）
    /// 如果是“真的”用户态软中断则具体含义可由用户自定义
    pub message: usize,
}
```

### os/src/syscall/process.rs

和用户态中断相关的几个 syscall：

- `sys_init_user_trap`

    - 调用当前 task 的 tcb inner 的 `init_user_trap` 初始化 `UserTrapQueue`

- `sys_send_msg`

    - 用户态软中断相关
    - 向目标 task (pid) 的 `UserTrapQueue` 插入一条 `UserTrapRecord`
    - `UserTrapRecord::cause` 设置为 `pid << 4`，从而保证 `cause & 0xF == 0`，于是通过 `sys_send_msg` 插入的 record 都是“**真的**”用户态软中断，在“真的”用户态软中断中通过 `cause >> 4` 获取发起这条 record 的 task 的 pid

- `sys_set_timer`

    - 用户态时钟中断相关
    - 提供给用户调用，用户可指定未来的某个时刻打断自己并执行自己提供的 `timer_intr_handler`

- `sys_claim_ext_int`
- `sys_set_ext_int_enable`

## 用户程序部分：user/

### 用户程序库：用户程序入口

相关代码在 user/src/ 下的 lib.rs 和 linker.ld 中。lib.rs 中的 `_start` 函数地址被设置到用户程序的 `.text.entry` 处作为用户程序的入口，即用户程序最开始会从 `_start` 执行。`_start` 中将用户态 csr 中的 `utvec` 寄存器设置为同文件夹下 trap.asm 汇编中的 `__alltraps_u`，也即指定了用户态 trap 的入口。这里设置了 TrapMode 为 `Direct` 模式，表示所有 trap 都跳转到 `__alltraps_u` 同一个地址（与之相对的 `Vectored` 模式则是让不同 trap 跳转到不同地址）。这个 `__alltraps_u` 后面马上会提及。

除此之外 _start 还依次做了以下几件事：

- 初始化用户程序堆空间，大小 `0x8000` 字节
- 执行 `main` 函数

同文件下提供了一个 weak 的（即 `#[linkage = "weak"]`）、默认版本的 `main` 函数，需要被用户程序开发者自己的 `main` 覆盖。

### 用户态中断何时何处触发

在 os/src/trap/mod.rs 中的 `init` 函数中设置了三种中断的 `sideleg`，可委托给 U 态处理。这个 `init` 函数会在 `rust_main` 中被调用。

#### 用户态软中断触发

触发用户态软中断（“假的”软中断）应该是专门为了处理 `UserTrapQueue` 中的 `UserTrapRecord` 的，而这些 record 中可能包含“真的”软中断。这里的“假的”“真的”的意思是，**实际**的用户态软中断处理工作而不是处理 record 的过程在哪里被执行。

**每次 os 的 trap_handler 被调用时**，在 os 的 trap_handler 的最后一步调用是 `trap_return` 函数，`trap_return` 会调用 `current_task()`（也就是当前被 os trap 中断的任务）的 tcb inner 中的 `restore_user_trap_info` 函数，检查 `UserTrapInfo` 中的 `UserTrapQueue`，如果非空，则调用 `uip::set_usoft()` 触发“假的”用户态软件中断处理 `UserTrapQueue` 中的 `UserTrapRecord`。

值得注意的是这里的特权级切换。`trap_return` 调用时处于 S 态，在触发用户态中断后会短暂回到 U 态处理用户态中断逻辑，处理完成后会再次回到 S 态的 `trap_return` 函数中，然后 `trap_return` 执行完成后回到被 S 态中断打断前的 U 态代码中去。

#### 用户态外部中断触发

#### 用户态时钟中断触发

在 os 的 `trap_handler` 中通过 `sip::set_utimer()` 设置 CSR 触发用户态的时钟中断。

### 用户程序库：user trap handler

与用户态中断 trap_handler 相关的代码在 rcoren 的用户程序库 user/src/ 下，相关文件为 trap.asm 和 trap.rs。和 os 的 trap_handler 类似，当触发用户态中断的时候，跳转到 `utvec` CSR 中的地址也即上文中设置的 `__alltraps_u` 去执行。trap.asm 中一小段汇编代码 `__alltraps_u` 执行和课程实验中介绍过的、S 态中断发生时类似的工作（csr 、保存上下文），并跳转到 `user_trap_handler`，位于同文件夹下的 trap.rs。在 `user_trap_handler` 中处理了三种用户态中断：

- 用户态软中断
- 用户态外部中断
- 用户态时钟中断

在 `user_trap_handler` 中，用户态软中断会处理当前在 `UserTrapQueue` 中排队的所有 `UserTrapRecord` 并且清空队列，根据这些 `UserTrapRecord::cause` 的值调用以下三种具体的 handler。但其实用户程序库提供的实现并没有真正对这些 record 做什么，位于同文件中的所谓的具体的 handler 函数 `ext_intr_handler`, `soft_intr_handler`, `timer_intr_handler` 只是输出了相应中断的简单信息而已。上述 3 个处理函数都是 `#[linkage = "weak"]` 弱链接的，这表明它们只是默认的实现，如果提供了有实际功能的实现（同名的强符号）则这些默认实现会被覆盖掉而不是触发链接错误。而提供有实际功能的 handler 实现的工作则交给了用户程序开发者，或者说，允许用户程序开发者自定义这些实现，可以在 rcoren 测例文件中（即 user/src/bin/ 下的程序）发现一些例子。

在 `user_trap_handler` 中，用户态外部中断和用户态时钟中断会分别调用对应的 handler 函数 `ext_intr_handler` 和 `timer_intr_handler`。

### 用户程序示例

一些用户程序似乎不是单独运行的，而是和其它某些用户程序协同运行。

#### 用户态软中断：例子

#### 用户态外部中断：例子

#### 用户态时钟中断：以 user/src/bin/hello_world.rs 为例

```rust
static IS_TIMEOUT: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub fn main() -> i32 {
    println!("[hello world] from pid: {}", getpid());
    sleep(1000); // 用户程序库提供的忙等 sleep
    let init_res = init_user_trap(); // 初始化用户态 trap，主要就是初始化 UserTrapQueue
    println!(
        "[hello world] trap init result: {:#x}, now using timer to sleep",
        init_res
    );
    unsafe { // 使能了之后，后续 sip::set_utimer() 触发时钟中断才会跳转到 utvec
        uie::set_usoft();
        uie::set_utimer();
    }
    let time_us = get_time() * 1000;
    set_timer(time_us + 1000_000); // 设置一个 U 态时钟中断在 1000_000 us 后打断自己，执行下面的 timer_intr_handler 
    while !IS_TIMEOUT.load(Relaxed) {} // 在下面的 timer_intr_handler 中原子地修改循环条件跳出循环
    println!("[hello world] timer finished, now exit");

    0
}

#[no_mangle]
pub fn timer_intr_handler(time_us: usize) { // 将会覆盖用户程序库中的默认 timer_intr_handler
    println!(
        "[user trap default] user timer interrupt, time (us): {}",
        time_us
    );
    IS_TIMEOUT.store(true, Relaxed);
}
```

