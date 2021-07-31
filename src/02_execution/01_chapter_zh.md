# 深入了解：执行 `Future` 和任务

在本章中，我们将介绍调度 `Future` 和异步任务的内部结构。
如果你只想学习如何编写使用 `Future` 类型的高级代码，
而对 `Future` 类型的工作原理不感兴趣，可以直接跳到 `async`/`await` 章节。
但是，本章中提及的几个主题，对理解 `async`/`await` 是如何实现的，
以及对应的运行时和性能特性，和构建新的异步原型大有帮助。
如果你现在决定跳过此章，那最好将它加入到书签中以便将来再重新审读它。

那么现在，让我们来聊一聊 `Future` 特征吧。