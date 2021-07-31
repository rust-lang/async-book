# 一次执行多个 Futures

到目前为止，我们都是主要通过 `.await` 来运行 futures，它会阻塞当前进程，
直到一个特定的 `Future` 完成。然而，真正的异步程序通常需要同时运行多个不同的操作。

在本章中，我们将介绍几种可同时执行多个异步操作的方法：

- `join!`：等待直到 futures 全部完成
- `select!`：在多个 futures 中等待其中一个完成
- Spawning：创建一个顶级任务，去推动 future 完成。
- `FuturesUnordered`：一个 futures 组，来为其中的每个子 future 产生结果 
