use std::future::Future;
use std::pin::Pin;
use std::sync::OnceLock;
use std::task::{Context, Poll};
use tokio::runtime::{Builder, Handle, Runtime};

/// Polls `future` with the dedicated UniFFI runtime entered as the current
/// Tokio context. This lets SlateDB capture the multi-threaded runtime handle
/// during open without moving synchronous foreign callbacks onto runtime worker
/// threads.
pub(crate) fn enter<F>(future: F) -> impl Future<Output = F::Output>
where
    F: Future,
{
    EnterRuntime {
        handle: runtime().handle().clone(),
        future: Box::pin(future),
    }
}

// Wraps an open future so every poll occurs inside `handle.enter()`.
//
// Entering once around `.await` would hold the enter guard across suspension,
// leaking the runtime context back to the caller. Entering per poll ensures the
// guard is dropped before returning `Poll::Pending` or `Poll::Ready`.
struct EnterRuntime<F> {
    handle: Handle,
    future: Pin<Box<F>>,
}

impl<F: Future> Future for EnterRuntime<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let _guard = this.handle.enter();
        this.future.as_mut().poll(cx)
    }
}

pub(crate) fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        // The runtime must outlive every Db/DbReader opened through UniFFI,
        // because their background tasks keep using the handle captured at open.
        Builder::new_multi_thread()
            .enable_all()
            .thread_name("slatedb-uniffi-rt")
            .build()
            .expect("failed to build SlateDB UniFFI runtime")
    })
}
