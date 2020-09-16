use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

// Ideally this isn't an Enum that allows you to see what type it is..
pub enum JoinHandle<T>
where
    T: Send + 'static,
{
    Noop(T),
    #[cfg(feature = "tokio02")]
    Tokio(tokio::task::JoinHandle<T>),
    #[cfg(feature = "async-std16")]
    AsyncStd(async_std::task::JoinHandle<T>),
}

unsafe impl<T> Send for JoinHandle<T> where T: Send + 'static {}
unsafe impl<T> Sync for JoinHandle<T> where T: Send + 'static {}
impl<T> Unpin for JoinHandle<T> where T: Send + 'static {}

impl<T> Future for JoinHandle<T>
where
    T: Send + 'static,
{
    type Output = Result<T, ()>;

    #[allow(unused_mut, unused_variables)]
    fn poll(mut self: Pin<&mut JoinHandle<T>>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let slf = self.get_mut();

        match slf {
            JoinHandle::Noop(_) => panic!("Joining on a Noop join handle"),
            #[cfg(feature = "tokio02")]
            JoinHandle::Tokio(task) => {
                let pinned = Pin::new(task);
                pinned.poll(cx).map_err(|_| ())
            }
            #[cfg(feature = "async-std16")]
            JoinHandle::AsyncStd(task) => {
                let pinned = Pin::new(task);
                pinned.poll(cx).map(|r| Ok::<T, ()>(r))
            }
        }
    }
}
