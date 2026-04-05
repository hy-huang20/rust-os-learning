# 关于 rCore-N 的 rustsbi-qemu.bin

## 1. 问题概述

rCore-N 的 `rust_main` 实现和 rCore 的区别首先在于前者多了一个 `hart_id: usize` 的参数。

在 rCore 中，加电执行后的执行过程，rCore-2025 实验指导书中是这么描述的：

>当我们执行包含上述启动参数的 qemu-system-riscv64 软件，就意味给这台虚拟的 RISC-V64 计算机加电了。 此时，CPU 的其它通用寄存器清零，而 PC 会指向 `0x1000` 的位置，这里有固化在硬件中的一小段引导代码， 它会很快跳转到 `0x80000000` 的 RustSBI 处。 RustSBI完成硬件初始化后，会跳转到 `$(KERNEL_BIN)` 所在内存位置 `0x80200000` 处， 执行操作系统的第一条指令。

`os/src/linker-qemu.ld` 文件开头规定了项目的入口：`ENTRY(_start)`，即 `entry.asm` 中的 `_start`。rCore 中这个函数的内容很简单：

```asm
_start:
    la sp, boot_stack_top
    call rust_main
```

可以看到这一步并没有为 `rust_main` 准备参数，没有看到对类似 `a0/a1` 之类寄存器的赋值。rCore 的 `rust_main()` 函数实现也是没有参数的。

rCore-N 这边的实现就不一样了。首先 `_start` 的实现就不一样：

```asm
    .section .text.entry
    .globl _start
_start:
    # a0: hart id
    mv tp, a0
    la sp, boot_stack
    # li t1, 4096 * 16 # t1 = 4096 * 16 64KB
    addi t0, a0, 1  # t0 = a0 + 1 hartid+1
    slli t0, t0, 16 # 64K * (hartid + 1)
    add sp, sp, t0  # sp = sp + t0
    call rust_main

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16 * 4
    .globl boot_stack_top
boot_stack_top:
```

关于上述代码：

