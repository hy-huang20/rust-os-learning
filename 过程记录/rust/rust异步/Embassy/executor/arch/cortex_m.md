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
    - 可以拥有多个中断优先级的 Executor：[示例程序](https://github.com/embassy-rs/embassy/blob/ec812d3e66e699db671f262239dc7c277c6cef2d/examples/stm32f0/src/bin/multiprio.rs)

### 2.1. Thread mode executor

`Executor::run()` 进行 spawn task 后进入循环。在循环中每 `poll()` 一次便执行 `wfe` 进入低功耗模式，等待别处的 `sev` 唤醒；被别处的 `sev` 唤醒后继续 `poll()`，周而复始：

```rust
impl Executor {
    pub fn new() -> Self { /* ... */ }

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

>**注意**：用户不会直接调用这里的 `Executor::new()` 和 `Executor::run()`，这两个函数是 `#[embassy_executor::main]` 和 `#[embassy_executor::task]` 在宏展开后的代码中被调用。这里有一份宏展开后的代码供[参考](https://github.com/hy-huang20/rust-learning/blob/embassy-learning/embassy-learning/src/expand.rs)，同目录下另一文件是其展开前的形式。

### 2.2. Interrupt mode executor

```rust
impl InterruptExecutor {
    pub unsafe fn on_interrupt(&'static self) {
        let executor = unsafe { (&*self.executor.get()).assume_init_ref() };
        executor.poll();
    }
}
```

简单流程是：

```
触发中断 -> interrupt handler -> InterruptExecutor::on_interrupt()
```

`executor.poll()` 会从 `raw::Executor` 的 `poll()` 执行下去，后续逻辑在之前关于 src/raw/mod.rs 的学习记录中已经分析过了。

### 2.3. __pender

`__pender` 会在 wake 时被调用：

```rust
Waker::wake() /* dynamic dispatch */ ->  wake_task() -> SyncExecutor::pender.pend()
```

#### 线程模式

```rust
#[export_name = "__pender"]
#[cfg(any(feature = "executor-thread", feature = "executor-interrupt"))]
fn __pender(context: *mut ()) {
    unsafe {
        let context = context as usize;

        #[cfg(feature = "executor-thread")]
        if !cfg!(feature = "executor-interrupt") || context == THREAD_PENDER {
            core::arch::asm!("sev");
            return;
        }
    }
}
```

如果是在中断模式而非线程模式下，`context` 的初值 `THREAD_PENDER` 会被修改为 irq number，从而无法进入 if。线程模式下的 `__pender` 执行 `sev` 指令唤醒 `wfe` 的 cpu，从 `wfe` 处继续执行。

#### 中断模式

```rust
#[export_name = "__pender"]
#[cfg(any(feature = "executor-thread", feature = "executor-interrupt"))]
fn __pender(context: *mut ()) {
    unsafe {
        let context = context as usize;

        #[cfg(feature = "executor-interrupt")]
        {
            use cortex_m::interrupt::InterruptNumber;
            use cortex_m::peripheral::NVIC;

            #[derive(Clone, Copy)]
            struct Irq(u16);
            unsafe impl InterruptNumber for Irq {
                fn number(self) -> u16 {
                    self.0
                }
            }

            let irq = Irq(context as u16);

            // STIR is faster, but is only available in v7 and higher.
            #[cfg(not(armv6m))]
            {
                let mut nvic: NVIC = core::mem::transmute(());
                nvic.request(irq);
            }

            #[cfg(armv6m)]
            NVIC::pend(irq);
        }
    }
}
```

中断模式下的 `__pender` 提供了在不同 arm cortex_m 处理器架构下触发中断的方法。无论是 cortex_m v6m 还是 cortex_m v7 及以上，这段代码的作用都是**触发中断**，从而运行 `InterruptExecutor::on_interrupt()`。

## 3. 参考

- [赵方亮的 embassy 学习记录](https://github.com/zflcs/learning/blob/main/notes/embassy%E5%AD%A6%E4%B9%A0%E7%AC%94%E8%AE%B0.md)