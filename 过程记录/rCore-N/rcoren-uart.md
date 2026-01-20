# rCore-N 串口逻辑

os 部分：os/src/uart.rs 中提供了 `BufferedSerial`

user 部分：user/src/user_uart.rs 中提供了 `BufferedSerial` 和 [AsyncSerial](https://github.com/duskmoon314/rCore-N/commit/72bffef189ee4d0fce712ba714c0727f13b7bf45)

关于 user 部分的 `AsyncSerial`

使用的 Executor 为外部库：(Cargo.toml)

```
executor = { git = "https://github.com/rcore-os/executor" }
```

Waker 为 rust 自带

然后林晨的那个 async-uart-driver 是 Executor 拿的上面的库的，waker.rs 拿的 Embassy 的，serial.rs 中的 BufferedSerial 和 AsyncSerial 拿的 rCore-N 的。然后 task.rs 中的逻辑的来源暂时没找到。

TODO：后续需要弄清楚 os 部分和 user 部分的实现逻辑