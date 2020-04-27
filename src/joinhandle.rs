use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::marker::Unpin;


pub enum JoinHandle<T> 
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
{
    Noop(T),
    #[cfg(feature = "tokio02")]
    Tokio(tokio::task::JoinHandle<T::Output>),
    #[cfg(feature = "async-std15")]
    AsyncStd(async_std::task::JoinHandle<T::Output>),
}

unsafe impl<T> Send for JoinHandle<T>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
{}
unsafe impl<T> Sync for JoinHandle<T>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
{}
impl<T> Unpin for JoinHandle<T> 
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
{}

impl<T> Future for JoinHandle<T> 
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
{
    type Output = Result<T::Output, ()>;

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
            #[cfg(feature = "async-std15")]
            JoinHandle::AsyncStd(task) => {
                let pinned = Pin::new(task);
                pinned.poll(cx).map(|r| Ok::<T::Output, ()>(r))
            }
        }
    }
}