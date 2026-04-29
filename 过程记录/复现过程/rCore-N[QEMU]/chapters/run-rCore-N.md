# 跑 rCore-N

## 更新-20260429

这次是在 virtualbox 上从零开始配置的。我把之前成功构建 rcoren 的 Cargo.lock 也上传到 github 仓库了。这次的 [commit](https://github.com/hy-huang20/rCore-N/commit/bb36643a8c483b359d487b4c5b593401a9abb0ef)。

```
# 下载 Rust
# 不要通过 apt, snap 非官方路径安装 cargo, rustup！
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

git clone https://github.com/hy-huang20/rCore-N.git
git clone https://github.com/hy-huang20/qemu.git
mkdir qemu-build
cd qemu-build
../qemu/configure --target-list="riscv64-softmmu" --extra-cflags=-Wno-error
make -j

cd ..
cd rCore-N/
# --locked 表示强制按照 just 1.14.0 发布时 just crate 自带的 Cargo.lock
# 保证构建环境可以复现
cargo install just --version 1.14.0 --locked
cd os
rustup target add riscv64imac-unknown-none-elf
# 如果 LOG=DEBUG just run 提示找不到 rust-objcopy 则需要安装下面的工具
cargo install cargo-binutils --version 0.3.6 --locked
rustup component add llvm-tools-preview

# 运行之前开三个窗口
# 输入 tty 验证分别为 /dev/pts/0, /dev/pts/1, /dev/pts/2
# 在 /dev/pts/0 窗口下输入如下指令启动内核
LOG=DEBUG just run
# 在 /dev/pts/1 窗口中与内核交互
# >> Rust user shell
# >>
# 当然上述的 0, 1, 2 不是固定的
# 可以看自己开的三个窗口的 tty
# 然后去 qemu 配置中改对应的三处 -serial
# 具体修改 justfile 的 SERIAL_FLAGS 的前三项
```

## 更新-20251015

我的 rCore-N 成功运行[对应 commit](https://github.com/hy-huang20/rCore-N/commit/ed76dd59a7596158954c8b1ae358c400d5eadca6)。

## 更新-20251009

过程记录：

```
su
git clone https://github.com/hy-huang20/rCore-N.git
git clone https://github.com/duskmoon314/qemu.git
mkdir qemu-build
cd qemu-build
../qemu/configure --target-list="riscv64-softmmu"
make -j8

cd ..
cd rCore-N/
git checkout mydev
# 如果提示版本问题，可能是 just 版本太新了
cargo install just --version 1.14.0
cd os
rustup target add riscv64imac-unknown-none-elf
# 如果碰到 lock_api 问题，可根据输出提示进行依赖降级
# 个人尝试还需要修改 user/Cargo.toml 文件依赖
# 在 [dependencies] 下加一行 lock_api = "=0.4.10"
cargo update -p lock_api --precise 0.4.10

LOG=DEBUG just run
# 这时应该会遇到林晨记录中所说的执行 qemu 报串口错误
# 即林晨记录中图片里面的第 4 步。照做林晨的步骤
```

## 资料

- rCore-N 仓库地址：
https://github.com/duskmoon314/rCore-N
- 参考林晨学长的记录：
https://github.com/BITcyman/Rust-os-learning/blob/main/rCore-N.md
- 以及这篇博客：
https://zjp-cn.github.io/os-notes/async-os-dev-log_rCore-N.html

## 踩坑过程

项目文件组织

- rCore-N
- qemu
- qemu-build

需要让 rCore-N 和 qemu-build 位于同一文件夹下（原因可以参考 makefile, justfile 中关于 QEMU 的设置）

一些工具安装

```
cargo install cargo-binutils
rustup component add llvm-tools-preview
rustup component add rust-src
```

如果 install 失败则在后面添加 --locked 选项

rCore-N 作者的最新 Commit 似乎无法通过编译（2023-08，主要是 user/src/user_uart.rs 文件），参考林晨学长毕设时间（2023 上半年），于是使用这个 commit: e2f8266b26b70e4069cca0b3b9386b1917c36f8b 的内容（2023-03-30），编译通过。

qemu 启动一直有问题，于是去掉了两个 tcp 的 -serial，似乎可以看见正常输出了（暂时还未确定这样的改动会造成什么影响）

![](../img/serial0.png)

![](../img/serial1.png)
