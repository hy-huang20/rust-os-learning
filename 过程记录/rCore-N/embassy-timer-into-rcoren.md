# rCore-N timer 使用 embassy timer 逻辑重写

## 1. timer 如何关联 TaskRef/Waker

### 问题描述

embassy 里面的 timer queue 类比 rCore-N 中的 TIMER_MAP：

```rust
lazy_static! {
    pub static ref TIMER_MAP: [Arc<Mutex<BTreeMap<usize, usize>>>; CPU_NUM] = Default::default();
}
```

每个 CPU 都有一个 b-tree 的 timer_map 存储 (time, pid) 键值对

但这样 TIMER_MAP 没法和 TaskRef/Waker 联系起来

### 解决方法

移植 embassy 的 timer queue。

embassy-time-queue-utils 中提供了两种 queue 的实现方式：queue_generic.rs 和 queue_integrated.rs，前者额外分配空间即 Vec::new() 构造 timer queue；而后者无需额外分配空间，timer queue 实现为链表，链表的结点存储在 TaskHeader 结构体中。

虽然 embassy-executor 使用后者，不过这里准备参考前者实现，但并不准备像前者那样使用定长的 heapless::Vec 而是使用变长的 alloc::vec::Vec 来实现。

## 2. 多 hart 内核写代码如何保证互斥

### 问题描述

在代码移植时，由于需要在 rCore-N 内核中引入 embassy timer 的相关逻辑，不是在用户态写代码，所以是用不了 Mutex 互斥锁的。embassy 的解决方法是基于 critical_section 实现了自己的 Mutex，实现在 embassy_sync 中：

```rust
use critical_section::CriticalSection;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;
```

如果是单 hart 的 riscv 确实可以类似下面这样直接用 critical_section，像 [embassy-into-rcore](https://github.com/hy-huang20/rCore-Tutorial-v3/commit/a345ccf1cda80c2c69f0f88628f6ac4a56ab2b3e) 中就是这么用的，因为 rCore 是单 hart 的：

```
[dependencies]
riscv = { version = "0.10.1", features = ["critical-section-single-hart"] }
```

在单 hart 上相当于只要关闭本核中断就能形成互斥。但是 rCore-N 有 4 个 hart，不能直接使用 critical_section，需要平台专门实现。

### 解决方法

使用自旋锁 + critical_section 的方式：

- 自旋锁：防止别的 hart 访问，别的 hart 执行时会卡在自选锁的获取上
- critical_section：riscv 的 critical_section 期间会关闭本 hart 中断，以防止本 hart 中断重入

顺序是在 critical_section 中使用自旋锁：

```rust
use spin::Mutex;

critical_section::with(|cs| {
    // 代码开头立刻获取 spin::Mutex

    // 临界区代码 ...

    // 代码结尾释放 spin::Mutex
});
```

只要保证对数据结构的访问都是形如上述代码这样，数据结构本身就可以不用再被 embassy_sync 的 Mutex 包裹了。

### 更新

上述的担心是多余的。因为 rCore-N 不同的 hart 会根据不同的 hart_id 访问不同的 BTreeMap：

```rust
lazy_static! {
    pub static ref TIMER_MAP: [Arc<Mutex<BTreeMap<usize, usize>>>; CPU_NUM] = Default::default();
}
```

多个 hart 之间实际并不存在冲突，所以这样看来，自旋锁是多余的。

而且 riscv 默认禁止中断嵌套，如果对这个数据结构的访问是在中断处理中进行的，这么看来 critical_section 也是多余的。

## 3. RefCell/Cell 是否保留

`RefCell` 是在只有 `&self` 时仍能够可变访问内部字段，通过 borrow_mut()。而 `Cell` 的不同之处在于其包裹的主要是 Copy 类型的小字段比如 u64，通过 set()。

访问 `TimerDriver` 数据结构的接口的参数列表是 `&self` 而不是 `&mut self` 的情况下，如果还想修改数据结构内部字段数据，那 `RefCell/Cell` 就得保留。

## 4. 两个不同的上下文

TimerDriver::check_alarm() -> ... -> Waker::wake() -> ... -> __pender() 和 executor.poll() 应位于两套不同的上下文中。

目前想法是前者放在时钟中断处理中，后者放在软件中断处理中。

更完整的：

```text
Timer IRQ
  -> TimerDriver::check_alarm()
  -> trigger_alarm()
  -> Waker::wake()
  -> __pender()
  -> pend software IRQ

Software IRQ
  -> executor.poll()
```

总结一下线程模式和中断模式：

- 线程模式：另一个执行流执行 check_alarm -> ... -> __pender() 唤醒当前 wfe 线程模式执行流（即 executor 所在执行流）进行 executor.poll()
- 中断模式：另一个执行流执行 check_alarm -> ... -> __pender() pend 一个中断执行流并在其上运行 executor.poll()，也即利用一个已有且未被使用的中断入口作为 executor 的运行上下文

我的理解是，embassy-executor 的线程模式还是中断模式，主要还是看 executor.poll() 跑在哪个上下文执行流中。前者跑在普通线程模式/主执行流中，后者跑在某个中断处理上下文中；前者 __pender() 负责唤醒睡着的线程模式执行流，后者负责 pend 一个专门给 executor 的中断。

**在 rCore-N 中准备参考中断模式的代码进行实现。**

## 5. embassy waker 在 rCore-N 报错

### 问题描述

embassy 的核心之一，从 Waker 获取 TaskRef 的过程在 rCore-N 中会出问题：

```rust
pub fn task_from_waker(waker: &Waker) -> TaskRef {
    if waker.vtable() as *const _ != &VTABLE as *const _ {
        panic!("Found waker not created by the executor. `Timer` only works with the executor.")
    }
    unsafe { TaskRef::clone_from_raw(waker.data() as *const TaskHeader) }
}
```

由于 rCore-N Rust 版本较老，rCore-N 中无法使用 Waker::vtable() 和 Waker::data() 方法。

配好的 rCore-N 环境 Rust 版本最好不要动，否则会牵一发动全身。

因此需要修改 task_from_waker() 的设计。

### 解决方法

使用 Waker::vtable() 的代码本质上是在比较两个 Waker 是否是同一个，这应该可以使用 `w1.will_wake(w2)` 来代替。

使用 Waker::data() 的代码怎么改？

TODO: 之前在照搬林晨 async-uart-driver 的过程中没有遇到 Waker 编译错误，因为其没有使用到 task_from_waker() 这个方法，也许可以看看林晨是怎么做的

## 6. Timer().await

最核心的问题，Timer().after(...).await() 写在哪里