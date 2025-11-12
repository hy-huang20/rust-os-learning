# src/arch/cortex_m.rs

## 1. 一些概念

- `risc-v` 和 `arm`
    - 都是指令集架构
- `cortex_m`
    - 一系列支持 `arm` 指令集架构的处理器核心，是一种 CPU，并非一个完整的芯片
- `stm32`
    - 基于 `cortex_m` 的 32 位单片机，是一个完整的芯片
    - 集成了 `cortex_m` CPU 核心 + 存储器 + 时钟电路 + 各种外设 + ...
- `wfe` 和 `sev`
    - Wait For Event 和 Set Event
    - 都是 `arm` 指令
    - `wfe` 让当前 cpu 进入低功耗等待状态
    - 在多核系统中，`sev` 会唤醒所有处于 `wfe` 状态的 cpu
- `critical_section::with()`
    - 创建一个临界区，确保其闭包内的代码在单核系统上不会被中断打断，在多核系统上临界区不会被其他核心同时访问，从而实现对共享资源的原子性访问


## 2. 源码分析

src/arch/cortex_m.rs 在封装了 raw::Executor 的基础上提供了能够在 ARM/Cortex-M 上运行的 Executor。源码提供了以下 3 部分：

- `__pender` 函数
- 以**线程模式**工作的 Executor
    - Executor 的最低优先级
    - 依赖 WFE/SEV
    - 优点：简单、低功耗
- 以**中断模式**工作的 Executor

### 2.1. __pender

`__pender` 会在 wake 时被调用。线程模式下的 `__pender` 执行 `sev` 指令唤醒 `wfe` 的 cpu，从 `wfe` 处继续执行：

```rust
impl Executor {
    pub fn run(&'static mut self, init: impl FnOnce(Spawner)) -> ! {
        init(self.inner.spawner());

        loop {
            unsafe {
                self.inner.poll();
                asm!("wfe");
            };
        }
    }
}
```

`self.inner` 即为封装的 `raw::Executor`，在 embassy_executor/src/raw/mod.rs 中，如需深入研究其逻辑可以参考[我之前的 embassy 学习记录](https://github.com/hy-huang20/rust-os-learning/tree/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy)。简单地说，`poll()` 会清空当前 `RunQueue` 并为清出来的每个 task 执行其 `poll_fn`。

### 2.2. Thread mode executor

TODO

>**注意**：用户不会直接调用这里的 `Executor::new()` 和 `Executor::run()`，这两个函数是 `#[embassy_executor::main]` 和 `#[embassy_executor::task]` 在宏展开后的代码中被调用。这里有一份宏展开后的代码供[参考](https://github.com/hy-huang20/rust-learning/blob/embassy-learning/embassy-learning/src/expand.rs)，同目录下另一文件是其展开前的形式。

### 2.3. Interrupt mode executor

TODO