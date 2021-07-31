# `Stream` 特征

`Stream` 特征类似于 `Future` 但是可以在完成前产生多个值，
亦类似于标准库中的 `Iterator` 特征。

```rust,ignore
{{#include ../../examples/05_01_streams/src/lib.rs:stream_trait}}
```

一个常见的 `Stream` 的例子是 `futures` crate 中 channel 类型的 `Receiver`。
每当 `Sender` 端发送一个数据，它都会产生一个 `Some(val)`，
而在通道里所有数据都被取出或 `Sender` 被删除时，则产生 `None`：

```rust,edition2018,ignore
{{#include ../../examples/05_01_streams/src/lib.rs:channels}}
```
