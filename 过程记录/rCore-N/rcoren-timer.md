# rCore 的 timer

## 1. rCore timer 

### 1.1. 概述

关于**时钟中断**：

>RISC-V 要求处理器维护时钟计数器 ``mtime``，还有另外一个 CSR ``mtimecmp`` 。 一旦计数器 ``mtime`` 的值超过了 ``mtimecmp``，就会触发一次时钟中断。

### 1.2. 执行流程

触发时钟中断时会执行到 ``trap_handler``：

```rust
// os/src/trap/mod.rs
pub fn trap_handler() -> ! {
    // ...
    match scause.cause() {
        // ...
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // 执行这里的逻辑 ...
        }
        // ...
    }
    // ...
}
```

## 2. rCore-N 和 rCore-2025S

rCore-2025S 的 timer 比较简单，而且只有 os 能设置 timer，主要是 os 用来实现[抢占式分时多任务](https://learningos.cn/rCore-Tutorial-Guide-2025S/chapter3/4time-sharing-system.html)。但 rCore-N 提供了一个系统调用 ``sys_set_timer`` 使得**用户程序**也可以设置 timer。这个 timer 是用户程序用来设置在未来某个时间点**中断自己**的。

TODO：概括 sys_set_timer 的功能

以下列出了 rCore-N 和 rCore-2025S 关于 timer 实现上的一些区别：

### 2.1. set_next_trigger

os 任务切换调度时用到的函数。

#### rCore-2025S

rCore-2025S 的比较简单，直接使用了 ``sbi`` 子模块提供的 ``set_timer`` 接口设置 CSR ``mtimecmp`` 的值。 ``time::read()`` 读取当前 ``mtime`` 的值，而 ``CLOCK_FREQ`` / ``TICKS_PER_SEC`` 为 10ms 内计数器的增量，结果是 10ms 后会继续触发一个时钟中断。

```rust
// os/src/timer.rs
pub fn set_next_trigger() {
    set_timer(time::read() + CLOCK_FREQ / TICKS_PER_SEC);
}
```

#### rCore-N

```rust
// os/src/timer.rs
pub fn set_next_trigger() {
    set_virtual_timer(time::read() + CLOCK_FREQ / TICKS_PER_SEC, 0);
}
```

注意到通过 ``set_next_trigger`` 途径设置的时钟中断对应的 ``pid`` 均为 0。

### 2.2. rCore-N 的 set_virtual_timer

rCore-N 有而 rCore-2025S 没有这个函数。``set_virtual_timer`` 实现：

```rust
// os/src/timer.rs

lazy_static! {
    pub static ref TIMER_MAP: [Arc<Mutex<BTreeMap<usize, usize>>>; CPU_NUM] = Default::default();
}

pub fn set_virtual_timer(mut time: usize, pid: usize) {
    if time < time::read() {
        warn!("Time travel!");
        // return;
    }
    let mut timer_map = TIMER_MAP[hart_id()].lock();
    while timer_map.contains_key(&time) {
        time += 1;
    }
    timer_map.insert(time, pid);
    if let Some((timer_min, _)) = timer_map.first_key_value() {
        if time == *timer_min {
            set_timer(time);
        }
    }
}
```

每个 cpu 核心都对应一个 ``timer_map``，为一个 ``BTreeMap`` 类型，注意其会按照**键**的顺序存储元素。如果此时插入的 ``time`` 已经在 ``timer_map`` 中存在则不断加 1 直到找到一个空闲的时间点；和 rCore-2025S 不同的是，当且仅当待插 ``time`` 比当前 ``timer_map`` 中所有 time 值小（早），将会在 ``time`` 时触发一次时钟中断。

TODO

### 2.3. 通过 sys_set_timer 调用 set_virtual_timer

rCore-N 实现了而 rCore-2025S 没有实现这个系统调用。``sys_set_timer`` 实现：

```rust
// os/src/syscall/process.rs
pub fn sys_set_timer(time_us: usize) -> isize {
    let pid = current_task().unwrap().pid.0;
    use crate::config::CLOCK_FREQ;
    use crate::timer::{set_virtual_timer, USEC_PER_SEC};
    let time = time_us * CLOCK_FREQ / USEC_PER_SEC;
    set_virtual_timer(time, pid);
    0
}
```

### 2.4. trap_handler

#### rCore-2025S

```rust
// os/src/trap/mod.rs
pub fn trap_handler() -> ! {
    // ...
    match scause.cause() {
        // ...
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // 设置下一次时钟中断发生
            set_next_trigger();
            // 把所有过期 timer 对应 task 设置为 Ready 并加到 ready_queue 中
            check_timer();
            // task 切换
            suspend_current_and_run_next();
        }
        // ...
    }
    // ...
}
```

#### rCore-N

逻辑复杂一些。主要是

```rust
// os/src/trap/mod.rs
pub fn trap_handler() -> ! {
    // ...
    match scause.cause() {
        // ...
        Trap::Interrupt(Interrupt::SupervisorTimer) => { // timer 触发
            let mut timer_map = TIMER_MAP[hart_id()].lock();
            while let Some((_, pid)) = timer_map.pop_first() { // pop 掉的即现在触发的这个 timer
                if let Some((next_time, _)) = timer_map.first_key_value() { // 下一个需要触发的 timer
                    set_timer(*next_time);
                }
                drop(timer_map);
                if pid == 0 { // os 时间片轮转任务调度
                    set_next_trigger();
                    suspend_current_and_run_next();
                } else if pid == current_task().unwrap().pid.0 { // 当前 timer 由当前任务设置并中断了当前任务
                    // 设置 sip CSR 的 UTIP 字段
                    // 表示一个 U 态的 timer 等待处理
                    // 也即 U 态的时钟中断就是从这里触发的
                    debug!("set UTIP for pid {}", pid);
                    unsafe {
                        sip::set_utimer();
                    }
                } else { // 当前 timer 由另外任务设置并中断了当前任务
                    // 往另外任务的 UserTrapQueue 中 enqueue 一条 UserTrapRecord
                    let _ = push_trap_record(
                        pid,
                        UserTrapRecord {
                            cause: 4,
                            message: get_time_us(),
                        },
                    );
                }
                break;
            }
        }
        // ...
    }
    // ...
}
```

从 os 中的 `trap_handler` 逻辑可以明白，user timer 只会中断设置它的任务，也即**自己设置时钟中断并在未来中断自己**。并且，os 通过 `sip::set_utimer()` 触发一个 U 态的时钟中断，跳转到 `utvec` CSR 指定的地址去执行。[接下来的执行流分析](./usertrap.md#用户程序库user-trap-handler)。

假设设置这个 utimer 的任务是 A（即上述代码中的 pid 代表的任务），那么如果正好在 A 执行到的时候 A 曾经设置的 utimer 触发了则可以直接跳转到 utvec（即上面判断 pid 的第二个 if-else 分支）；而假如当 A 曾经设置的 utimer 触发时，此时正在执行另外一个任务 B，则 os 会将代表这个 utimer 的 trap_record 通过  `push_trap_record` 放进任务 A 的任务控制块 TCB 中的 `UserTrapInfo` 中的 `UserTrapQueue`。