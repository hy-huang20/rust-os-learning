# rcoren 异步 timer 开发

## 1. async_timer mod

几乎将 embassy 的 Timer 移植过来。参考[这篇记录](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rCore-N/embassy-timer-into-rcoren.md)。

- 动态 Vec 存储 TaskRef
  - embassy 可选集成的 TaskRef 链表或定长的 heapless::Vec 存储 TaskRef，这里均没有使用
- 使用 S 态软件中断的中断模式 Executor
  - 如需实现线程模式 Executor 可以在 rcoren 另起一个内核线程（也是放进用户线程们的那个 ready_queue 统一管理只是相比用户线程可以访问内核数据），上面运行一个类似 block_on 的 Executor
- TaskRef 和 Waker 转换实现任务和 Waker 的对应
- 每个 hart 有自己独立的 TimerDriver 和 Executor

## 2. async_timer 与 os 的对接

在 S 态时钟中断中调用 async_timer::on_time_interrupt() -> TimerDriver::check_alarm() -> 修改 TaskRef 状态 -> pend S 态软件中断

在 S 态软件中断中执行 executor.poll()

注意：需要区分 rcoren 的任务和 async_timer 的 TaskRef，不是一个东西

## 3. 提供给用户态的 sleep

在 rcoren 中，提供给用户态的 sleep 函数为忙等实现。

在 rcore 中，提供给用户态的 sleep 函数，设置 timer，设置当前任务为 Block 并让出 cpu，等到 timer 超时将对应任务状态设置为 Ready 并放入 ready_queue 中

这里的目标是在 rcoren 中为用户态提供一个基于 rust future 的 sleep 函数。

rcoren 没有 rcore 的线程模型，因此最小执行单位是进程。

都是三种 TaskStatus，但与 rcore 相比 rcoren 任务没有 Block 状态而是变成了 Zombie 状态。为了实现 sleep 需要添加一个 Block 状态。相应地需要添加一个 block_current_and_run_next() 函数实现，sleep 不能使用 suspend_and_run_next() 函数。

rcoren 中没有 sys_sleep 系统调用实现，这里需要加上。在新实现的 sys_sleep 中需要具体做：

- `add_timer(ms, on_timeout)` 注册一个 timer。这部分和我实现的 async_timer mod 对接
  - `on_timeout()` 回调中负责唤醒任务后将其 TaskStatus 设置为 Ready 并加入 ready_queue
- `block_current_and_run_next()` 将当前任务状态改为 Block 并让出 cpu

### 关于 rcoren 中的 xxx_current_and_run_next()

rcoren 中的 xxx_current_and_run_next() 和 rcore 中的实现有所区别。

rcoren 中每个 hart 都有一个 idle 进程，执行 hart 对应的 Processor::run() 函数

```rust
impl Processor {
    pub fn run(&self) {
        loop {
            if let Some(task) = fetch_task() {
                // unsafe { riscv::asm::sfence_vma_all() }
                self.run_next(task);
                // __switch inside run_next
                // debug!("idle");
                self.suspend_current();
            }
        }
    }
}
```

rcoren 中的 suspend_current_and_run_next() 也和 rcore 中不同。在 rcoren 中它只做一件事，就是调用 schedule() 将上下文切换到 idle 进程上下文，也就是上面代码中的 self.run_next(task) 调用的末尾处，而将原任务的状态设置为 Ready 以及加入 ready_queue 则在 self.suspend_current() 中完成。而 rcore 中则将上述的两段过程全都合在 suspend_current_and_run_next() 中了。至于为什么 self.suspend_current() 还能够知道原来的任务，是因为在 rcoren 的 suspend_current_and_run_next() 中并没有把原任务 take 掉，调用的是 current_task() 而不是像 rcore 中那样调用 take_current_task() 函数：

```rust
pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    task_inner.time_intr_count += 1; // 我觉得这句大概是个历史遗留问题，不用管它
    let task_cx_ptr = task_inner.get_task_cx_ptr();
    drop(task_inner);

    // jump to scheduling cycle
    schedule(task_cx_ptr);
}
```

综上，为 rcoren 设计 block_current_and_run_next() 时不能将 rcore 中的实现照搬过来。而且这样的话就会发现，在 rcoren 中设计一个 block_current_and_run_next() 就会特别不方便。

### 解决

将 rcoren 中的两段式逻辑按照 rcore 那样改回去。这样 block_current_and_run_next() 也可以直接使用 rcore 中的实现了。

### 更新：tcb 中某些字段值的历史遗留问题

rcoren 中的 tcb_inner.time_intr_count 的计数方式本身就有问题，这个字段仅在每次调用 suspend_current_and_run_next() 时加 1。按照 rcoren 实现，这个思路是有问题的，因为只要简单搜索一下就会发现 suspend_current_and_run_next() 不止在 S 态时钟中断到来时被调用。

rcoren 中 tcb_inner.total_cpu_cycle_count 的计数思路没问题，但是按照其实现的话会少记录一段，因为其在 exit_current_and_run_next() 中并没有更新该字段的值。

上述已经发现的两个历史遗留问题看起来似乎不重要，因为第一个字段没看到被用过，第二个字段也仅限于 exit_current_and_run_next() 里面的一句输出，不是重点。

## 4. OS 时间片逻辑与 async_timer

除了 os 时间片逻辑，其余在 timer_interrupt_handler 中的逻辑都可以不太费劲地使用 async_timer 重写。

感觉不太建议将 os 时间片逻辑强行改成使用 async_timer，因为 os 时间片调用的 suspend_current_and_run_next() 的上下文会出问题。旧逻辑的 current_task() 是某个被时钟中断打断的某用户任务，而使用 async_timer 的话 current_task() 则变成了被 pend 的软件中断打断的某用户任务。

但如果不改的话也会出问题。因为旧的 timer_interrupt_handler() 逻辑和 async_timer 逻辑都涉及到对 sbi::set_timer() 的调用，这样会起冲突，二者只能存在一个。

### 解决方法

我的想法是，放弃 __pender()，放弃使用 SupervisorSoft 中断，也即放弃 embassy-executor 的中断模式。在 SupervisorTimer 中，async_timer::on_timer_interrupt() 后紧跟着执行 async_timer::executor_poll()。这样回调函数所在上下文就和旧 timer 逻辑一致了。

这样设计就有些往林晨的设计那边走了的感觉，即在同一个中断上下文中处理 wake 和 poll 工作。

但这样又出现了一系列新的问题...

如果将 wake 和 poll 放在同一个中断上下文中工作，就会出现 executor poll 到中途的 os 时间片 future 时上下文就切换走了。

目前的解决办法是避免在 executor poll 中途执行 suspend_current_and_run_next() 切走上下文，取而代之置位一个 AtomicBool，等 executor poll 结束后，紧跟着检查这个 AtomicBool 的值根据其值决定是否调用 suspend_current_and_run_next()。这样既保证 executor poll 不会被中途打断，也保证 suspend_current_and_run_next() 确实是从被当前 SupervisorTimer 打断的任务切换出去的，上下文没问题。

目前的实现是在 Executor::spawn() 中会调用 Executor::poll()。这个对于 os 时间片会出问题。因为 async_timer 实现的 os 时间片逻辑，在 on_timeout 回调中又会调用 Executor::spawn()，而 on_timeout 回调又是在 Executor::poll() 中执行。于是这会导致 Executor::poll() 重入问题。

解决方法和上面一样，也利用上面的那个 AtomicBool 标志位，把原本回调中涉及到 Executor::spawn() 的那句也移到 executor poll 结束后。一个标志位解决两个问题。
