# rust 中的 move 关键字

## 概念

- ``闭包``
    - [定义](https://doc.rust-lang.org/book/ch13-01-closures.html)：捕获它们所处环境的``匿名函数``
    - ``闭包的环境``：闭包创建时所能访问的外部作用域中的所有变量。闭包可以捕获这些变量并在自身内部使用
    - ``捕获``：闭包如何获取和使用其外部作用域中的变量
        - **默认**规则：按需自动选择
        
            闭包会根据变量的使用方式，自动选择最轻量的捕获方式。比如，如果只读取变量，则默认以不可变借用 ``&T`` 形式捕获；如果需要修改变量，则默认以可变借用 ``&mut T`` 形式捕获。
        - 需要延长变量[生命周期](./rust-lifetime.md)的情况：显式 ``move``

## 关于 move

除非变量实现了 ``Copy`` [trait](./rust-copy-clone-trait.md)，否则 ``move`` 会**强制**闭包获取所有捕获变量的所有权（按值捕获），无论闭包如何使用这些变量。之后外部不能再使用这些变量。

## 参考

- [Rust 官方文档](https://doc.rust-lang.org/std/keyword.move.html)
- [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-01-closures.html)