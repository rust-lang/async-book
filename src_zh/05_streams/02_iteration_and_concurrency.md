# 迭代与并发

与同步的`Iterator`类似，有很多不同的方法可以迭代处理`Stream`中的值。有很多组合子风格
的方法，如`map`，`filter`和`fold`，以及它们的“遇错即断”版本`try_map`，`try_filter`和
`try_fold`。

不幸的是，`for`循环不能用在`Stream`上，但是对于命令式编程风格（imperative style）的
代码，`while let`以及`next`/`try_next`函数还可以使用：

```rust
{{#include ../../examples/05_02_iteration_and_concurrency/src/lib.rs:nexts}}
```

然而，如果我们每次只处理一个元素，我们就要是去并发的机会，毕竟这是我们编写异步代码的首要
目的。为了并发处理一个`Stream`的多个值，使用`for_each_concurrent`或
`try_for_each_concurrent`方法：

```rust,no_run
{{#include ../../examples/05_02_iteration_and_concurrency/src/lib.rs:try_for_each_concurrent}}
```
