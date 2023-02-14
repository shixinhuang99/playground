use std::{
    future::Future,
    pin::Pin,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl SharedState {
    fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }))
    }
}

struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl TimerFuture {
    fn new(duration: Duration) -> Self {
        let shared_state = SharedState::new();

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

const QUEUE_BUSY: &'static str = "QUEUE BUSY";

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let arc_self_cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(arc_self_cloned)
            .expect(QUEUE_BUSY);
    }
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect(QUEUE_BUSY)
    }
}

fn new_excutor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASK: usize = 10000;

    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASK);

    (Executor { ready_queue }, Spawner { task_sender })
}

fn main() {
    let (excutor, spawner) = new_excutor_and_spawner();

    for i in 0..10 {
        spawner.spawn(async move {
            println!("start: {}", i);
            TimerFuture::new(Duration::from_secs(2)).await;
            println!("end: {}", i);
        });
    }

    drop(spawner);

    excutor.run();
}
