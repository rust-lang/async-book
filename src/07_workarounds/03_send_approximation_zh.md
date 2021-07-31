# `Send` 类

一些 `async fn` 状态机可以安全地跨线程发送，而另一些则不是。
`async fn` `Future` 是否为 `Send`，取决于是否跨 `.await` 持有非 `Send` 类型。
编译器会尽可能地预估出值可能通过 `.await` 的时间点，
但现在这种分析在许多地方都太过于保守。

比如，考虑一种简单的 non-`Send` 类型，也许只是一个包含 `Rc` 的类型：

```rust
use std::rc::Rc;

#[derive(Default)]
struct NotSend(Rc<()>);
```

即使 `async fn` 返回的结果必须是 `Send` 类型，但 Non-`Send` 类型变量，
也可短暂地作为临时变量在 `async fn` 里使用：

```rust,edition2018
# use std::rc::Rc;
# #[derive(Default)]
# struct NotSend(Rc<()>);
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

一旦我们将 `NotSend` 存储在变量里，这个例子就无法通过编译了：

```rust,edition2018
# use std::rc::Rc;
# #[derive(Default)]
# struct NotSend(Rc<()>);
# async fn bar() {}
async fn foo() {
    let x = NotSend::default();
    bar().await;
}
# fn require_send(_: impl Send) {}
# fn main() {
#    require_send(foo());
# }
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

这个报错是正确的。如果我们将 `x` 存储在一个变量里，在 `.await`
之后它才会被删除，而此时 `async fn` 可能在其它的进程中运行。
因为 `Rc` 不是 `Send`，它不能安全地在线程间传输。一个简单的解决办法是，
在 `.await` 之前删除 `Rc`，但遗憾的是目前无法这么做。

你可以通过使用一个代码块（{}）来包裹住所有的 non-`Send` 变量，这可解决这个问题。
这样就很方便的告知编译器，这些变量在 `.await` 前就被丢弃了。

```rust,edition2018
# use std::rc::Rc;
# #[derive(Default)]
# struct NotSend(Rc<()>);
# async fn bar() {}
async fn foo() {
    {
        let x = NotSend::default();
    }
    bar().await;
}
# fn require_send(_: impl Send) {}
# fn main() {
#    require_send(foo());
# }
```
