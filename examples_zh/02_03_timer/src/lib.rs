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
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
// ANCHOR_END: future_for_timer

// ANCHOR: timer_new
impl TimerFuture {
    /// Create a new `TimerFuture` which will complete after the provided
    /// timeout.
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
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
