# 特征中的 `async`

目前，`async fn` 不能在特性中使用。其原因很繁杂，但现在已计划将来取消这一限制。

然而，目前，你可以使用这个替代方法
[async-trait crate from crates.io](https://github.com/dtolnay/async-trait)。

注意，使用这些 trait 方法，在每次功能调用时都会导致堆内存分配。
这对大多数的程序来说，这样的成本代价是可接受的，但是，请仔细考虑，
是否在可能每秒产生上百万次调用的低端公共 API 上使用它。
