# 返回类型错误

典型的Rust函数中，返回值类型错误会导致如下错误：

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

然而，当前的`async fn`支持不知道要“相信”函数签名中写到的返回类型，导致不支配或者不合理
的错误。例如，函数`async fn foo() {"foo" }`会导致这样的错误：

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

这个错误是说*期望*`&str`但是却发现`()`类型，而实际上应该是要反过来。这是因为编译器错误地
相信函数体返回的才是正确的类型。

绕过这个问题的就是认出错误中指出的，带有”expected `SomeType`, found `OtherType`"信息
的函数签名，这统一意味着有一个或者多个返回是错误的。

这个问题的修复在[这个bug](https://github.com/rust-lang/rust/issues/54326)里跟踪。

## `Box<dyn Trait>`

类似的，因为函数签名中的返回类型没有正确地传递下去，`async fn`的返回值会不准确地解析为
他们的期望类型。

实践中，这意味着从`async fn`返回`Box<dyn Trait>`对象需要手动地从`Box<MyType>`类型
`as`声明为`Box<dyn Trait>`类型。

以下代码会报错:

```
async fn x() -> Box<dyn std::fmt::Display> {
    Box::new("foo")
}
```

这个问题能够用手工`as`声明的方法规避：

```
async fn x() -> Box<dyn std::fmt::Display> {
    Box::new("foo") as Box<dyn std::fmt::Display>
}
```

这个问题的修复在[这个bug](https://github.com/rust-lang/rust/issues/60424)里跟踪。
