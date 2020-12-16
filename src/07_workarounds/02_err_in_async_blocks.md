# `?` in `async` Blocks

Just as in `async fn`, it's common to use `?` inside `async` blocks.
However, the return type of `async` blocks isn't explicitly stated.
This can cause the compiler to fail to infer the error type of the
`async` block.

For example, this code:

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

will trigger this error:

```
error[E0282]: type annotations needed
 --> src/main.rs:5:9
  |
4 |     let fut = async {
  |         --- consider giving `fut` a type
5 |         foo().await?;
  |         ^^^^^^^^^^^^ cannot infer type
```

Unfortunately, there's currently no way to "give `fut` a type", nor a way
to explicitly specify the return type of an `async` block.
To work around this, use the "turbofish" operator to supply the success and
error types for the `async` block:

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

