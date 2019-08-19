#![cfg(test)]
#![feature(async_await)]

// ANCHOR: imports
use {
    futures::{
        future::{FutureExt, BoxFuture},
        task::{ArcWake, waker_ref},
    },
    std::{
        future::Future,
        sync::{Arc, Mutex},
        sync::mpsc::{sync_channel, SyncSender, Receiver},
        task::{Context, Poll},
        time::Duration,
    },
    // 我们在上一章中写过的定时器:
    timer_future::TimerFuture,
};
// ANCHOR_END: imports

// ANCHOR: executor_decl
/// 从管道中接收任务并运行它们的执行程序.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner` 在任务管道中创建新的 `future`.
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// 一个可以重新安排自己被 `Executor` 调用的 `future`.
struct Task {
    /// 正在运行的 `future` 应该被推进到运行完成.
    ///
    /// 这个 `Mutex` 不是必要的, 因为我们一次只有一个线程
    /// 执行任务，但是，Rust不够聪明，没有办法知道 `future`
    /// 只会在一个线程中发生变化，所以我们需要 `Mutex` 来
    /// 让Rust知道我们保证了跨线程之间的安全性.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 将任务放回到任务队列.
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 允许通知在管道中排队的最大任务数.
    // 这只是为了让 `sync_channel` 满足, 并不会出现在真正的执行器中.
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender})
}
// ANCHOR_END: executor_decl

// ANCHOR: spawn_fn
impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}
// ANCHOR_END: spawn_fn

// ANCHOR: arcwake_for_task
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 通过将这个任务发送回任务管道来实现 `wake`，
        // 以便让执行器再次轮询它.
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks queued");
    }
}
// ANCHOR_END: arcwake_for_task

// ANCHOR: executor_run
impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 以 `future` 为例子，如果它还没有完成，就轮询并试图完成它.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 从任务自身创建一个 `LocalWaker`.
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>` 是 `Pin<Box<dyn Future<Output = T> + Send + 'static>>` 的类型别名.
                // 我们可以调用 `Pin::as_mut` 方法获得 `Pin<&mut dyn Future + Send + 'static>`.
                if let Poll::Pending = future.as_mut().poll(context) {
                    // 我们还没有完成对 `future` 的处理，所以把它再次
                    // 放回它的任务中，以便在某个时段再次运行.
                    *future_slot = Some(future);
                }
            }
        }
    }
}
// ANCHOR_END: executor_run

// ANCHOR: main
fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // 在定时器之前和之后创建一个要输出的任务.
    spawner.spawn(async {
        println!("howdy!");
        // 定时器在两秒钟之后完成.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // 释放这个 `spawner`，以便让我们的执行程序知道它已经工作
    // 完成，并且不会接收到更多要运行的任务传入.
    drop(spawner);

    // 运行执行器，直到任务队列为空.
    // 这将输出 "howdy!", 等待一会, 然后输出 "done!".
    executor.run();
}
// ANCHOR_END: main

#[test]
fn run_main() { main() }
