# rCore-N

## 目录

- [跑 rCore-N 记录](./chapters/run-rCore-N.md)

- [跑 rCore-Tutorial-v3](./chapters/run-rCore-turorial-v3.md)

## 概述

林晨的[记录](https://github.com/BITcyman/Rust-os-learning/blob/main/driver/uart-crate.md)中的实验过程：

|序号|步骤|备注|结果|备注|
|---|---|---|---|---|
|1|使用 svd2rust 库将 qemu-16550.svd 文件转化成一个 PAC crate: [qemu-16550-pac](https://github.com/BITcyman/qemu-16550-pac)|<ul><li>该 crate 的作用是向外暴露 safe API 的接口访问外围设备的库</li><li>通过 PAC 中的接口操作寄存器，从而达到对串口硬件的操作</li></ul>|✅||
|2|根据 rCore-N 中的 BufferedSerial 和 AsyncSerial 相关逻辑实现一个独立的 crate: [async-uart-driver](https://github.com/BITcyman/async-uart-driver/tree/main)|<ul><li>A rust asynchronous serial driver that is not related to the operating system</li></ul>|✅||
|3|抽离 rCore-N 中的 BufferedSerial 和 AsyncSerial 使用该独立 crate 并跑通||✅||
|4|使用 Embassy 运行时改进 async-uart-driver 实现并跑通|<ul><li>该版本 [async-uart-driver](https://github.com/BITcyman/async-uart-driver) 是已经参考 Embassy 改进过的</li></ul>|✅||
|5|async-uart-driver 与 Alien OS 适配|<ul><li>验证该独立 crate 的操作系统无关性</li></ul>|✅||