- 在执行到这个 `_start` 之前 `a0` 就已经被赋值为 hart_id
    - 关于 hart 硬件线程的[简单介绍](https://github.com/hy-huang20/cpp-code-notes/blob/main/concurrency/ljhxyyjxc%E9%80%BB%E8%BE%91%E6%A0%B8%E5%BF%83%E4%B8%8E%E7%A1%AC%E4%BB%B6%E7%BA%BF%E7%A8%8B.md)
    - 这里的 `a0` 也即 rCore-N 的 `rust_main` 的 `hart_id: usize` 参数
- `slli` 指令即逻辑左移立即数
- rCore 中只有一个启动栈，而 rCore-N 为每一个 hart 分配了独立互不重叠的启动栈
    - 这里支持最多为 4 个 hart 分配启动栈 
- `tp` 寄存器 Thread Pointer 即线程指针寄存器
    - 这里用来存放当前 hart_id
    - 后续会读取该寄存器，但在任何地方都不会再修改它的值了，保存上下文的时候也会跳过这个寄存器
        ```rust
        pub fn hart_id() -> usize {
            let hart_id: usize;
            unsafe {
                asm!("mv {}, tp", out(reg) hart_id);
            }
            hart_id
        }
        ```

所以 `a0` 是被谁设置为 hart_id 的值的呢？

## 2. rustsbi-qemu

在内核的 `_start` 之前便是 `rustsbi-qemu.bin` 在执行。但是这个二进制文件不方便读取，因此可以去查看 rustsbi-qemu 项目的[源代码](https://github.com/rustsbi/rustsbi-qemu/tree/main/rustsbi-qemu)。

根据 rustsbi-qemu/build.rs 中内容（build.rs 会在运行时生成链接文件 `.ld`），这个项目的入口也会是位于其中的 `_start`，在 rustsbi-qemu/src/main.rs 中可以找到该函数：

```rust
/// 入口。
///
/// # Safety
///
/// 裸函数。
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    asm!(
        "   call {locate_stack}
            call {rust_main}
            j    {trap}
        ",
        locate_stack = sym trap_stack::locate,
        rust_main    = sym rust_main,
        trap         = sym trap_vec,
        options(noreturn),
    )
}
```

这是 rustsbi-qemu 会被执行到的第一个函数。关于 `_start`：

- 调用 `trap_stack::locate`
- 调用同文件下的 `rust_main`
- 跳转到 `trap_vec` 函数（这里相当于触发了一次 M 态 trap）

关于同文件下的 `rust_main` 函数。前面代码有些多这里省略了，注意函数中的最后一句：

```rust
/// rust 入口。
extern "C" fn rust_main(hartid: usize, opaque: usize) {
    // ...
    // 准备启动调度
    unsafe {
        // ...
        mtvec::write(trap_vec as _, mtvec::TrapMode::Vectored);
    }
}
```

将 `mtvec.BASE` 设置为 `trap_vec` 函数的地址，将 `mtvec.MODE` 设置为 `Vectored` 模式。这意味着 `trap_vec` 是一个**中断向量表**，位于 rustsbi-qemu/src/trap_vec.rs 中：

```rust
/// 中断向量表
///
/// # Safety
///
/// 裸函数。
#[naked]
pub(crate) unsafe extern "C" fn trap_vec() {
    asm!(
        ".align 2",
        ".option push",
        ".option norvc",
        "j {default}", // exception
        "j {default}", // supervisor software
        "j {default}", // reserved
        "j {msoft} ",  // machine    software
        "j {default}", // reserved
        "j {default}", // supervisor timer
        "j {default}", // reserved
        "j {mtimer}",  // machine    timer
        "j {default}", // reserved
        "j {default}", // supervisor external
        "j {default}", // reserved
        "j {default}", // machine    external
        ".option pop",
        default = sym trap_entry,
        mtimer  = sym mtimer,
        msoft   = sym msoft,
        options(noreturn)
    )
}
```

这个中断向量表中的每个表项就是其中的一条 `j 指令`，trap 到来时硬件会根据 `mcause` 计算应该跳转到其中的哪条 `j 指令`执行。

这里的 default 情况跳转到 `trap_entry`，最后会执行到 rustsbi-qemu/src/main.rs 中的 `fast_handler()` 函数。具体怎么过去的这里不作讨论，如需了解可以去看 fast-trap 库的[源代码](https://github.com/rustsbi/fast-trap)。

可以把这里的 `fast_handler()` 理解为 M 态的 `trap_handler`，类比 rCore 内核 S 态的 `trap_handler`。以下省略了一些无关代码，用注释代替：

```rust
extern "C" fn fast_handler(
    mut ctx: FastContext,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) -> FastResult {
    use riscv::register::{
        mcause::{self, Exception as E, Trap as T},
        mtval, satp, sstatus,
    };

    #[inline]
    fn boot(mut ctx: FastContext, start_addr: usize, opaque: usize) -> FastResult {
        unsafe {
            sstatus::clear_sie();
            satp::write(0);
        }
        ctx.regs().a[0] = hart_id();
        ctx.regs().a[1] = opaque;
        ctx.regs().pc = start_addr;
        ctx.call(2)
    }
    loop {
        match local_hsm().start() {
            Ok(supervisor) => {
                // 当前 hart 被安排去启动一个 S 态 supervisor 入口了
                mstatus::update(|bits| {
                    *bits &= !mstatus::MPP;
                    *bits |= mstatus::MPIE | mstatus::MPP_SUPERVISOR;
                });
                mie::write(mie::MSIE | mie::MTIE);
                break boot(ctx, supervisor.start_addr, supervisor.opaque);
            }
            Err(rustsbi::spec::hsm::HART_STOP) => {
                // ...
            }
            _ => match mcause::read().cause() {
                // SBI call
                T::Exception(E::SupervisorEnvCall) => {
                    // S 态内核执行了一次 ecall，也就是在调用 SBI
                }
                // 其他陷入
                trap => {
                    println!(
                        "
-----------------------------
> trap:    {trap:?}
> mstatus: {:#018x}
> mepc:    {:#018x}
> mtval:   {:#018x}
-----------------------------
            ",
                        mstatus::read(),
                        mepc::read(),
                        mtval::read()
                    );
                    panic!("stopped with unsupported trap")
                }
            },
        }
    }
}
```

目前的情况对应 `Ok(supervisor)` 逻辑，调用 `boot()`，之前提到的 `a0` 设置为 `hart_id` 就是在这里完成的！然后 `boot()` 中设置 `pc` 为 `supervisor.start_addr`，这个地址值就是 `0x8020_0000`！`supervisor.start_addr` 的值是在同文件下的 `rust_main` 中被设置的，里面有这么一段代码：

```rust
mod constants {
    /// 特权软件入口。
    pub(crate) const SUPERVISOR_ENTRY: usize = 0x8020_0000;
    // ...
}

/// rust 入口。
extern "C" fn rust_main(hartid: usize, opaque: usize) {
    static GENESIS: AtomicBool = AtomicBool::new(true);
    static BOARD_INFO: Once<BoardInfo> = Once::new();

    // 全局初始化过程
    if GENESIS.swap(false, Ordering::AcqRel) {
        // ...
        // 设置内核入口
        local_remote_hsm().start(Supervisor {
            start_addr: SUPERVISOR_ENTRY,
            opaque,
        });
    } else {
        // ...
    }
    // ...
    // 准备启动调度
    unsafe {
        // ...
        mtvec::write(trap_vec as _, mtvec::TrapMode::Vectored);
    }
}
```