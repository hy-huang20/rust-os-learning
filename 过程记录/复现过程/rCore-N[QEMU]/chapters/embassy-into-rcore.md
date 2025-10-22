# embassy into rcore

## 1. 概述

首先需要能够运行 rCore-Tutorial-v3/ch9，参考我的运行[记录](./run-rCore-turorial-v3.md)。

将 embassy 引入 rCore-Tutorial-v3/ch9，参考这篇[记录](https://github.com/lighkLife/rcore-blog/issues/1)。

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
```

#### 解决方法

TODO