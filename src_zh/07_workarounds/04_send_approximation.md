# `Send`模拟

一些`async fn`状态机是可以安全地跨线程传递（Send）的，但另外的不可以。一个`async fn`
的`Future`是否`Send`取决于是否有非`Send`类型跨越`.await`点被持有了。当编译器发现
有些值可能会跨`.await`持有时。编译器尽可能地模拟`Send`，但是这种分析今天在一些地方过于
保守。

例如，考虑一个简单的非`Send`类型，可能是一种持有`Rc`的类型：

```rust
use std::rc::Rc;

#[derive(Default)]
struct NotSend(Rc<()>);
```

类型`NotSend`的变量可能会很简单地作为临时变量出现在`async fn`函数中，甚至会出现在
`async fn`函数返回的`Future`类型必须是`Send`的时候：

```rust
async fn bar() {}
async fn foo() {
    NotSend::default();
    bar().await;
}

fn require_send(_: impl Send) {}

fn main() {
    require_send(foo());
}
```

然而，如果我们改动`foo`来存一个`NotSend`变量，这个例子就不再编译了：

```rust
async fn foo() {
    let x = NotSend::default();
    bar().await;
}
```

```
error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
  --> src/main.rs:15:5
   |
15 |     require_send(foo());
   |     ^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
   |
   = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
   = note: required because it appears within the type `NotSend`
   = note: required because it appears within the type `{NotSend, impl std::future::Future, ()}`
   = note: required because it appears within the type `[static generator@src/main.rs:7:16: 10:2 {NotSend, impl std::future::Future, ()}]`
   = note: required because it appears within the type `std::future::GenFuture<[static generator@src/main.rs:7:16: 10:2 {NotSend, impl std::future::Future, ()}]>`
   = note: required because it appears within the type `impl std::future::Future`
   = note: required because it appears within the type `impl std::future::Future`
note: required by `require_send`
  --> src/main.rs:12:1
   |
12 | fn require_send(_: impl Send) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
```

这个错误是正确的。如果我们把`x`存到变量中去，它不会被丢弃（drop），直到`.await`之后，
这时`async fn`可能在另外一个线程中运行。因为`Rc`不是`Send`的，允许它穿过线程是不合理的。
一个简单的解决方法是应该在`.await`之前`drop`掉这个`Rc`，但是不幸的是现在这种方法还不能
工作。

为了规避这个问题，你可能需要引入一个块作用域来封装任何非`Send`变量。这会让编译器更容易
发现这些变量不会存活超过`.await`点。

```rust
async fn foo() {
    {
        let x = NotSend::default();
    }
    bar().await;
}
```
