#![feature(async_await)]

// ANCHOR: imports
use {
    std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    },
};
// ANCHOR_END: imports

// ANCHOR: timer_decl
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在 `future` 线程和 `awiting` 线程之间共享状态
struct SharedState {
    /// 是否已经达到休眠时间.
    completed: bool,

    /// `TimerFuture` 表示正在运行的 `waker`.
    /// 线程可以在设置完 `completed = true` 之后来通知 `TimerFuture` 任务被唤醒并
    /// 检查 `completed = true`，然后继续执行.
    waker: Option<Waker>,
}
// ANCHOR_END: timer_decl

// ANCHOR: future_for_timer
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 检查共享状态，检查定时器是否已经完成.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // 设置 `waker`， 让线程可以在定时器完成时唤醒当前 `waker`，确保
            // 再次轮询 `future` 并获知 `completed = true`.
            //
            // 这样做是非常不错的，而不用每次都重复 `clone` `waker`. 然而, 这个 `TimerFuture`
            // 可以在执行器之间移动, 这可能会导致旧的 `waker` 指向错误的 `waker`, 这会阻止 
            // `TimerFuture` 被正确得唤醒.
            //
            // 注意：可以使用 `Waker::will_wake` 函数来做检查, 但是
            // 为了简单起见，我们忽略了他.
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
// ANCHOR_END: future_for_timer

// ANCHOR: timer_new
impl TimerFuture {
    /// 创建一个新的 `TimerFuture`，它将在提供的超时之后完成.
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // 创建一个新的线程.
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 设置状态，表示定时器已经完成，并唤醒轮询 `future` 中的最后一个
            // 任务 (如果存在的话).
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}
// ANCHOR_END: timer_new

#[test]
fn block_on_timer() {
    futures::executor::block_on(async {
        TimerFuture::new(Duration::from_secs(1)).await
    })
}
