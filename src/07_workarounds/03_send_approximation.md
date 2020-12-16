# `Send` Approximation

Some `async fn` state machines are safe to be sent across threads, while
others are not. Whether or not an `async fn` `Future` is `Send` is determined
by whether a non-`Send` type is held across an `.await` point. The compiler
does its best to approximate when values may be held across an `.await`
point, but this analysis is too conservative in a number of places today.

For example, consider a simple non-`Send` type, perhaps a type
which contains an `Rc`:

```rust
use std::rc::Rc;

#[derive(Default)]
struct NotSend(Rc<()>);
```

Variables of type `NotSend` can briefly appear as temporaries in `async fn`s
even when the resulting `Future` type returned by the `async fn` must be `Send`:

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

However, if we change `foo` to store `NotSend` in a variable, this example no
longer compiles:

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

This error is correct. If we store `x` into a variable, it won't be dropped
until after the `.await`, at which point the `async fn` may be running on
a different thread. Since `Rc` is not `Send`, allowing it to travel across
threads would be unsound. One simple solution to this would be to `drop`
the `Rc` before the `.await`, but unfortunately that does not work today.

In order to successfully work around this issue, you may have to introduce
a block scope encapsulating any non-`Send` variables. This makes it easier
for the compiler to tell that these variables do not live across an
`.await` point.

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
