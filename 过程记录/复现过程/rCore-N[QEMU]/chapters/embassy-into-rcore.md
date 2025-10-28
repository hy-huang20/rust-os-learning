# embassy into rcore

## 1. 概述

首先需要能够运行 rCore-Tutorial-v3/ch9，参考我的运行[记录](./run-rCore-turorial-v3.md)。

将 embassy 引入 rCore-Tutorial-v3/ch9，参考这篇[记录](https://github.com/lighkLife/rcore-blog/issues/1)。

我的成功运行的[仓库 commit](https://github.com/hy-huang20/rCore-Tutorial-v3/commit/a345ccf1cda80c2c69f0f88628f6ac4a56ab2b3e)。

## 2. 遇到的问题

### 2.1. item does not constrain `Fut::{opaque#0}`, but has it in its signature

#### 报错信息

```
error: item does not constrain `Fut::{opaque#0}`, but has it in its signature
  --> src/main.rs:87:1
   |
87 | #[embassy_executor::task]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: consider moving the opaque type's declaration and defining uses into a separate module
note: this opaque type is in the signature
  --> src/main.rs:87:1
   |
87 | #[embassy_executor::task]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this error originates in the attribute macro `embassy_executor::task` (in Nightly builds, run with -Z macro-backtrace for more info)
```

#### 解决过程

使用 cargo new 新建一个项目，写一个最简单的使用 embassy-executor 的例子，把相关依赖原样粘贴过去（不过不借助 qemu 而直接在 wsl 中运行的话还需要将 feature 中的 arch-riscv32 改成 arch-std）。

尝试每次修改一点后运行试错。最后发现是 embassy_executor 版本的问题？把 version 从 0.3.2 改成 0.9.0 之后：

```
error[E0658]: `impl Trait` in associated types is unstable
  --> src/main.rs:11:1
   |
11 | #[embassy_executor::task]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
```

于是根据输出提示在 crate 入口加上：

```
#![feature(impl_trait_in_assoc_type)]
```

运行成功！

#### 解决方法

- 到 Cargo.toml 中将 embassy-executor 的 version 由 0.3.2 改成 0.9.0（随便试了试 0.7.0 发现也可以）

- 在 crate 入口处加上 `#![feature(impl_trait_in_assoc_type)]`

### 2.2. qemu-system-riscv64: -netdev user,id=net0,hostfwd=udp::6200-:2000,hostfwd=tcp::6201-:80: Could not set up host forwarding rule 'tcp::6201-:80'

#### 报错信息

os/ 和 user/ 都编译通过了，是 qemu 相关的报错

```
qemu-system-riscv64: -netdev user,id=net0,hostfwd=udp::6200-:2000,hostfwd=tcp::6201-:80: Could not set up host forwarding rule 'tcp::6201-:80'
```

#### 解决方法

可能是之前成功运行过 make run 但 qemu 没有正常关闭。如果是这种情况，应该可以发现你的 6201 端口仍然被 qemu-* 进程占用着。