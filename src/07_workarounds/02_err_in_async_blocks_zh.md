# `async` 代码块中的 `?`

在 `async` 代码块中，可以同在 `async fn` 中一样使用 `?` 语法。
但是，由于 `async` 代码块的返回值却并非是明确声明的。
这可能导致编译器无法推断出 `async` 代码块中出现的错误的类型。

例如下面这段代码：

```rust,edition2018
# struct MyError;
# async fn foo() -> Result<(), MyError> { Ok(()) }
# async fn bar() -> Result<(), MyError> { Ok(()) }
let fut = async {
    foo().await?;
    bar().await?;
    Ok(())
};
```

会引发这种错误：

```
error[E0282]: type annotations needed
 --> src/main.rs:5:9
  |
4 |     let fut = async {
  |         --- consider giving `fut` a type
5 |         foo().await?;
  |         ^^^^^^^^^^^^ cannot infer type
```

遗憾的是，目前我们没有办法为 `fut` 指定一个类型，也没办法明确说明 `async`
代码块返回的具体类型。要解决这个问题，可以使用 `turbofish` 运算符，
它可以为 `async` 代码块提供成功和错误对应的类型：

```rust,edition2018
# struct MyError;
# async fn foo() -> Result<(), MyError> { Ok(()) }
# async fn bar() -> Result<(), MyError> { Ok(()) }
let fut = async {
    foo().await?;
    bar().await?;
    Ok::<(), MyError>(()) // <- note the explicit type annotation here
};
```

