# `select!`

`futures::select`宏同时跑多个future，允许用户在任意future完成时响应：

```rust,no_run
{{#include ../../examples/06_03_select/src/lib.rs:example}}
```

上面的函数会并发跑`t1`和`t2`。当`t1`和`t2`结束时，对应的句柄（handler）会调用`println!`，
然后函数就会结束而不会完成剩下的任务。

`select`的基本格式为`<pattern> = <expression> => <code>,`，可以重复你想`select`的
任意多future。

## `default => ...` and `complete => ...`

`select`也支持`default`和`complete`分支。

`default`会在没有被`select`的future完成时执行，因此，带有`default`分支的`select`总是
马上返回，因为`default`会在没有其它future准备好的时候返回。

`complete`分支则用来处理所有被`select`的future都完成并且不需进一步处理的情况。这在循环
`select`时很好用：

```rust,no_run
{{#include ../../examples/06_03_select/src/lib.rs:default_and_complete}}
```

## 和`Unpin`与`FusedFuture`交互
你会注意到，在上面第一个例子中，我们在两个`async fn`函数返回的future上调用了`.fuse()`，
然后用`pin_mut`来固定他们。这两个调用都是必需的，用在`select`中的future必须实现`Unpin`
和`FusedFuture`。

需要`Unpin`是因为`select`是用可变引用访问future的，不获取future的所有权。未完成的future
因此可以在`select`调用后继续使用。


类似的，需要`FusedFuture`是因为`select`一定不能轮询已完成的future。`FusedFuture`用来
实现追踪（track）future是否已完成。这种使得在循环中使用`select`成为客官您，只轮询尚未
完成的future。这可以从上面的例子中看出，`a_fut`或`b_fut`可能会在第二次循环的时候已经
完成了。因为`future::ready`返回的future实现了`FusedFuture`，所以`select`可以知道不必
再次轮询它了。

注意，stream也有对应的`FusedStream`特质。实现了这个特质或者被`.fuse()`包装的Stream会
从它们的`.next`/`try_next()`组合子中返还`FusedFutre`。

```rust,no_run
{{#include ../../examples/06_03_select/src/lib.rs:fused_stream}}
```

## 带有`Fuse`和`FuturesUnordered`的`select`循环中的并发任务

有个不太好找但是很趁手的函数叫`Fuse::terminated()`。这个函数允许构造已经被终止的空
future，并且能够在之后填进需要运行的future。
这个在一个任务需要`select`循环中运行但是它本身是在`select`循环中创建的场景中很好用。

注意`.select_next_some()`函数的是使用。这可以用在`select`上，并且只运行从stream返回的
`Some(_)`值而忽略`None`。

```rust,no_run
{{#include ../../examples/06_03_select/src/lib.rs:fuse_terminated}}
```

当有很多份相同future的拷贝同时执行时，使用`FutureUnordered`类型。下面的例子和上面的
例子很类似，但会运行`run_on_new_num_fut`的所有拷贝都到完成状态，而不是当一个新拷贝
创建时就中断他们。它也会打印`run_on_new_num_fut`的返回值：

```rust,no_run
{{#include ../../examples/06_03_select/src/lib.rs:futures_unordered}}
```
