// ANCHOR: simple_future
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
// ANCHOR_END: simple_future

struct Socket;
impl Socket {
    fn has_data_to_read(&self) -> bool {
        // 检查当前 `socket` 是否可读.
        true
    }
    fn read_buf(&self) -> Vec<u8> {
        // 从 `socket` 中读取数据.
        vec![]
    }
    fn set_readable_callback(&self, _wake: fn()) {
        // 注册 `_wake` 并在 `socket` 变成可读的时候调用它.
        // 类似基于 `epoll` 的事件循环.
    }
}

// ANCHOR: socket_read
pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // `socket` 有数据的时候将其读取并放置在缓冲区并返回.
            Poll::Ready(self.socket.read_buf())
        } else {
            // `socket` 还没有数据.
            //
            // 当数据来到，将调用 `wake`.
            // 这个 `future` 的调用者将知道何时调用 `poll` 并接收数据.
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}
// ANCHOR_END: socket_read

// ANCHOR: join
/// 一个基本的 `future`，它将同时运行其他两个 `future` 直到完成.
///
/// 并发特性是通过对每个 `future` 的轮询交错调用来实现的，
/// 从而允许每个 `future` 以自己的速度前进.
pub struct Join<FutureA, FutureB> {
    // 每个字段可能包含应该运行完成的 `future`.
    // 如果 `future` 运行完成，则将该字段设置为 `None`.
    // 这可以防止我们在运行完成之后再次对 `future` 轮询，
    // 这将不符合 `future` trait 的规范.
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        // 尝试运行完成这个 future `a`.
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        // 尝试运行完成这个 future `b`.
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            // 两个 `future` 都已经完成，我们可以返回成功回调.
            Poll::Ready(())
        } else {
            // 一个或者两个 `future` 都返回了 `Poll::Pending`，说明仍需要做其他工作.
            // 当有新的进度时，他们将调用 `wake()`.
            Poll::Pending
        }
    }
}
// ANCHOR_END: join

// ANCHOR: and_then
/// 这是一个 `SimpleFuture`，依次运行直到两个 `future` 都完成.
//
// 提示: 这只是一个简单的示例, `AndThenFut` 是假设两个 `future` 在创建的时候都可用. 
// 真正的 `AndThen` 允许基于第一个 `future` 输出并创建第二个 `future`, 比
// 如 `get_breakfast.and_then(|food| eat(food))`.
pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                // 我们已经完成了第一个 `future`
                // 移除它，开始第二个.
                Poll::Ready(()) => self.first.take(),
                // 我们没有完成第一个 `future`.
                Poll::Pending => return Poll::Pending,
            };
        }
        // 现在，第一个 `future` 已经完成，
        // 那么就尝试完成第二个.
        self.second.poll(wake)
    }
}
// ANCHOR_END: and_then

mod real_future {
use std::{
    future::Future as RealFuture,
    pin::Pin,
    task::{Context, Poll},
};

// ANCHOR: real_future
trait Future {
    type Output;
    fn poll(
        // 注意这个 `&mut self` 到 `Pin<&mut Self>` 的变化:
        self: Pin<&mut Self>,
        // 以及从 `wake: fn()` 到 `cx: &mut Context<'_>` 的变化:
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}
// ANCHOR_END: real_future

// 为 `RealFuture` 实现 `Future` trait:
impl<O> Future for dyn RealFuture<Output = O> {
    type Output = O;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        RealFuture::poll(self, cx)
    }
}
}
