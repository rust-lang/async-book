# 迭代和并发

与同步中的 `Iterator`s 类似，对 `Stream` 中的值进行迭代与处理的方法有多种。
有组合器风格的方法如 `map`、`filter` 和 `fold`，以及在它们错误时退出的变种
`try_map`、`try_filter` 和 `try_fold`。

不幸的是，`Stream`s 不能使用 `for` 循环，而只能使用命令式风格的代码，像
`while let` 和 `next`/`try_next` 函数：

```rust,edition2018,ignore
{{#include ../../examples/05_02_iteration_and_concurrency/src/lib.rs:nexts}}
```

但是，如果我们每次只处理一个元素，这样就潜在地留下了产生并发的机会，
毕竟这也就是我们首先编写异步代码的原因。在一个 stream 并发中，
可以使用 `for_each_concurrent` 和 `try_for_each_concurrent`
函数来处理多个项目：

```rust,edition2018,ignore
{{#include ../../examples/05_02_iteration_and_concurrency/src/lib.rs:try_for_each_concurrent}}
```
