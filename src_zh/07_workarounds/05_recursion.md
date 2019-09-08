# 递归

在内部，`async fn`创建了一个包含了要`.await`的子`Future`的状态机。这样递归的`async fn`
有点诡异，因为结果的状态机必须包含它自身：

```rust
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

这不会工作——我们创建了大小为无限大的类型！
编译器会抱怨：

```
error[E0733]: recursion in an `async fn` requires boxing
 --> src/lib.rs:1:22
  |
1 | async fn recursive() {
  |                      ^ an `async fn` cannot invoke itself directly
  |
  = note: a recursive `async fn` must be rewritten to return a boxed future.
```

为了允许这种做法，我们需要用`Box`来间接调用。而不幸的是，编译器限制意味着把`recursive()`
的调用包裹在`Box::pin`并不够。为了让递归调用工作，我们必须把`recursive`转换成非`async`
函数，然后返回一个`.boxed()`的异步块

```rust
{{#include ../../examples/07_05_recursion/src/lib.rs:example}}
```
