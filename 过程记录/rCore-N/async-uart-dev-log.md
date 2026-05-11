# rcoren 异步 uart 开发日志

接 [rcoren 异步 timer 开发日志](./async-timer-dev-log.md)后进行。

## 20260511

Commit ID: [f51e42e](https://github.com/hy-huang20/rCore-N/commit/f51e42ea7539ee0d8c270fa57c6e160a9f365b2c)

为求编译通过先在 feature = board_qemu 的 serial_config mod 中也加上相同的 RTS_PULSE_WIDTH 的定义，编译通过，uart_benchmark 也能正常运行：

```
/dev/pts/1
Rust user shell
>> uart_benchmark
[uart benchmark] User mode unbuffered async driver benchmark begin.
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
[uart load [uart load 32] Async mode, claim result: ]0 Asyxnc mode, claim result: 0x1000500010004000, enable res: , en0axble res: 0x00    

[uart 3] Unbuffered Async, refcnt: 3
[uart 3] Unbuffered Async, Intr count: 2, Tx: 
1, Rx: 1
[uart 3] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
[uart 2] Unbuffered Async, refcnt: 4
[uart 2] Unbuffered Async, Intr count: 1, Tx: 
1, Rx: 0
[uart 2] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
[uart benchmark] User mode unbuffered async driver benchmark finished.
Shell: Process 1 exited with code 0
```

试图同时跑 user 的 BufferedSerial 和 AsyncSerial 的结果：

```
>> uart_benchmark
[uart benchmark] User mode interrupt driver benchmark begin.
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
[uart load 2[uart load ] Interrupt mode, claim result: 0x130004000, enable res: ] Interrupt 
mode, claim result: 0x00x10005000
, enable res: 0x0
[uart 3] intr, Intr count: 2, Tx: 0, Rx: 2, err pos: -1
[uart 3] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
[uart 2] intr, Intr count: 1, Tx: 0, Rx: 1, err pos: -1
[uart 2] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
[uart benchmark] User mode interrupt driver benchmark finished.
[uart benchmark] User mode async driver benchmark begin.
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
[uart load 2] Async mode, claim result: 0x10004000, enable res: 0x0
[uart load 3] Async mode, claim result: 0x10005000, enable res: 0x0
[uart 3] Async, write: 2*247=494, read: 1*247=247, refcnt: 5
[uart 3] Async, Intr count: 2, Tx: 1, Rx: 1, err pos: -1
[uart 3] Test finished, 0 bytes sent, 16 bytes received, 0 bytes error.
[uart 2] Async, write: 22*247=5434, read: 1*247=247, refcnt: 6
[uart 2] Async, Intr count: 1, Tx: 1, Rx: 0, err pos: -1
[uart 2] Test finished, 16 bytes sent, 0 bytes received, 0 bytes error.
[uart benchmark] User mode async driver benchmark finished.
Shell: Process 1 exited with code 0
```

结果第二个出了问题。尝试单独跑 user 的 AsyncSerial 结果：

```
>> Rust user shell
>> uart_benchmark
[uart benchmark] User mode async driver benchmark begin.
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
[uart load 3] Async mode, claim result: 0x10005000, enable res: 0x0[uart load
2] Async mode, claim result: 0x10004000, enable res: 0x0
[uart 3] Async, write: 22*247=5434, read: 1*247=247, refcnt: 6
[uart 3] Async, Intr count: 3, Tx: 1, Rx: 2, err pos: -1
[uart 3] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
[uart 2] Async, write: 22*247=5434, read: 1*247=247, refcnt: 6
[uart 2] Async, Intr count: 2, Tx: 1, Rx: 1, err pos: -1
[uart 2] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
[uart benchmark] User mode async driver benchmark finished.
Shell: Process 1 exited with code 0
```

又正常了。看来 uart_benchmark 最好同一次只跑一种进行测试，别的测试代码像原作者那样先注释掉。

## 20260510

不知道为什么即使把波特率设置为 9600 后 uart_load 仍然会有巨量的 error bytes。林晨的成功记录和我之前的成功记录里面为什么字节都很少（比如 16 bytes），现在收发的字节会很多（最多有上万字节）。

草看错了，发现林晨是 2024 年春季做的毕设，而 rCore-N 最新只更新到 2023 年，因此完全可以用 rCore-N 最新的代码，而不是用 2023-03-30 的代码。难怪我运行 uart_benchmark 的输出和林晨不一样。这下完了，代码都是基于 2023-03-30 的 rCore-N 代码写的。

亡羊补牢只能手动把 2023-03-30 之后 rCore-N 原作者的更新的最新状态手动复制到我这边的 async-timer 分支了。2023-03-30 和最新状态之间的[区别](https://github.com/duskmoon314/rCore-N/compare/e2f8266..1099266)。

直接这样吧：

```bash
git checkout async-timer
git merge master
```

git merge master 起冲突了，幸好只有一个文件冲突，就是[之前](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rCore-N/async-timer-dev-log.md#20260507)试图同步最新 uart_load.rs 因此对其作了修改：

```
Auto-merging user/src/bin/uart_load.rs 
CONFLICT (content): Merge conflict in user/src/bin/uart_load.rs 
Auto-merging user/Cargo.toml 
Auto-merging os/justfile 
Auto-merging os/Cargo.toml 
Automatic merge failed; fix conflicts and then commit the result.
```

在 uart_load.rs 中一律选择 Accept Incoming Change 即可。

不出所料又又遇到了环境问题：

```
error: package `futures-task v0.3.32` cannot be built because it requires rustc 1.71 or newer, while the currently active rustc version is 1.68.0-nightly
Either upgrade to rustc 1.71 or newer, or use
cargo update -p futures-task@0.3.32 --precise ver
where `ver` is the latest version of `futures-task` supporting rustc 1.68.0-nightly
make: *** [Makefile:13: elf] Error 101
```

解决方法：

- 去 user/Cargo.toml 将 futures 的 version 从 0.3 改成 =0.3.28
- 去 user/Cargo.lock 中，将 futures 和那几个 futures-* 库的版本全部手动从 0.3.32 改成 0.3.28。由于是手动修改，需要删除对应的 checksum 校验和那一行，后续会自动重新生成。

这样环境问题应该就解决了。

这之后又遇到了海量的编译错误，不过都是类似：

```
error[E0425]: cannot find value `RTS_PULSE_WIDTH` in this scope 
```

而这个其实定义在同文件中的 serial_config mod 中。不知道原作者是怎么想的，这个量只在 feature = board_lrv 的 serial_config mod 中定义，但却在 feature = board_qemu 代码中也会使用。

