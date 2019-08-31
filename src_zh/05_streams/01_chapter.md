# `Steam`特质

`Stream`特质与`Future`类似，但能在完成前返还（yield）多个值，与标准库中的`Iterator`
类似：

```rust,no_run
{{#include ../../examples/05_01_streams/src/lib.rs:stream_trait}}
```

一个常见的使用`Stream`的例子是`futures`库中通道的`Receiver`。每次`Sender`端发送一个值
时，它就会返回一个`Some(val)`，并且会在`Sender`关闭且所有消息都接收后返还`None`:

```rust,no_run
{{#include ../../examples/05_01_streams/src/lib.rs:channels}}
```
