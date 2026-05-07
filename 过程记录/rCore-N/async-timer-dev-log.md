# rcoren 异步 timer 开发日志

## 概述

[仓库分支：rCore-N/async-timer](https://github.com/hy-huang20/rCore-N/tree/async-timer)

记录追踪开发过程中的想法和实现过程，可能会频繁修改，且**不能**保证所有历史内容的正确性。更新中...

## 20260507

Commit ID: [6d93efe](https://github.com/hy-huang20/rCore-N/commit/6d93efe27b9eade19cde8bfa2ba697710a9c10db)

之前注意到 User mode async driver benchmark 的 sent bytes 一直是 0，检查发现我仓库里的 uart_load.rs 实现有些问题，user_async_test() 里面 executor::run_until_idle() 返回值用得不对。于是去 rcoren 原作者仓库复制了最新的 uart_load.rs 代码版本。运行结果：

```
>> uart_benchmark
[uart benchmark] Kernel mode driver benchmark begins.
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...        
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...        
Test finished, 17378 bytes sent, 13152 bytes received, 56158 bytes error.
Test finished, 16367 bytes sent, 11249 bytes received, 21408 bytes error.
[uart benchmark] Kernel mode driver benchmark finished.
[uart benchmark] User mode polling driver benchmark begins.
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...        
[uart load] Polling mode, claim result: [0uartx load] Polling mode, claim result: 0x10001000
 0005000
[uart load] err pos: 1, empty read: 3900
Test finished, 19050 bytes sent, 15150 bytes received, 148095 bytes error.
[uart load] err pos: 1, empty read: 4288
Test finished, 20200 bytes sent, 15912 bytes received, 155773 bytes error.
[uart benchmark] User mode polling driver benchmark finished.
[uart benchmark] User mode interrupt driver 
benchmark begin.
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...        
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...        
[uart load] Interrupt mode, claim result: 0x10004000, enable res: 0x0
[uart load] Interrupt mode, claim result: 0x10005000, enable res: 0x0
[uart load] Intr count: 673, Tx: 340, Rx: 333, err pos: 85
Test finished, 4250 bytes sent, 4900 bytes received, 9111 bytes error.
[uart load] Intr count: 653, Tx: 328, Rx: 325, err pos: 108
Test finished, 4100 bytes sent, 4850 bytes received, 12837 bytes error.
[uart benchmark] User mode interrupt driver 
benchmark finished.
[uart benchmark] User mode async driver benchmark begin.
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...        
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...        
[uart load] Async mode, claim result: 0x10004000, enable res: 0x0
[uart load] Async mode, claim result: 0x10005000, enable res: 0x0
[uart load] Intr count: 3787, Tx: 3780, Rx: 7, err pos: -1
Test finished, 59940 bytes sent, 86 bytes received, 0 bytes error.
[uart load] Intr count: 3599, Tx: 3591, Rx: 8, err pos: -1
Test finished, 56943 bytes sent, 94 bytes received, 0 bytes error.
[uart benchmark] User mode async driver benchmark finished.
Shell: Process 1 exited with code 0
```

目前发现只有 user_async_test() 能够做到 0 bytes error，不知道其它三种模式有什么问题。既然有模式能够做到 0 bytes error，我的理解是可以认为串口是没问题的，只是其它三种实现在目前情景下容易出现数据错误。

## 20260506

目前的几个任务：

|任务|状态|备注|
|---|---|---|
|[uart_benchmark 学习](https://github.com/hy-huang20/rust-os-learning/issues/28)|[进行中](https://github.com/hy-huang20/rust-os-learning/commit/c5a87040adc06ab69bb6c7c3b15cd72a1e28aa78)|目前运行 uart_benchmark 还有问题|
|rcoren 异步 timer 验证|未开始|当一轮 executor.poll() 下来 Poll::Pending 的任务的 poll 栈帧会被回收并且状态存到堆上（表现为 sp 退回到 executor.poll() 函数），而不是像线程那样直到任务执行完成前需要维持线程栈|
|写论文大纲|未开始|按照原计划这周就得开始写了，先写 embassy 介绍，rcoren timer 这部分，写成一个 markdown|

## 20260505

Commit ID: [f79afce](https://github.com/hy-huang20/rCore-N/commit/f79afce991370d40c8574bcff33edd56802aff45)

将 suspend_current_and_run_next() 改成了原 rcoren 逻辑，去掉 block_current_and_run_next()，保证不在中断上下文中试图获取 TASK_POOL 锁，**确实解决了内核崩溃问题**，但是 uart_benchmark 运行输出提示的 error bytes 令人在意。

目前没有解决 uart_benchmark 出现的问题。

突然发现曾经运行 uart_benchmark 的记录中，uart_benchmark 的输出结果也是[有问题的](http://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/%E5%A4%8D%E7%8E%B0%E8%BF%87%E7%A8%8B/rCore-N%5BQEMU%5D/chapters/run-rCore-N.md)。也就是说其实还没有真正成功运行过 uart_benchmark ...

可能需要先学习一下 uart_benchmark 的原理。

## 20260504

```
[ERROR 1]: Unsupported trap Exception(InstructionPageFault)! stval = 0x18, sepc = 0x18, sstatus = Sstatus {
                                                    bits: 0x101,
     }, trap frame: TrapContext { x: [0, 18, ffffffffffffed40, 0, 1, 20, 8020ca1c, f0f0f0f0f0f0f0f, 80775680, ffffffffffffee18, 805763f0, 80776060, 0, 807763c0, 4, 6010, 802205b0, 0, 802205b0, 80775690, 5000, 1, 2, 7380, 1, 5040, 8, 18, 807756c0, 80775680, 80775d00, 0], sstatus: Sstatus { bits: 18 }, sepc: 8, kernel_satp: 5040, kernel_sp: 1, trap_handler: 7380 }
[kernel 1] Panicked at src/trap/mod.rs:233 a trap Exception(InstructionPageFault) from kernel!
```

目前对报错的分析：

- 内核崩溃：trap_from_kernel
- 报 InstructionPageFault 时 sepc 为 0x18，说明试图去执行内核虚拟地址空间 0x18 处的指令
- ra 寄存器也恰好为 0x18

寄存器的值：

|CSR|值|备注|
|---|---|---|
|scause||InstructionPageFault|
|stval|0x18||
|sepc|0x18|记录出问题的指令地址|
|sstatus|0x101|bits: usize|

|通用寄存器|值|备注|
|---|---|---|
|zero|0x0|恒 0|
|ra|0x18|返回地址。正常不会落到如此低地址|
|sp|0xffffffffffffed40|栈指针。内核地址空间没有 trap context 页。所以这个地址应该位于 pid 0 的内核栈内|
|gp|0x0|被用来存储 pid。pid 从 0 开始分配，idle 进程没有 tcb 不对应 pid，看来对应 initproc 进程|
|tp|0x1|hart id。对应输出 [ERROR 1] 的 hart|
|t0|0x20||
|t1|0x8020ca1c||
|t2|0xf0f0f0f0f0f0f0f||
|s0|0x80775680||
|s1|0xffffffffffffee18||
|a0|0x805763f0|函数参数|
|a1|0x80776060||
|a2|0x0||
|a3|0x807763c0||
|a4|0x4||
|a5|0x6010||
|a6|0x802205b0||
|a7|0x0||
|s2|0x802205b0||
|s3|0x80775690||
|s4|0x5000||
|s5|0x1||
|s6|0x2||
|s7|0x7380||
|s8|0x1||
|s9|0x5040||
|s10|0x8||
|s11|0x18||
|t3|0x807756c0||
|t4|0x80775680||
|t5|0x80775d00||
|t6|0x0||

如果去看 kernelvec 的代码，这之后的信息都是无效的，也就是这里的 TrapContext 只有 x 字段的值是有效的。

推测是内核中某个函数试图返回时发现 ra 为 0x18 于是跳转到 0x18 去执行，而 0x18 是内核虚拟地址空间的无意义地址，触发了 InstructionPageFault

trap from kernel 来源有两处，一处是 trap_handler 函数里面触发，一处是 idle 进程里面触发。后者应该不可能，因为如果是这样，sp 则应该位于 boot_stack，而不是位于 pid 0 内核栈。所以这里的 trap from kernel 应该是 trap_handler 函数里面的异常。

trap_handler 函数里面的异常来源也有两处，一处是在执行用户系统调用时出现异常，一处是在处理中断时出现异常。

将 async_timer 切换回 rcoren 原来的 timer 仍然可能遇到内核崩溃。要不就是 suspend_current_and_run_next 两段和一段的锅？中断上下文不应该持有 TASK_POOL 锁的锅？

git 切换到最初能正常运行且未改动的状态：

```bash
git checkout embassy-into-rcoren
```

此状态不会遇到内核崩溃，且 uart_benchmark 正常运行输出均为 0 bytes error。基于该状态只改动 suspend_current_and_run_next 将两段逻辑合为一段，看看会不会遇到内核崩溃或者 uart_benchmark 异常。

**不幸的是，结果确实印证了我的猜测**。只是按照上述改动了 suspend_current_and_run_next，就出现了之前遇到的一切问题。运行 hello_world_simple 或 uart_benchmark 均会导致内核崩溃；有时出现未有任何操作内核便崩溃的情况。

**目前结论：不要试图在中断上下文中获取 TASK_POOL 锁**。

如果禁止在中断上下文获取 TASK_POOL 锁的话，目前 sys_sleep 实现似乎难以改动。可能要修改 sys_sleep 的原理？比如改成类似于一个 while 循环套 suspend_current_and_run_next 的实现，而不是引入新的 block_current_and_run_next？这样就可以保留原来 rcoren 的设计，不在中断上下文获取 TASK_POOL 锁。

目前想的 sys_sleep 大概的写法：使用 async_timer 设置一个 timer，引入一个标志位，初值 true，作为 sys_sleep 中 while 的循环条件；while 中跑 rcoren 原 suspend_current_and_run_next 实现；设置的 timer 回调中修改标志位值为 false 打破 while 循环，sys_sleep 结束。

## 20260502

rcoren 每一个 hart 都有自己独立的 idle 进程，但是在 idle 进程中 fetch_task() 时 4 个 hart 都会从同一个 TASK_POOL 里面的同一个 TaskManager 里面的同一个 ready_queue 里面取 TaskStatus::Ready 的任务来运行。所以 fetch_task() 对 TASK_POOL 的访问上了锁 .lock()。  

在 add_task() 中，在获取 TASK_POOL 锁前加上一条 debug!() 日志输出，运行，输出了一会就卡住了，Rust user shell 也卡住无法输入。猜测是 debug!() 遇到死锁。

之前发现 rcoren 的实现硬生生将 suspend_current_and_run_next() 的逻辑拆分成了两段，在中断上下文里只负责 schedule 到 idle 进程，然后在 idle 进程中负责修改任务状态以及 add_task() 到 ready_queue 中。而现在的实现将上面两部分合到一个函数中，并在中断上下文就调用这个合并的函数（rcore 做法）。因为 add_task() fetch_task() 这些都需要获取 TASK_POOL 的锁，在原来的实现中这两个函数也都是仅在 idle 进程调用，所以我猜想**也许原作者不希望在 rcoren 中断上下文中持有 TASK_POOL 锁？**

怎么改回去呢？sys_sleep 设置的 timer 回调里面那个 add_task(tcb) 似乎怎么也没法避免？借助 TaskPool 的接口？TaskPool::wake() 不得不放到中断上下文去，否则中断到来如果仅仅是修改 TaskStatus 的话，sleep 就不知道要等到猴年马月了。但是调用 TaskPool::wake() 又必须先获取 TASK_POOL 锁。

既然这么麻烦，也许一定程度上解释了为什么 rcoren 原作者把 sleep 实现为忙等，把 suspend_current_and_run_next() 别扭地拆成两部分了。

现在并不能确定，在 supervisor timer 中断上下文中试图获取 TASK_POOL 锁，是内核崩溃 bug 的原因，但算是可能的原因之一。

同时发现，使用 async_timer 替代原有 timer 实现后 uart_benchmark 测例出问题了。单核 -smp 1 输出不对，多核 -smp 4 会出现内核崩溃。

## 20260430

### 单核只跑 os 时间片

在 justfile 中先将 qemu 设置 -smp 改成 1 在单核上测试。目前看来单核只跑 os 时间片是没有问题的，截取了一部分调试输出，输出大致按照下面这样循环，**符合预期**：

```
[DEBUG 0]: [EXECUTOR] spawn
[DEBUG 0]: [EXECUTOR] poll
[DEBUG 0]: [TASKREF] poll
[DEBUG 0]: [TIMER] poll
[DEBUG 0]: [TIMERDRIVER] schedule_wake
[DEBUG 0]: [QUEUE] schedule_wake
[DEBUG 0]: [QUEUE] next_expiration
[DEBUG 0]: queue len: 1
[DEBUG 0]: timer.at 24381469 now 24273923
[DEBUG 0]: queue len: 1
[DEBUG 0]: set 24381469 now 24299104
[DEBUG 0]: [TIMERDRIVER] schedule_wake end
[DEBUG 0]: [TIMERDRIVER] on_interrupt
[DEBUG 0]: [QUEUE] next_expiration
[DEBUG 0]: queue len: 1
[DEBUG 0]: timer.at 24381469 now 24387942
[DEBUG 0]: wake_task
[DEBUG 0]: queue len: 0
[DEBUG 0]: set 18446744073709551615 now 24412059       
[DEBUG 0]: [EXECUTOR] poll
[DEBUG 0]: [TASKREF] poll
[DEBUG 0]: [TIMER] poll
[DEBUG 0]: await end
[DEBUG 0]: on_timeout end
[DEBUG 0]: [EXECUTOR] spawn
```

### 单核跑 os 时间片 + sys_sleep

Commit ID: [eb820a1](https://github.com/hy-huang20/rCore-N/commit/eb820a1e66d5ec4d4fa39dccd90deb541d3abfb0)

基于 async_timer 的 sys_sleep 和用户态对接，在用户态 user_lib 实现了一个 sleep_blocking()，添加了一个用户态测试程序 sleep_blocking.rs 和原有 hello_world_simple.rs 的唯一区别是：忙等的 sleep() 换成了 sleep_blocking() 函数。手动输入，多次运行 sleep_blocking.rs 结果，**符合预期**：

```
>> Rust user shell
>> sleep_blocking
[sleep blocing] from pid: 1
current time_msec = 3695
time_msec = 3809 after calling sleep_blocking(period_ms: 100), delta = 114 ms!
Test sleep blocking passed!
Shell: Process 1 exited with code 0
>> sleep_blocking
[sleep blocing] from pid: 2
current time_msec = 7382
time_msec = 7498 after calling sleep_blocking(period_ms: 100), delta = 116 ms!
Test sleep blocking passed!
Shell: Process 2 exited with code 0
>> sleep_blocking
[sleep blocing] from pid: 3
current time_msec = 10100
time_msec = 10215 after calling sleep_blocking(period_ms: 100), delta = 115 ms!
Test sleep blocking passed!
Shell: Process 3 exited with code 0
>> sleep_blocking
[sleep blocing] from pid: 4
current time_msec = 12867
time_msec = 12984 after calling sleep_blocking(period_ms: 100), delta = 117 ms!
Test sleep blocking passed!
Shell: Process 4 exited with code 0
>> sleep_blocking
[sleep blocing] from pid: 5
current time_msec = 15745
time_msec = 15861 after calling sleep_blocking(period_ms: 100), delta = 116 ms!
Test sleep blocking passed!
Shell: Process 5 exited with code 0
```

再添加一个用户态测试程序 sleep_blocking1.rs，里面开 4 个进程跑 sleep_blocking.rs 结果，**符合预期**：

```
>> Rust user shell
>> sleep_blocking1
[sleep blocking 1] from pid: 1
[sleep blocking] from pid: 2
current time_msec = 5158
[sleep blocking] from pid: 3
current time_msec = 5171
[sleep blocking] from pid: 4
current time_msec = 5189
[sleep blocking] from pid: 5
current time_msec = 5196
time_msec = 5272 after calling sleep_blocking(period_ms: 100), delta = 114 ms!
Test sleep blocking finished!
time_msec = 5299 after calling sleep_blocking(period_ms: 100), delta = 128 ms!
Test sleep blocking finished!
time_msec = 5315 after calling sleep_blocking(period_ms: 100), delta = 119 ms!
Test sleep blocking finished!
time_msec = 5317 after calling sleep_blocking(period_ms: 100), delta = 128 ms!
Test sleep blocking finished!
[sleep blocking 1] Test sleep blocking 1 finished!
Shell: Process 1 exited with code 0
```

### 多核跑 os 时间片 + sys_sleep

将 qemu 的 -smp 设置恢复为 4 再运行 sleep_blocking1.rs 的结果（为了不在 /dev/pts/0 看到密密麻麻的 DEBUG 调试信息，可以运行时改成 LOG=ERROR just run，顺便提一下 log 的几个等级从低到高：ALL, TRACE, DEBUG, INFO, WARN, ERROR, FATAL, OFF）：

```
>> sleep_blocking1
[sleep blocking 1] from pid: 2
[sleep blocking] from pid: 3
current time_msec = 20900
[sleep blocking] from pid: 4
current time_msec = 20905
[sleep blocking] from pid: 5
current time_msec = 20909
[sleep blocking] from pid: 6
current time_msec = 20915
time_msec = 21001 after calling sleep_blocking(period_ms: 100), delta = 101 ms!
Test sleep blocking finished!
time_msec = 21006 after calling sleep_blocking(period_ms: 100), delta = 101 ms!
Test sleep blocking finished!
time_msec = 21011 after calling sleep_blocking(period_ms: 100), delta = 102 ms!
Test sleep blocking finished!
time_msec = 21016 after calling sleep_blocking(period_ms: 100), delta = 101 ms!
Test sleep blocking finished!
[sleep blocking 1] Test sleep blocking 1 finished!   
Shell: Process 2 exited with code 0
```

现在令人在意的是，**log 的输出似乎会影响程序的行为**。如上面设置 LOG=ERROR 可以正常运行，但是如果设置为 LOG=INFO 遇到过内核崩溃（不能稳定触发，有时一切正常），如果保持 LOG=DEBUG 遇到过输出了一会儿卡住不动的情况（卡住的地方只输出了一截内容，且内核输出卡住的时候 Rust user shell 这边也会卡住无法输入字符）。

## 20260429

Commit ID: [509f3e9ccd7d3a569c45c5610000681feb768a64](https://github.com/hy-huang20/rCore-N/commit/509f3e9ccd7d3a569c45c5610000681feb768a64)

在 virtualbox 里面配置 rcoren 环境又折腾了一阵。把 Cargo.lock 也上传到 github，且用 cargo 下载工具时一律指定版本加 --locked，这样也许可以保证下次在别的地方配环境构建时容易些吧。

virtualbox 环境配好了，但是不知道为什么我在 /dev/pts/1 里面的输入完全无法到达内核。

终于发现了，**在 /dev/pts/0 启动内核之前需要在另外两个窗口先跑 rCore-N/sleep.sh（之前一直忘记了）**。这样尝试后 wsl 上可以成功运行用户态程序，但在我的 virtualbox 中，/dev/pts/1 无法接收输入。后续还是在 wsl 上开发吧。看来**不是 rcoren 自己实现的问题**，这之前有关的记录基本可以无视，**不过导致内核崩溃的情况应和这个无关**，令人在意。

在 wsl 上，async_timer 分支，试图输入 hello_world_simple 运行可能会遇到内核崩溃：（目前还不知道稳定触发的方法，有时候还在输入 app 名称还没回车时就会触发，有时候加几条 debug!() 又不触发了。而且异常类型也并不总是长下面这样，还遇到过 StorePageFault）

```
[ERROR 1]: Unsupported trap Exception(InstructionPageFault)! stval = 0x18, sepc = 0x18, sstatus = Sstatus {
                                                    bits: 0x101,
     }, trap frame: TrapContext { x: [0, 18, ffffffffffffed40, 0, 1, 20, 8020ca1c, f0f0f0f0f0f0f0f, 80775680, ffffffffffffee18, 805763f0, 80776060, 0, 807763c0, 4, 6010, 802205b0, 0, 802205b0, 80775690, 5000, 1, 2, 7380, 1, 5040, 8, 18, 807756c0, 80775680, 80775d00, 0], sstatus: Sstatus { bits: 18 }, sepc: 8, kernel_satp: 5040, kernel_sp: 1, trap_handler: 7380 }
[kernel 1] Panicked at src/trap/mod.rs:233 a trap Exception(InstructionPageFault) from kernel!
```

hello_world_simple 例子行为很简单，user_lib::sleep 还没有接上 async_timer 实现的 sys_sleep，依然是死循环忙等。所以目前只有 os 时间片在使用 async_timer。代码如下： 

```rust
use user_lib::{getpid, sleep};

#[no_mangle]
pub fn main() -> i32 {
    println!("[hello world] from pid: {}", getpid());
    sleep(100);
    0
}
```

## 20260428

继续 [20260427](#20260427) 的问题。利用这个 bug，冗余输入 hhhhhheelllllooo__wwoorlllllldd，终于成功运行了 hello_world！

有时候会看见形如这样的输出：

```
hhhhhellloo_wollllld: command not found
```

里面的字符正好是那些漏掉的字符。猜想是这部分字符并没有经 /dev/pts/1 给 rcoren 而是给了 wsl 从而输出 command not found 这样的内容。

**由于原来有在 wsl 中成功运行 uart_benchmark 用户程序的[记录](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/%E5%A4%8D%E7%8E%B0%E8%BF%87%E7%A8%8B/rCore-N%5BQEMU%5D/chapters/run-rCore-N.md)，现在运行用户程序出了问题，也有可能是因为本地的 wsl 出问题了。之前电脑 windows 系统因为蓝屏无法启动，使用 u 盘启动盘重装过。**

下一步的想法是把 rcoren 放到 virtualbox linux 虚拟机中去运行，看是不是会出现一样的丢字符的 bug。

## 20260427

这之前的记录基本可以无视。没有将 trap_handler 实现为 async 函数。

### 1. 解决编译错误

Commit ID: [2d6752e45b2a63a7116005e5418f552cdf3d7a29](https://github.com/hy-huang20/rCore-N/commit/2d6752e45b2a63a7116005e5418f552cdf3d7a29)

解决了编译错误。内核能启动了。

此时并没有用实现的 sys_sleep 提供给用户态，用户态的 sleep 仍然是那个忙等。

但是试图运行一个 hello_world_simple 测例时内核崩溃：

**程序提示信息**

```
>> hello_world_simple
Error when executing!
Shell: Process 1 exited with code -4
>> 
```

**内核提示信息**

```
[DEBUG 0]: run_tasks
[DEBUG 0]: Fork start
[DEBUG 0]: forked task cx ptr: 0xffffffffffff9f58
[DEBUG 0]: new_task 1 via fork
[DEBUG 2]: EXEC eo_ordmple
[ WARN 2]: exec failed!
[ INFO 1]: pid: 1 exited with code -4, time intr: 1, cycle count: 20761282
[ERROR 1]: Unsupported trap Exception(IllegalInstruction)! stval = 0x0, sepc = 0x8020ee94, sstatus = Sstatus { bits: 0x101, }, trap frame: TrapContext { x: [0, 8020ee94, ffffffffffffee10, 0, 1, 20, 8020c998, f0f0f0f0f0f0f0f, fffffffffffff000, 7340, 80776020, 1, 1, 80776018, 131ba22, 4db0, 802205b0, 0, c, 5030, 5000, 1, 2, 14eb0, d, 5040, 8, 18, 80775640, 807766a8, 80775600, 0], sstatus: Sstatus { bits: 8020ee94 }, sepc: 0, kernel_satp: 0, kernel_sp: 0, trap_handler: 0 }
[kernel 1] Panicked at src/trap/mod.rs:233 a trap Exception(IllegalInstruction) from kernel!
```

综上目前发现的问题：

- 我键盘输入的是 hello_world_simple，但是 os 调试信息中输出的却是 `EXEC eo_ordmple`，看起来像是随机丢失了一些字母
  - 试了几次发现这个 bug 稳定触发
- 内核因为不明原因崩溃了，看提示信息是跳转到 trap_from_kernel 了，而且错误原因是 IllegalInstruction 异常
  - 试了几次发现并不一定输入某个执行用户 app 后马上就崩溃，但每次确实都会因为一些目前不明的原因崩溃

### 2. rcoren 实现本来就有 bug?

我切换到了内核能运行的最早的版本 [e45c3c1](https://github.com/hy-huang20/rCore-N/commit/e45c3c12579d4a79feefeb2d4381529106a4aee3)，随机丢字符串的问题还是能稳定触发，看来是 rcoren 本身的实现有问题。

## 20260128

组会后记录。从向老师那里确认：

- 将 `trap_handler` 视为一个协程，将其实现为 `async` 函数

所以之前的实现是有问题的，可以无视这之前的记录。

## 20260127

rCore-N 线程内核栈大小 `KERNEL_STACK_SIZE` 为 16 KiB，而 os 的 boot stack 的 大小 `.bss.stack` 的大小为 256 KiB。 

设想：之前的设计在 os 的 `trap_handler` 中进行 wake 和 poll 两个操作（即 `waker.wake()` 和 `executor.run_until_idle()`，在 `AsyncTimer` 的 `interrupt_handler` 中）。现在的设想是在 `trap_handler` 中仅进行 wake 而**将 poll 操作移动到 idle 进程中**。这样，poll 使用的栈就由被中断打断的某个随机线程的内核栈变成**固定**的 os 的 `.bss.stack` 栈。这样的好处是可以减轻线程内核栈的内存压力，因为传统 timer **不得不**在某个随机线程内核栈上运行 `set_next_trigger()` 函数调用，之前的异步 timer 实现也没有避免使用线程内核栈空间的情况。现在准备将 `set_next_trigger()` 移动到 poll 函数内部（否则在之前的实现中异步改写显得多余），将 poll 的调用移动到 idle 进程中。如果能省去这部分在之前看来是**必需**的线程内核栈内存使用，则后续设计线程内核栈的固定大小时便可相应缩减以节省内存使用。而且无论是几千还是几万个 future，对于 `.bss.stack` 栈来说都仅仅只会多一个 future.poll 空间的使用。

但是这样的设计似乎不利于将异步逻辑封装为 crate。因为如果封装成不依赖于 os 的 crate 的话意味着需要暴露 crate 内的 Executor 提供给 os，同时还要修改 os 的 idle 进程的实现。林晨的 async-uart-driver crate 就是在异步串口驱动 `AsyncSerial` 的 `interrupt_handler()`（中断时调用）中同时进行 wake 和 poll 操作。

## 20260113

[commit id: 1f36d22](https://github.com/hy-huang20/rCore-N/commit/1f36d220e19a481e1b1e538caf2bbeb89724b4ec)

之前的想法和实现错了。**不要将 os trap_handler 函数实现为 async 的。**

修改的话，首先应该将 trap_handler 和 timer_interrupt_handler 都改为非 async 的，然后将 `AsyncTimer` 中的 get_async_timer 函数改成非 async 的，名称改为 set_async_timer，函数内逻辑不变。

因为之前的想法存在问题，所以可以几乎无视 [20260107](#20260107) 的记录。

对应的总结记录[参考](https://github.com/hy-huang20/rust-os-learning/blob/800cb07326f4c1c82af5bf2cff41743872305dfa/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rCore-N/async-timer.md)。

## 20260107

[commit id: 8663da0](https://github.com/hy-huang20/rCore-N/commit/8663da00f4ef4c1679071646cabbc7819964c335)

移植林晨的 async-uart-driver 逻辑：**解决了编译错误，目前 os 可以跑起来**

目前还不太明白 rcoren 中的 `hart_id` 的作用

目前的实现可能导致 `AsyncTimerExecutor` 中 `AsyncTask` 的重复，即多个 `AsyncTask` 其实都是服务于一个 `AsyncTimerFuture`，不过目前不影响正常进行，因为在 `AsyncTimer::interrupt_handler()` 中会 wake 所有 `AsyncTask` 为 Ready。后续也许可以参考 write an os in rust 中的做法，建立一个从 `time: usize` 到 `AsyncTask` 的 `BTreeMap`?

## 20260101

之前的设想是，当 timer 触发时，在 os 的 `trap_handler` 中直接修改 `AsyncTask` 的 `AsyncTimerFuture::timeout` 这一 `AtomicBool` 变量，后来发现这在代码上是并不可行的。因为照搬林晨设计的 `Task` struct 而设计的 `AsyncTask` 中：

```rust
pub struct AsyncTask {
    /// The task future
    pub fut: AtomicCell<Pin<Box<dyn Future<Output = ()> + 'static + Send + Sync>>>,
}
```

因为无法知道 `fut` 这个 trait 对象的具体类型，所以无法修改。

于是回看林晨代码。如果按照林晨的设计的话，我不应该在我的 `AsyncTimerFuture` 中维护 `timeout` 字段，而是维护一个 `Arc<AsyncTimer>` driver。那该如何知道这个 future 是否 timeout 了呢？我想应该可以在 `AsyncTimer` 中维护相应的数据结构用来记录，然后 future 通过 `AsyncTimer` 提供的方法从而得知自己是否已经 timeout。

## 20251231

我理解的在内核中编写的异步驱动交互代码执行的一个大致流程：

- `let future = async { ... }` 创建 task 对应的 future
- `future.await` 第一次 poll 该 Future
- 如果完成则直接返回 Poll::Ready 并结束否则返回 Poll::Pending 让出 CPU 给 executor
- 未来某个时刻XX驱动触发中断
- CPU 跳转到 `trap_handler()` 并根据中断原因跳转到XX驱动对应的中断处理程序 `XX_intr_handler()`
- 执行 `XX_intr_handler()` 完成中断处理
- 调用XX驱动中断对应的完成回调 `XX::on_interrupt()`
- 在 `XX::on_interrupt()` 回调中调用关联 Future 的 waker.wake()
waker 调用使内核调度器重新 poll 该 Future

如果是与 timer 驱动交互，想将 timer 改成使用 async/await 的异步的话，初步的想法是在 Future 中维护一个初始为 false 的变量，每当该 Future 被 poll 的时候便检查该变量的值，若为真则返回 `Poll::Ready` 否则返回 `Poll::Pending`。在 `on_interrupt()` 中将 Future 中的该变量设置为 true。

还没有想清楚栈，最终的结果应该是能够省一些栈空间（因为协程是主动让出 cpu，可以自己选择保存哪些量，相比于线程，线程被打断的时机是随机的，所以可能一刀切地保存一些不需要的量）。

可能需要修改 os/task 部分的逻辑，在异步的构想中一个 task 就是一个 future，但是 rcoren 目前还是传统的 task 实现

感觉需要在 os `trap_handler` 中将相应 timer future 中的 timeout 设置为 true 并 wake 所有 wakers

也许可以借助 rcoren 中用户态中断的设计思路，os `trap_handler` 中使用一个“假”的时钟中断。“真”的时钟中断相关的处理逻辑放到各自**对应的 Future 中**去。

## 20251109

rCore-N 中原 timer 实现：

```rust
// os/src/trap/mod.rs
pub fn trap_handler() -> ! {
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // do something
        }
    }
    trap_return();
}
```

修改后大致的样子：

```rust
// os/src/trap/mod.rs

pub fn trap_handler() -> ! {
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // 在中断处理例程中 poll 一次执行器
            timer.executor.poll();
        }
    }
}
```

```rust
// os/src/timer.rs

// 无论是 os 的时间片轮转还是 sys_set_timer 系统调用
// 都会使用 set_virtual_timer 函数
// 这里使用 async 关键字将其改成异步函数
pub async fn set_virtual_timer(mut time: usize, pid: usize) {
    // ...

    // 每个 timer 对应一个 task
    // 注册 waker
    let task = Task::new(/*...*/);
    register_waker(
        unsafe { from_task(task.clone())}
    );
    timer.executor.tasks.push_back(task);

    // ...
}
```

此外还要增加一些代码逻辑：

Timer

```rust
pub struct Timer {
    wakers,
    executor,
}
```

Executor

```rust
pub struct Executor {
    timer_queue,
    run_queue,
    tasks,
}

impl Executor {
    pub fn poll() {
        // 1. dequeue_expired 函数将 timer_queue 中
        // 所有 expired 任务拿出
        // 2. 对于每个被拿出的 task 通过 wake_task_no_pend 函数
        // 将其放入 run_queue
        self.timer_queue.dequeue_expired(wake_task_no_pend);

        // 1. dequeue_all 拿出 run_queue 中的所有任务
        // 2. 在传入 dequeue_all 的回调函数中执行 poll_fn
        self.run_queue.dequeue_all(|p| {
                
            // do something
            task.poll_fn()(p);

            self.timer_queue.update(p);
        });
    }
}
```

Waker

注册 waker 的实现：

```rust
pub fn register_waker(&self, waker: Waker) {
    timer.wakers.lock().push_back(waker)
}
```

waker.rs 的实现可以照搬 [embassy 的做法](https://github.com/embassy-rs/embassy/blob/main/embassy-executor/src/raw/waker.rs)。林晨的[实现](https://github.com/BITcyman/async-uart-driver/blob/main/src/waker.rs)也是以此为参考的：

```rust
//! This mod specific the waker related with coroutine
//!

use super::task::{TaskRef, Task, wake_task};
use core::task::{RawWaker, RawWakerVTable, Waker};

const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake, drop);

unsafe fn clone(p: *const ()) -> RawWaker {
    RawWaker::new(p, &VTABLE)
}

/// nop
unsafe fn wake(p: *const ()) { 
    wake_task(TaskRef::from_ptr(p as *const Task))
}

unsafe fn drop(_p: *const ()) {
    // nop
}

/// 
pub(crate) unsafe fn from_task(task_ref: TaskRef) -> Waker {
    Waker::from_raw(RawWaker::new(task_ref.as_task_raw_ptr() as _, &VTABLE))
}
```