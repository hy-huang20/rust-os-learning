# 用户态中断

这是 [Risc-V Extension N Implementation](https://gallium70.github.io/rv-n-ext-impl/ch2_2_user_trap_handle_flow.html) 的阅读记录。

以下小写 `x` 一般代表特权级，大写 `X` 一般代表中断类型。

## 1. 系统设计

### 1.1. N 扩展（用户态中断扩展）

CSR 及指令均与 M 态，S 态类似。

#### 1.1.1. N 扩展 CSR

|CSR|说明|
|---|---|
|ustatus|用户状态寄存器|
|utvec|用户陷入向量基址寄存器|
|uip, uie|用户中断寄存器|
|sedeleg, sideleg|内核态陷入委托寄存器|
|uscratch|-|
|uepc|用户异常程序计数器|
|ucause|用户陷入原因寄存器|
|utval|用户陷入值寄存器|

#### 1.1.2. N 扩展指令

|Instr|说明|
|---|---|
|uret|恢复中断前的状态|

### 1.2. 用户态中断类型

|中断类型|缩写|触发设置位|特性|
|---|---|---|---|
|软件中断|xSI|uip.USIP|无 OS 参与，U 态|
|时钟中断|xTI|uip.UTIP|不可控，S 态**委托**|
|外部中断|xEI|uip.UEIP|不可控，S 态**委托**|

#### 1.2.1. 用户态陷入委托寄存器

|寄存器|说明|
|---|---|
|xideleg|根据其中设置位将特定**中断**委托给低一特权级处理|
|xedeleg|根据其中设置位将特定**异常**委托给低一特权级处理|

### 1.3. 用户态中断异常处理流程

|-|流程|
|---|---|
|中断|设置 uip.UXIP 并确认 sideleg.X/ustatus.UIE/uie.UXIE 后处理|
|异常|确认 sedeleg.X 后处理|

|uie.UXIE|作用|
|---|---|
|USIE|用户态软件中断使能位|
|UTIE|用户态时钟中断使能位|
|UEIE|用户态外部中断使能位|

[用户态中断异常处理过程](https://gallium70.github.io/rv-n-ext-impl/ch2_2_user_trap_handle_flow.html#%E4%B8%AD%E6%96%AD%E7%9A%84%E5%A4%84%E7%90%86)和之前学过的、在 M 态，S 态处理中断异常的过程类似。

### 1.4. 外部中断与 PLIC

原文并没有详细介绍 PLIC，需要看[PLIC 官方文档](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#risc-v-plic-operation-parameters)

### 1.5. 软件中断与 UINTC

#### 1.5.1. 勘误

原文档 2.4 节中有几个错误：

- `发送方部分` 和 `接收方部分` 表格中所有地址形如 0x????9FC 的**寄存器描述**部分应是 enable 而非 pending
- 在[使能寄存器 enable](https://gallium70.github.io/rv-n-ext-impl/ch2_4_software_interrupt_and_uintc.html#%E4%BD%BF%E8%83%BD%E5%AF%84%E5%AD%98%E5%99%A8-enable) 小节中，在**考虑发送方 s 的第 i 个地址的 32 位 enable 寄存器**时，关于读取的情况，`r` 的范围应为 1 到 R-1 而非 0 到 n-1

## 2. 系统实现

### 2.1. 硬件与模拟器

#### 2.1.1. QEMU with extension N

跑 rCore-N 时用到的 QEMU 是带 N 扩展的 QEMU，多了以下特征：

- 添加寄存器 ustatus uip uie sideleg sedeleg uepc utvec ucause utval uscratch
- 添加用户态中断的触发部分：符合条件时使上述处理器进入中断状态
- 实现 uret 指令
- 修改 PLIC 以支持用户态外部中断
- 添加串口用于测试