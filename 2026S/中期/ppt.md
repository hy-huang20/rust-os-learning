---
marp: true
---

# 毕业设计中期报告

## Rust 异步驱动模块设计与实现

汇报人：黄昊颖
指导老师：向勇

---

# 目录

## 主要研究内容

## 实验步骤与目前进展

## 存在问题与拟解决方案

## 下一步研究任务和进度安排

---

# 主要研究内容

- Vision Five 2 星光二实体开发板
- 开发基于 Rust Future 的异步硬件驱动模块，上板运行

## 预期研究成果

- rCore-N 使用 Rust Future 异步串口逻辑上板运行
    - 使用已有 async-uart-driver 或者基于 Embassy 实现一个自己的版本

- 在星光二开发板上支持 SD 闪存异步块设备驱动模块
    - 将原有的同步逻辑使用 Rust Future 重写并形成独立 crate

（板子图片，rCore 画的几张图）

---

# 实验步骤与目前进展

## 实验步骤

- rCore-N 环境配置
- rCore-N 异步 timer 开发，将 timer 逻辑使用 Rust Future 重写，参考 Embassy
- 移除 rCore-N 中的 uart.rs 已有串口逻辑并引入林晨的 async-uart-driver 或者自己实现的 crate，在 QEMU 运行
- 学习星光二开发板已有的 PAC HAL
- rCore-N + 独立异步串口驱动 crate 上板
- 将上面的 Rust Future 逻辑移植到异步块设备驱动

---

## 目前进展：rCore-N

- rCore-N 简介
    - 基于 rCore 开发，支持用户态 U 态 CSR 指令
    - 只有进程模型，没有线程模型
    - 有自己的串口逻辑

- rCore-N 环境配置
    - 带 N 扩展 QEMU 编译
    - rCore-N 编译运行
    - 跑通 uart_benchmark 串口测例

---

## 目前进展：Embassy

- Embassy 学习：运行 Embassy 示例程序并追踪程序执行流
    - embassy-executor 状态 State 转移
    - embassy timer 设计
    - embassy 主要数据结构之间的转换：各个数据结构的设计不是孤立的，而是互相联系的
    - 基于中断的 executor

上面 4 点分 4 页 ppt 讲

---

## 目前进展：异步 timer 开发

- Waker 设计：已完成
    - 直接复用 embassy-executor 的 waker 实现
    - Waker::wake() -> wake_task() 只负责修改相应任务的状态为 TaskState::Ready
- Task 设计：已完成
    - 参考 Embassy 的 TaskRef 设计了一样的
    - 任务状态：枚举，包含 3 种状态 Ready, Running, Pending
- Executor 设计：已完成
    - 维护一个存储任务的 VecDeque，任务 future 创建时放入
    - 执行时从其中取 TaskState::Ready 的任务 poll，先修改状态为 TaskState::Pending，返回 Poll::Pending 则放回，返回 Poll::Ready 则踢出
- Future 设计：进行中
    - 实现一个 Timer Future，将 timer 逻辑放到 poll 中


目前仍然在开发中

---

# 存在问题与拟解决方案

- 为什么不直接使用 Embassy
    - 已有的将 embassy 引入 rcore 的记录是基于线程模式 thread mode 的 executor
        - embassy-executor = { version = "0.3.2", features = ["arch-riscv32", "nightly", "executor-thread"] }（配图）
    - 记录中的步骤使用的修改版 riscv 依赖和 rCore-N 的修改版 riscv 依赖产生冲突
        - embassy-into-rcore 记录
        ```toml
        riscv = { version = "0.10.1", features = ["critical-section-single-hart"] }
        # 如果已经依赖了 rcore 的 riscv 
        # 可以 将其替换为下面的依赖, 同时修改 trap/context 中 riscv 的路径为 riscv_asm
        riscv_asm = { git = "https://github.com/lighklife/riscv-asm", features = ["inline-asm"] }
        ```
        - rCore-N 使用的依赖
        ```toml
        riscv = { git = "https://github.com/duskmoon314/riscv", branch = "extN", features = ["inline-asm",] }
        ```    
    - 基于中断的 executor：`#[interrupt]` 宏底层中断基于 Vectored 模式，rCore-N 底层中断基于 Direct 模式，无法直接使用。

---

# 存在问题与拟解决方案

- 可以借鉴 Embassy 的可取之处自行设计逻辑
    - Embassy 的意义以及借鉴之处主要在于以下两点
        - embassy 基于中断的 executor
            - 状态转移和唤醒过程
            ：wake_task() 调用 SyncExecutor::enqueue() 调用 __pender 函数触发软件中断（如 cortex_m armv6m 上通过 NVIC::pend(irq) 接口）执行 InterruptExecutor::on_interrupt() 进行 poll
        - embassy 主要数据结构之间的转换
            - Waker 和 TaskRef 可以相互转换
            - Waker 标识任务：通过 Waker 获取 TaskRef 访问 TaskHeader 任务元数据以及存储在 TaskHeader 中的 timer queue 和 RunQueue 链表节点

---

# 存在问题与拟解决方案

- rCore-N 异步 timer 开发中，拟解决方案
    - embassy 官方没有为 risc-v 支持基于中断的 executor，但可以自行实现（如下）
    - 在时钟中断中只做简单的事进行 wake 修改任务状态，然后 sip::set_ssoft() 触发软件中断（这不会造成中断嵌套，会在试图回到用户态时检查是否有软件中断并执行软件中断逻辑，因此不会拖累时钟中断）（embassy 官方也是推荐这么做的，配图）
    - rCore-N 中目前没有地方调用 sip::set_ssoft()，因此 trap_handler 软件中断没有被别的地方使用，适合将 executor poll 执行逻辑放到这里

- 如果上面的任务逻辑在简单场景中被验证为正确有效，后续可以更容易地移植到串口和块设备逻辑开发中

---

# 下一步研究任务和进度安排

## 下一步研究任务

- rCore-N 异步 timer 完成开发
- rCore-N 异步串口 crate QEMU/上板
- 基于之前的设计，开始异步块设备驱动实现

## 进度安排

按照开题那样的表格，不过起始时间改成从今天开始后的

---

# 结尾

谢谢

敬请各位老师批评指正