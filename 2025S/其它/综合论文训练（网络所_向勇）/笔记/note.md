# 笔记

## 概念

https://blog.csdn.net/wangpaiblog/article/details/117236684

### 同步、异步、阻塞、非阻塞

对于主线程上执行的一系列代码块，当其中的某个代码块需要与相关线程交互，而调用了一个函数时：

- 如果立即去执行此函数，这称为同步。

- 如果没有去执行此函数，而是将执行此函数的时机安排在未来的某个时间，然后马上继续执行刚才的代码块，这称为异步。

- 当执行此函数时，直至获得完整的资源之前，都暂停执行当前的代码块，这称为阻塞。

- 当执行此函数时，立即获得瞬时的结果，然后马上继续执行当前的代码块。如果获得的瞬时资源不是完整的资源，之后周期性发送类似的请求，直至获得完整的资源，这称为非阻塞。

可以看出，同步与异步的区别在于函数调用的时机，而阻塞与非阻塞的区别在于发起请求后是否对本线程进行暂停。

### 同步阻塞、同步非阻塞、异步阻塞、异步非阻塞

- 同步阻塞：在需要某资源时马上发起请求，并暂停本线程之后的程序，直至获得所需的资源。

- 同步非阻塞：在需要某资源时马上发起请求，且可以马上得到答复，然后继续执行之后的程序。但如果得到的不是完整的资源，之后将周期性地的请求，直至获得所需的资源。

- 异步阻塞：在需要某资源时不马上发起请求，而安排一个以后的时间再发起请求。当到了那时发出请求时，将暂停本线程之后的程序，直至获得所需的资源。在获取资源之后，使用共享信号量、异步回调等方式将结果异步反馈。

- 异步非阻塞：在需要某资源时不马上发起请求，而安排一个以后的时间再发起请求。当到了那时发出请求时，可以马上得到答复，然后继续执行之后的程序。但如果得到的不是完整的资源，之后将周期性地的请求。在最终获取到资源之后，使用共享信号量、异步回调等方式将结果异步反馈。

## 前人工作：林晨（异步驱动）

### 题目：跨操作系统的异步串口驱动模块设计与实现

### 毕设启动：建议目标

给出星光2开发板的各种外设的异步驱动，并在这些驱动之上同时支持 ArceOS 和 AlienOS

### 工作

- 学习Rust在异步和模块化方面的相关支持；
- 学习并总结 Embassy 的异步运行时；
- 设计并实现异步串口驱动模块；
- 在QEMU虚拟环境下对 Alien 完成异步串口驱动模块的适配。

### 实验结果

- 在 QEMU 环境下 Alien 使用异步串口驱动
    成功
- Alien 上板
    成功
- 异步串口驱动尝试上板
    向串口输出数据时出现问题

## 选题

- 在块设备驱动上实现异步驱动
- 在哪个块设备

## 后续

- 准备开题
    - 开题 PPT、文献翻译
    - 熟悉内容
- 学习Rust和异步串口驱动
- 首先在QEMU上复现异步驱动
- 实现异步串口驱动上板
- 选择一种块设备，实现这个块设备的异步驱动，以支持相应文件系统运行（QEMU?）
- 块设备异步驱动上机

## 外文文献

- Harris T. Special Topic: AC – Composable Asynchronous IO For Native Languages[C]//Conference on Object-Oriented Programming Systems, Languages, and Applications. ACM, 2011.DOI:10.1145/2048066.2048134.

- Zhu L , Huang L , Fu P ,et al. The upgrade to the EAST poloidal field power supply monitoring system[J]. Fusion Engineering and Design, 2021, 172(10):112757.DOI:10.1016/j.fusengdes.2021.112757.

- Kwon G , Lee W , Lee T ,et al. Development of a real-time data archive system for a KSTAR real-time network[J]. Fusion Engineering and Design, 2018, 127(feb.):202-206.DOI:10.1016/j.fusengdes.2018.01.019.

- Jan Axelson. Serial Port Complete: COM Ports, USB Virtual COM Ports, and Ports for Embedded Systems[M]. Madison: Lakeview Research, 2000.

- Anonymous. UART 16550 Documentation[EB/OL]. [2024-05-16]. https://uart16550.readthedocs.io/_/downloads/en/latest/pdf/
