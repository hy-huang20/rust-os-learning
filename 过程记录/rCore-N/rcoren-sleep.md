# user 程序执行 sleep 时发生了什么

rCore-N 提供的 user 程序使用 sleep：

```rust
use user_lib::sleep;
```

调用 sleep 的时候**并没有**走 sys_set_timer 系统调用，而是直接在 user_lib 中进行**忙等**：

```rust
// user/src/lib.rs

pub fn sleep(period_ms: usize) {
    let start = get_time();
    while get_time() < start + period_ms as isize {
        // sys_yield();
    }
}
```

不过上面的 get_time() 倒是走了 sys_get_time 系统调用用于获取时间。

之前以为/假定 rcoren 的 sleep 是这么实现的：sleep 会调用 sys_set_timer 然后放弃 cpu，等到 timer 时间到达后重新运行。

实际的 sleep 是直接在 user_lib 中忙等，这个忙等过程可以被 os 的时间片轮转打断。

可能这样的设计是为了简单性吧，感觉这里确实是可以优化的。