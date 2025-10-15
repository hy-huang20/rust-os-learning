# rust 异步驱动设计与实现

总体的计划安排 [实时更新中](#20250611)

## 20251015

### 进度

- 在本地成功运行 rCore-N
- 在 embassy 引入 rCore-Tutorial-v3/ch9 时遇到 bug
    - 使用 docker 运行 rCore：make build_docker
        - 网络问题：已解决
        - input/output error

### 下周

- 解决环境问题，embassy into rcore

## 20250924

### 进度

- 进行中：研究怎么抽离 rCore-N 中的 BufferedSerial 和 AsyncSerial 并使用独立的 [async-uart-driver](https://github.com/BITcyman/async-uart-driver/tree/main) crate 并跑通

### 下周

- 了解 lc 文档中 [embedded_hal_nb](https://github.com/BITcyman/Rust-os-learning/blob/main/driver/uart-crate.md#embedded_hal_nb) 的作用
- 研究怎么将 embassy 引入 rCore-N 并实现
    - 找到了一篇记录：[参考](https://github.com/lighkLife/rcore-blog/issues/1)

## 20250917

### 进度

- Embassy 总结
    - 根据赵方亮的 Embassy 学习记录形成的[初稿](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/readme.md)
- 理清林晨的实现思路
    - 划分为 5 个[步骤](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/%E5%A4%8D%E7%8E%B0%E8%BF%87%E7%A8%8B/rCore-N%5BQEMU%5D/readme.md)
    - 复现了第 1 步，第 2 步的 async-uart-driver crate 已有林晨的现成代码

### 下周

- 继续之前的工作
    - rCore-N 中的 timer 用 embassy timer 代替，能够在 qemu 工作
    - 对应林晨实现思路中的第 4 步
    - 任务
        - 把林晨的 async-uart-driver 在 rCore-N 中跑起来
        - 研究用 embassy 改进 async-uart-driver 并跑起来

## 20250903

### 进度

- Embassy 总结
    - [Embassy 代码注释与执行流程](https://github.com/hy-huang20/rust-os-learning/tree/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy)，已完成
    - 参考赵方亮的 Embassy 学习[笔记](https://github.com/zflcs/learning/blob/main/notes/embassy%E5%AD%A6%E4%B9%A0%E7%AC%94%E8%AE%B0.md)，后续以此为大纲进行自己的补充
- rCore-N 中的 timer 用 embassy timer 代替，能够在 qemu 工作
    - 进行中，[实验过程](https://github.com/hy-huang20/rust-os-learning/tree/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/%E5%A4%8D%E7%8E%B0%E8%BF%87%E7%A8%8B/rCore-N%5BQEMU%5D/code)。思路是根据林晨的 uart-crate [记录](https://github.com/BITcyman/Rust-os-learning/blob/main/driver/uart-crate.md) 进行复现

### 下周

- Embassy 总结一份记录
    - 以赵方亮学习笔记为大纲，对其中细节进行注释
- rCore-N + Embassy
    - 移除 rCore-N 中的异步串口逻辑，改用独立 crate 代替并跑通
    - 使用 Embassy 运行时改进独立 crate 逻辑并跑通

## 20250827

### 进度

- 看 [Risc-V Extension N Implementation](https://gallium70.github.io/rv-n-ext-impl/intro.html)，尚未完成

### 下周

- embassy timer 怎么工作
    - 整理文档
    - 代码对应关系
    - 程序验证
- rCore-N 中的 timer 用 embassy timer 代替，能够在 qemu 工作

## 20250820

### 进度

- 熟悉一些基本概念 
    - 参考 lc [记录](https://github.com/BITcyman/Rust-os-learning/blob/main/driver/uart-crate.md#%E4%B8%80%E4%BA%9B%E5%9F%BA%E6%9C%AC%E6%A6%82%E5%BF%B5)
    - 串口，uart，svd，pac
- 学习用户态中断 [我的记录](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rCore-N/user-interrupt.md)
    - 看 [Risc-V Extension N Implementation](https://gallium70.github.io/rv-n-ext-impl/intro.html)

### 下周

- 看完 Risc-V Extension N Implementation
    - 整理报告
- 继续学习 rCore-N
- 跑 lc 的 qemu

## 20250811-20250813

### 进度

- 细化[秋季学期计划](#秋季学期计划)的时间安排
- 参考[日志撰写要求](https://github.com/orgs/arceos-hypervisor/discussions/197)写开发日志
    - 短期计划：Github Issues
    - 阶段任务：Github Milestones
    - 长期计划：Github Discussions/Announcements
    - 学习记录：Github Discussions/General
    - 正式文档：markdown 文件直接上传到相关仓库
    - 代码文件：直接上传到相关仓库

### 秋季学期计划

未来 3 个月时间内：

- 9 月初返校之前（4 周）
    - 至少在 qemu 中完全复现林晨的工作（3 周）
        - 按照林晨的毕设日志推进
    - （前项完成后）块设备异步
        - 调研，学习相关知识
- 9 月（、10 月）完成异步串口上板
- 10 月完成异步块设备驱动实现
- 11 月初完成毕设主要工作
    - 至少将林晨的异步串口工作推进至完成
    - 去年毕设意向问卷截至 11 月 11 日
- 提高自信

### 后续

- 看 rCore-N
    - 带着问题去看，完整看完工作量不小
- 跑林晨的 qemu

## 20250725

### 进度

- 属性宏的转化 [记录](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/proc-macro.md)
    - ``#[task]``, ``#[main]``
- time driver [源码分析](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/time-driver.md)
    - ``driver_std``
- embassy-executor/src/arch/std.rs [源码分析](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/executor/arch/std.md)
    - ``Pender`` 类型；作用
- 验证正确性：cargo test
    - 测试代码
        - [embassy-learning](https://github.com/hy-huang20/rust-learning/tree/embassy-learning/embassy-learning/tests)
        - [attribute-macros](https://github.com/hy-huang20/rust-learning/tree/attribute-macros/attribute-macros/tests)

### 后续

- 整理一下已有的 Embassy 记录
- 看 rCore-N
    - 跑林晨的 qemu

### 2025S 学期总结

[学期总结](./过程记录/总结/2025S.md)

- 已完成
    - Rust 入门
    - 写 rCore
    - Write an OS in Rust 文档学习
    - Embassy 源码学习
- 未完成
    - 毕设主体工作仍未完成

## 20250716

### 进度

embassy 学习[过程记录](https://github.com/hy-huang20/rust-os-learning/tree/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy)

已完成部分：（embassy-executor）

- src/raw/mod.rs [源码分析](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/executor/raw/mod.md)（根据其中的注释）
    - task 生命周期 4 个状态（对应 src/raw/state_atomics_arm.rs 中 ``State`` 不同属性值），6 个状态转移过程
- src/raw/run_queue_atomics.rs：从 RunQueue 开始的[源码分析](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/executor/raw/run_queue_atomics.md)
    - 关于函数指针 ``poll_fn``：什么时候被赋值，什么时候被调用，调用的时候是什么样的值
- time queue 和 run queue：[源码分析](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/queue.md)
    - 各自的功能
    - 两者之间的关系
- 对一个简单[例子](https://github.com/hy-huang20/rust-learning/blob/embassy-learning/embassy-learning/src/main.rs)分析了较为完整的调用过程
    - sequence diagram 见[记录](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/executor/readme.md)
- src/raw/utils.rs [源码分析](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/executor/raw/utils.md)
    - 这个不是很重要，主要是基于 core 提供的底层类型实现了简单的包装类型

### 后续

- **改进意见**：需要验证过程记录内容的正确性
    - 关于测试：写一些简单的使用例，然后进行断点调试，追踪函数调用栈？（方案待确认）
        - 尝试：在 vscode 中进行断点调试
- 目前还没有把调用逻辑整个地串起来
    - 完成 embassy [过程记录]((https://github.com/hy-huang20/rust-os-learning/tree/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy))
        - ``#[task]``, ``#[main]`` 在编译时是如何被转化的？转化成什么样的代码？
            - 和 embassy-executor-macros package 的关系?
        - "task wakes itself"（见 src/raw/mod.rs）的原理
            - 在 embassy-executor 中没有地方调用 src/raw/waker.rs 中的 ``wake()`` 函数。调用方?
        - 完善 [timer.md](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/timer.md) 
            - ``Timer::poll()`` 中的执行逻辑不清
        - 完善[从 RunQueue 开始的源码分析](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/executor/raw/run_queue_atomics.md)
            ``SyncExecutor::poll`` 的调用方？ 
        - time driver [源码分析](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/time-driver.md)
            - TODO: std, wasm
            - 在 PC 上运行的例子使用了 mock, std, wasm 中的哪个 time driver？
        - ``Pender`` 类型
            - ``Pender`` 的作用
            - 关于 ``unsafe extern "Rust" fn __pender()``
        - ``Token`` 类型
- 继续跑一些 Embassy Book 中的例子
    - 之前已经跑通 Embassy Book 中可以在 PC 上运行的例子
    - TODO：有些例子可能需要在 QEMU 中运行
    - 有些内容似乎和目前关系不大

## 20250709

### 进度

- 进行中：看 embassy 源码 [过程记录](https://github.com/hy-huang20/rust-os-learning/tree/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy)
    - 目前涉及到的 package:
        - embassy-time
        - embassy-time-driver
        - embassy_time_queue_utils
        - embassy-executor
        - embassy-executor-macros

### 后续

- 把 embassy-executor 执行过程理清楚，并完成源码阅读过程记录
    - embassy-time
    - embassy-executor
- 参考一下训练营中的 embassy 记录

## 20250701

### 进度

- 看 [Embassy Book](https://embassy.dev/book/)，[过程记录](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/readme.md)
    - Introduction 节
    - System Description 节
        - Embassy Executor 节
        - Time-keeping 节
- 跑通了 Embassy Book 中的 [2 个 example](https://github.com/hy-huang20/rust-learning/commit/955c5bb61689788dcc2a45fe7d03b5bd940f7ea7)
- 问题：林晨纪要 0416 中几个链接似乎失效了
    - [baremental 仓库](https://github.com/zflcs/baremental/)失效
    - [仍然有效的链接](https://github.com/ATS-INTC/ats-intc/blob/30e4ea6a47de5ca8ec757764d544f3c8e58d9e14/src/waker.rs?accessToken=eyJhbGciOiJIUzI1NiIsImtpZCI6ImRlZmF1bHQiLCJ0eXAiOiJKV1QifQ.eyJleHAiOjE3NTEzMzEwNDgsImZpbGVHVUlEIjoiWEtxNDI1eGIxbnQ0V3pBTiIsImlhdCI6MTc1MTMzMDc0OCwiaXNzIjoidXBsb2FkZXJfYWNjZXNzX3Jlc291cmNlIiwicGFhIjoiYWxsOmFsbDoiLCJ1c2VySWQiOjMwNTEwODA2fQ.lwgSV9-tymBeP2w-owXr-mwaahMgADLQSUjYye6lQqA)
    - 解决：可以看[其它例子](https://github.com/ATS-INTC/ats-intc/blob/30e4ea6a47de5ca8ec757764d544f3c8e58d9e14/src/lib.rs)

### 后续

- 看 Timer 源码
- 异步串口例子（还可以参考[训练营的代码：2，5](https://shimo.im/docs/KlkKvREgExUZM2qd)）在 qemu 中跑起来
- 继续看 Embassy Book
- 学习[在 rCore 中引入异步运行时](https://github.com/lighkLife/new-blog/issues/1?accessToken=eyJhbGciOiJIUzI1NiIsImtpZCI6ImRlZmF1bHQiLCJ0eXAiOiJKV1QifQ.eyJleHAiOjE3NTEzMzEwNDgsImZpbGVHVUlEIjoiWEtxNDI1eGIxbnQ0V3pBTiIsImlhdCI6MTc1MTMzMDc0OCwiaXNzIjoidXBsb2FkZXJfYWNjZXNzX3Jlc291cmNlIiwicGFhIjoiYWxsOmFsbDoiLCJ1c2VySWQiOjMwNTEwODA2fQ.lwgSV9-tymBeP2w-owXr-mwaahMgADLQSUjYye6lQqA)

## 20250625

### 进度

- 完成 Write an OS in Rust/async-await 的阅读 [总结](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/async-await/readme.md)
- 代码
    - 参照上述文档实现了一个[不带 Waker 的 Executor](https://github.com/hy-huang20/rust-learning/commit/9a56e8c0d5e5e0022983daca3dd3390a859076a4)
    - 基于一个[已有的异步爬虫](https://gitee.com/taoqi-cat/asyn/tree/master/spider)进行了简单改进

### 后续

- 学习 [embassy](https://github.com/embassy-rs/embassy/tree/main/embassy-executor)

## 20250618

### 进度

- 可选：[rCore 文档](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rCore/readme.md)
    - ch4, ch5 添加详细内容
- 学习 Write an OS in Rust [async-await](https://os.phil-opp.com/async-await/)
    - [文档总结](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/async-await/readme.md) 进行中

### 后续

- 可选：rCore 文档
    - ch6 比较简略
- 学完 async-await 并总结文档
- 用 tokio 库写一个异步的爬虫
    - 训练营中已有现成代码，可问老师要
- 有时间的话开始看 [embassy](https://github.com/embassy-rs/embassy/tree/main/embassy-executor)
    - timer：内核里写异步

## 20250611

### 进度

- 20250610 已和老师约时间演示实验完成情况
- 将写 rCore 的过程进行总结，已经形成[初版文档](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rCore/readme.md)

### 后续

- 毕设后续计划，明确任务/步骤以及前后任务之间的依赖关系
    - 复现：QEMU 串口驱动
        - 学习 rust 异步
            - Write an OS in Rust
                - [async-await](https://os.phil-opp.com/async-await/)  
            - [Embassy](https://github.com/embassy-rs/embassy/tree/main/embassy-executor)
                - 具体的细节需要看林晨的文档中的 20240416 的纪要中的描述
                - [embassy 记录](https://github.com/BITcyman/Rust-os-learning/blob/main/embassy/embassy.md)
                - [embassy into rCore 记录](https://github.com/BITcyman/Rust-os-learning/blob/main/embassy/embassy-into-rcore.md)
        - rCore-N 异步串口驱动 
            - 已经可以跑通 [记录](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/%E5%A4%8D%E7%8E%B0%E8%BF%87%E7%A8%8B/rCore-N%5BQEMU%5D/readme.md)
            - 原理学习 [参考](https://github.com/BITcyman/Rust-os-learning/blob/main/rCore-N.md)
        - 学习已有的[异步串口驱动 crate](https://github.com/BITcyman/async-uart-driver/commit/3d1265d17e6b2d6e1ce8df351f6e6d19d04136ce)
        - QEMU 上跑通异步串口驱动在 AlienOS 中的使用
    - 实现：串口驱动上板子
        - 学习星光二板子上已有的 [PAC](https://codeberg.org/weathered-steel/jh71xx-pac) 和 [HAL](https://codeberg.org/weathered-steel/jh71xx-hal)
            - [参考](https://github.com/BITcyman/Rust-os-learning/blob/main/Vision_Five2.md)
        - 已有的工作
            - QEMU 上，异步串口驱动在 AlienOS 中使用 [最新记录](https://github.com/BITcyman/Rust-os-learning/blob/main/driver/runtime.md)
            - **已实现** QEMU 中跑 AlienOS 异步串口驱动，**已实现** AlienOS 上板，**未实现** AlienOS 异步串口驱动上板
                - 以下内容摘抄自[林晨论文](https://github.com/BITcyman/Rust-os-learning/blob/main/report/10007_%E8%B7%A8%E6%93%8D%E4%BD%9C%E7%B3%BB%E7%BB%9F%E7%9A%84%E5%BC%82%E6%AD%A5%E4%B8%B2%E5%8F%A3%E9%A9%B1%E5%8A%A8%E6%A8%A1%E5%9D%97%E8%AE%BE%E8%AE%A1%E4%B8%8E%E5%AE%9E%E7%8E%B0.pdf)第 6 章总结和展望：
                    >没有在星光二实体板上进行适配。虽然当前 Alien 操作系统已经能够在板子上成功运行，但是实际的开发板和虚拟机情况存在一些不同：在 QEMU 模拟的虚拟 qemu-system-riscv64 中，我们不仅可以将几个虚拟串口的输入输出重定向到几个终端上，这样就可以对串口输出进行比较简单的可视化，并且使用该终端向串口中输入数据，并且由于具有多个串口，我们可以在某一个串口上使用原本 Alien 中的串口驱动实现，在另一个串口上使用我们实现的异步串口驱动；而在星光二 Vision Five2 开发板上启动 Alien 时使用的与 Ubuntu 进行通信的串口为默认串口，并且开发板上默认只开启一个串口。如果想要开启多串口，就需要修改硬件配置，同时使用更多的串口线连接开发板。
    - 实现：块设备驱动
        - QEMU，开发板
        - 了解不足，还需学习调研
            - 从老师那里了解到已经有能跑的实现了，只需要将已有的改成异步的
    - 最终目标
        - 星光二开发板上，串口/块设备异步驱动，在 AlienOS 中使用

## 20250604

### 进度

完成了 [rCore-2025S 实验](https://github.com/hy-huang20/rCore-2025S)

- 完成了 rCore-2025S 指导书 ch3-ch8 的编程练习，可以通过测例
- 偶尔会出现 rustup toolchain 的报错：st_name (xxx) is past the end of string table of size xxxx
- 上述报错和代码实现无关

### 后续

- 复盘，将写 rCore 的过程总结成文档
- 约向老师检查 rCore
- Rust 异步驱动设计与实现
    - 复现：QEMU 串口驱动（6月）
        - 学习并复现[林晨学长的工作](https://github.com/BITcyman/Rust-os-learning)
    - 实现：开发板串口驱动
    - 实现：QEMU 块设备驱动
    - 实现：开发板块设备驱动