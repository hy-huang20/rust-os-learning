# rcoren 异步 timer 开发日志

## 概述

[仓库分支：rCore-N/async-timer](https://github.com/hy-huang20/rCore-N/tree/async-timer)

记录追踪开发过程中的想法和实现过程，可能会频繁修改，且**不能**保证所有历史内容的正确性。更新中...

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