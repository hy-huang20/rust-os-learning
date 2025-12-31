# rcoren 异步 timer 开发日志

## 概述

记录追踪开发过程中的想法和实现过程，可能会频繁修改，且**不能**保证所有历史内容的正确性。更新中...

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