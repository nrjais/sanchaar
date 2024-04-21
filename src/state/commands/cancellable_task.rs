use iced::advanced::graphics::futures::MaybeSend;
use std::future::Future;
use tokio::sync::oneshot;

pub enum TaskResult<A> {
    Completed(A),
    Cancelled,
}

pub fn cancellable_task<A>(
    future: impl Future<Output = A> + 'static + MaybeSend,
) -> (
    oneshot::Sender<()>,
    impl Future<Output = TaskResult<A>> + 'static + MaybeSend,
) {
    let (tx, rx) = oneshot::channel();
    let fut = async {
        tokio::select! {
            _ = rx => TaskResult::Cancelled,
            r = future => TaskResult::Completed(r),
        }
    };

    (tx, fut)
}
