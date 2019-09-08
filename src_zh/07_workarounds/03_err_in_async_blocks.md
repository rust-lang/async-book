# `?` in `async` Blocks

就像在`async fn`中，在`async`块中使用`?`也很常见。然而，`async`块的返回值类型并没有
显式说明。这会导致编译器无法推断（infer）`async`块的错误类型。

例如，以下代码：

```rust
let fut = async {
    foo().await?;
    bar().await?;
    Ok(())
};
```

会触发以下错误：

```
error[E0282]: type annotations needed
 --> src/main.rs:5:9
  |
4 |     let fut = async {
  |         --- consider giving `fut` a type
5 |         foo().await?;
  |         ^^^^^^^^^^^^ cannot infer type
```

很不行地，目前没有版本来"give `fut` a type"，也没有显式指定`async`块返回值类型的方法。
要规避这个问题，使用“多宝鱼（turbofish）”操作符来提供`async`块的成功与错误类型：

```rust
let fut = async {
    foo().await?;
    bar().await?;
    Ok::<(), MyError>(()) // <- note the explicit type annotation here
};
```
