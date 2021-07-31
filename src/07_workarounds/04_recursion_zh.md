# 递归

在内部，`async fn` 创建了一个包含每个 `.await` 的子 `Future` 的状态机类型。
因为这种结果状态机必然包括其自身，这使得递归 `async fn` 变得有点儿麻烦了：

```rust,edition2018
# async fn step_one() { /* ... */ }
# async fn step_two() { /* ... */ }
# struct StepOne;
# struct StepTwo;
// This function:
async fn foo() {
    step_one().await;
    step_two().await;
}
// generates a type like this:
enum Foo {
    First(StepOne),
    Second(StepTwo),
}

// So this function:
async fn recursive() {
    recursive().await;
    recursive().await;
}

// generates a type like this:
enum Recursive {
    First(Recursive),
    Second(Recursive),
}
```

我们创建了一个无限大的类型，这将无法工作，编译器会报怨道：

```
error[E0733]: recursion in an `async fn` requires boxing
 --> src/lib.rs:1:22
  |
1 | async fn recursive() {
  |                      ^ an `async fn` cannot invoke itself directly
  |
  = note: a recursive `async fn` must be rewritten to return a boxed future.
```

为了解决这个，我们必须通过 `Box` 来间接引用它。不幸的是，编译器的限制规则中，
我们仅仅使用 `Box::pin` 来包装对 `recursive()` 的调用是不够的。
为了使它能工作，我们必须将 `recursive` 放进一个 non-`async` 函数中，
它返回一个 `.boxed()` 的 `async` 代码块。

```rust,edition2018
{{#include ../../examples/07_05_recursion/src/lib.rs:example}}
```
