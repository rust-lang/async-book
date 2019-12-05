# `Send` Approximation

Некоторые машины состояний асинхронных функций безопасны
для передачи между потокам, в то время как другие - нет. Так или
иначе, `async fn` `Future` является
`Send` если тип, содержащийся в
`.await`, тоже `Send`. Компилятор
делает всё возможное, чтобы при близиться к моменту, когда
значения могут удерживаться в `.await`, но сейчас в
некоторых местах этот анализ слишком консервативен.

Например, рассмотрим простой не-`Send` тип,
например, содержащий `Rc`:

```rust
use std::rc::Rc;

#[derive(Default)]
struct NotSend(Rc<()>);
```

Переменные типа `NotSend` могут появляться как
временные внутри `async fn` даже когда тип `Future`,
возвращаемой из `async fn` должен быть
`Send`:

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

Но если мы изменим `foo` таким образом, что она
будет хранить `NotSend` в переменной, пример не
скомпилируется:

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

Эта ошибка корректна. Если мы сохраним `x` в
переменную, она не будет удалена пока не будет завершён
`.await`. В этот момент `async fn` может
быть запущена в другом потоке. Так как `Rc` не
является `Send`, перемещение между потоками будет
некорректным. Простым решением будет вызов
`drop` у `Rc` до вызова
`.await`, но к сожалению пока что это не работает.

Для того, чтобы успешно обойти эту проблему, вы можете создать
блок, инкапсулирующий любые не-`Send`
переменные. С помощью этого, компилятору будет проще понять,
что такие переменные не переживут момент вызова
`.await`.

```rust
async fn foo() {
    {
        let x = NotSend::default();
    }
    bar().await;
}
```
