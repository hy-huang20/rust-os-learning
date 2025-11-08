# embassy_time::Timer

## 更新：20251105 讨论

将 rCore-N timer 改成异步是改内核中的实现，而之前看的关于 embassy 的内容都是在用户态的（比如依赖于 std 的 embassy_time 相关调用逻辑）。先进一步参考[赵方亮的 embassy 学习记录](https://github.com/zflcs/learning/blob/main/notes/embassy%E5%AD%A6%E4%B9%A0%E7%AC%94%E8%AE%B0.md)，看有没有什么思路。

## 更新：关于将 rCore-N timer 改成异步

embassy/embassy_time 包提供了三种驱动。但是将 rCore-N timer 改成异步本质上是在**内核**中写代码，因此之前学习的 driver_std 的实现无法移植，因为 driver_std 依赖于 rust 的 std 库实现，而 std 库依赖于底层主流操作系统的支持。如果需要在 rCore-N 中引入类似于 embassy 的 timer 模式，需要在 rCore-N 的基础上实现自己的 time driver，重点在于以下两步：

- 将 timer 逻辑封装并实现为 Future

    ```rust
    use core::future::Future;

    impl Future for Timer {
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            // TODO
        }
    }
    ```
- 提供一个 MyDriver，将其实现为 Driver

    ```rust
    use embassy_time_driver::Driver;

    impl MyDriver for Driver {
        fn now(&self) -> u64 {
            // TODO
        }

        fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
            // TODO
        }
    }
    ```

其它的细节参考以下的源码部分。

## 源码

我在 [embassy-learning](https://github.com/hy-huang20/rust-learning/tree/embassy-learning/embassy-learning) 中使用到了 embassy_time 包中 Timer 中的函数。这里分析一下使用的 Timer 中函数的调用过程，以 ``Timer::after_secs()`` 为例。

```rust
// src/main.rs

use embassy_executor::{Spawner, task, main};
use embassy_time::{Timer, Duration};

#[task]
async fn run() {
    loop {
        info!("tick");
        Timer::after_secs(1).await;
    }
}
```

首先查看一下 ``Timer::after_secs()`` 的实现：

```rust
// embassy/embassy-time/src/timer.rs

pub struct Timer {
    expires_at: Instant,
    yielded_once: bool,
}

impl Timer {
    pub fn after(duration: Duration) -> Self {
        Self {
            expires_at: Instant::now() + duration,
            yielded_once: false,
        }
    }

    #[inline]
    pub fn after_secs(secs: u64) -> Self {
        Self::after(Duration::from_secs(secs))
    }
}
```

所以 ``Timer::after_secs()`` 返回了一个 ``Timer`` 类型。由于能够对该函数返回值调用 ``.await``，可见该类型实现了 ``Future`` trait:

```rust
// embassy/embassy-time/src/timer.rs

impl Future for Timer {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded_once && self.expires_at <= Instant::now() {
            Poll::Ready(())
        } else {
            embassy_time_driver::schedule_wake(self.expires_at.as_ticks(), cx.waker());
            self.yielded_once = true;
            Poll::Pending
        }
    }
}
```

以上的 ``poll()`` 方法只有在显式调用或者在 future 上 ``.await`` 时才会被执行：关于 [await](https://os.phil-opp.com/zh-TW/async-await/#the-async-await-pattern)。第一次调用 ``poll()`` 由于 ``yielded_once`` 初值为 false，所以一定执行 else 分支，并调用了 ``embassy_time_driver`` 包中的 ``schedule_wake()``：

```rust
// embassy/embassy-time-driver/src/lib.rs

extern "Rust" {
    fn _embassy_time_schedule_wake(at: u64, waker: &Waker);
}

/// Schedule the given waker to be woken at `at`.
#[inline]
pub fn schedule_wake(at: u64, waker: &Waker) {
    unsafe { _embassy_time_schedule_wake(at, waker) }
}

#[macro_export]
macro_rules! time_driver_impl {
    (static $name:ident: $t: ty = $val:expr) => {
        static $name: $t = $val;

        #[no_mangle]
        #[inline]
        fn _embassy_time_schedule_wake(at: u64, waker: &core::task::Waker) {
            <$t as $crate::Driver>::schedule_wake(&$name, at, waker);
        }
    };
}
```

关于如何使用这个宏 ``time_driver_impl`` 的详细信息，可以查看同文件中的**注释**。简单来说，需要在外部模块实现类似于以下的内容：

```rust
use core::task::Waker;

use embassy_time_driver::Driver;

struct MyDriver{} // not public!

impl Driver for MyDriver {
    // ...

    fn schedule_wake(&self, at: u64, waker: &Waker) { // 会调用这个函数
        todo!()
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: MyDriver = MyDriver{});
```

之后，``_embassy_time_schedule_wake()`` 中实际就会调用 ``MyDriver`` 中的 ``schedule_wake()``。

在 ``embassy_time`` 包中提供了三种这样的 driver，它们位于 ``src/driver_*.rs`` 文件中；而具体启用哪种，在 ``src/lib.rs`` 中被决定，可以看到这样的条件编译代码：

```rust
// embassy/embassy-time/src/lib.rs

#[cfg(feature = "mock-driver")]
mod driver_mock;

#[cfg(feature = "mock-driver")]
pub use driver_mock::MockDriver;

#[cfg(feature = "std")]
mod driver_std;
#[cfg(feature = "wasm")]
mod driver_wasm;
```

关于 ``driver_mock``, ``driver_std``, ``driver_wasm`` 这三种 time driver 的介绍可以参考[这里](./time-driver.md)。

例子是在 wsl 中运行的，使用的是 ``driver_std``。[验证](https://github.com/hy-huang20/rust-learning/blob/embassy-learning/embassy-learning/tests/use_driver_std.rs)