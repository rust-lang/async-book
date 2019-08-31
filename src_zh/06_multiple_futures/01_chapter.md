# 同时执行多个Future

直到现在，我们几乎只用`.await`来执行future，而这会阻塞并发任务，直到特定的`Future`完成。
然而，真实的异步应用经常需要并发执行几个不同的操作。

这一章，我们会覆盖一些同事执行多个异步操作的方法：

- `join!`：等待所有future完成
- `select!`：等待其中一个future完成
- 开辟（Spawning）: 创建顶层任务，运行future至完成
- `FuturesUnordered`: 一组返还子future的future
