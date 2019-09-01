# `async`/`.await`

在[第一章]，我们简单介绍了`async`/`.await`，并且用它构建一个简单的服务器。这一章会详细
讨论`async`/`.await`，解释它如何工作以及`async`代码如何和传统Rust程序不同。

`async`/`.await`是特殊的Rust语法，使得让出当前线程控制权成为可能，而不是阻塞它，也允许
其他代码在等待一个车操作完成时取得进展。

有两种主要的方法使用`async`: `async fn`和`async`块。两种方法都返回一个实现了`Future`
特质的值：

```rust,no_run
{{#include ../../examples_zh/03_01_async_await/src/lib.rs:async_fn_and_block_examples}}
```

就像我们在第一章中看到，`async`体以及其他future类型是惰性的：除非它们运行起来，否则它们
什么都不做。运行`Future`最常见的方法是`.await`它。当`.await`在`Future`上调用时，它会
尝试把future跑到完成状态。如果`Future`被阻塞了，它会让出当前线程的控制权。能取得进展时，
执行器就会捡起这个`Future`并继续执行，让`.await`求解。

## `async`生命周期

和传统函数不同，`async fn`会获取引用以及其他`'static`生命周期参数，并返回被这些参数的
生命周期约束的`Future`：

```rust,no_run
{{#include ../../examples_zh/03_01_async_await/src/lib.rs:lifetimes_expanded}}
```

这意味着这些future被`async fn`函数返回后必须要在它的非`'static`参数仍然有效时`.await`。
在通常的场景中，future在函数调用后马上`.await`（例如`foo(&x).await`），并不会有
大问题。然而，如果储存了这些future或者把它发送到其他的任务或者线程，那就有问题了。

一个常用的规避方法以把带有引用参数的`async fn`转化成一个`'static`future是把这些参数
和应用的`async fn`函数调用封装到`async`块中：

```rust,no_run
{{#include ../../examples_zh/03_01_async_await/src/lib.rs:static_future_with_borrow}}
```
通过移动参数到`async`块中，我们把它的生命周期扩展到了匹配调用`foo`函数返回的`Future`的
生命周期。

## `async move`

`async`块和闭包允许使用`move`关键字，这和普通的闭包一样。一个`async move`块会获取
所指向变量的所有群，允许它超长存活（outlive）当前作用域，但是放弃了与其他代码共享这些
变量的能力：

```rust,no_run
{{#include ../../examples_zh/03_01_async_await/src/lib.rs:async_move_examples}}
```

## 在多线程执行器中`.await`

提醒一下，在使用多线程的`Future`执行器时，一个`Future`可能在线程间移动，所以任何在
`async`体重使用的变量必须能够穿过线程，所以任何`.await`都有可能导致切换到新线程。

这意味着使用`Rc`，`&RefCell`或者其他没有实现`Send`特质的类型是不安全的，包括那些指向
没有`Sync`特质类型的引用。

(告示：使用这些类型是允许的，只要他们不是在调用`.await`的作用域内。)

类似的，横跨`.await`持有一个非future感知的锁这种做法是很不好的，因为它能导致整个线程池
锁上：一个任务可能获得了所，`.await`然后让出到执行器，允许其他任务尝试获取所并导致死锁。
为了避免这种情况，使用`futures::lock`里的`Mutex`类型比起`std::sync`更好。

[第一章]: ../01_getting_started/04_async_await_primer.md
