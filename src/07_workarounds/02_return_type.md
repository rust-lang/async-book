# Return Type Errors

In a typical Rust function, returning a value of the wrong type will result
in an error that looks something like this:

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

However, the current `async fn` support doesn't know to "trust" the return
type written in the function signature, causing mismatched or even
reversed-sounding errors. For example, the function
`async fn foo() { "foo" }` results in this error:

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

The error says that it *expected* `&str` and found `()`,
which is actually the exact opposite of what you'd want. This is because the
compiler is incorrectly trusting the function body to return the correct type.

The workaround for this issue is to recognize that errors pointing to the
function signature with the message "expected `SomeType`, found `OtherType`"
usually indicate that one or more return sites are incorrect.

A fix to this issue is being tracked in [this bug](https://github.com/rust-lang/rust/issues/54326).

## `Box<dyn Trait>`

Similarly, because the return type from the function signature is not
propagated down correctly, values returned from `async fn` aren't correctly
coerced to their expected type.

In practice, this means that returning `Box<dyn Trait>` objects from an
`async fn` requires manually `as`-casting from `Box<MyType>` to
`Box<dyn Trait>`.

This code will result in an error:

```
async fn x() -> Box<dyn std::fmt::Display> {
    Box::new("foo")
}
```

This issue can be worked around by manually casting using `as`:

```
async fn x() -> Box<dyn std::fmt::Display> {
    Box::new("foo") as Box<dyn std::fmt::Display>
}
```

A fix to this issue is being tracked in [this bug](https://github.com/rust-lang/rust/issues/60424).
