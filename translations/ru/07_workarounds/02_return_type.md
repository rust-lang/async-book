# Ошибки вывода для возвращаемых типов

В типичной функции Rust возврат значения неправильного типа 
приведёт к тому, что мы увидим примерно такую ошибку:

```
error[E0308]: mismatched types
 --> src/main.rs:2:12
  |
1 | fn foo() {
  |           - expected `()` because of default return type
2 |     return "foo"
  |            ^^^^^ expected (), found reference
  |
  = note: expected type `()`
             found type `&'static str`
```

Однако текущая версия `async fn` не знает как 
"доверять" возвращаемому типу, записанному в сигнатуре 
функции, что приводит к не совпадающим или `reversed-sounding` 
ошибкам. Например, для функции `async fn foo() { "foo" }` 
будет следующая ошибка:

```
error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
 --> src/lib.rs:1:16
  |
1 | async fn foo() {
  |                ^ expected &str, found ()
  |
  = note: expected type `&str`
             found type `()`
  = note: the return type of a function must have a statically known size
```

Ошибка говорит, что *ожидает* `&str`, а 
находит` (`), что совершенно противоположно тому, 
что мы хотим. Это потому, что компилятор
доверяет телу функции для возврата правильного типа, а это неправильно.

Временным решением для этой проблемы является признание 
факта, что ошибка, указывающая на сигнатуру функции с 
сообщением "expecte`d SomeTy`pe, foun`d 
OtherT`ype", обычно показывает, что один или 
несколько возвращаемых вариантов не корректны.

Исправление этой ошибки отслеживается [здесь](https://github.com/rust-lang/rust/issues/54326).

## `Box<dyn Trait>`

Аналогично, так как возвращаемый тип из сигнатуры функции не 
распространяется должным образом, значение, которое 
возвращает`async fn` не правильно приводится к 
ожидаемому типу.

На практике, это означает, что возвращаемый из `async fn`
объект `Box<dyn Trait>` требует ручного 
преобразования при помощи` a`s из` 
Box<MyType&g`t; `в Box<dyn Trait&g`t;.

Этот код приведёт к ошибке:

```
async fn x() -> Box<dyn std::fmt::Display> {
    Box::new("foo")
}
```

Временным решением для этого будет ручное преобразование с 
использованием` a`s:

```
async fn x() -> Box<dyn std::fmt::Display> {
    Box::new("foo") as Box<dyn std::fmt::Display>
}
```

Исправление этой ошибки отслеживается [здесь](https://github.com/rust-lang/rust/issues/60424).
