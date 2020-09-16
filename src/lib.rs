use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(unused_imports)]
use std::pin::Pin;

pub mod joinhandle;
pub mod macros;
pub use joinhandle::JoinHandle;

//use crate::joinhandle::JoinHandle;

pub(crate) static mut SPWNER: Spwner = Spwner::Noop;
pub(crate) static STATE: AtomicUsize = AtomicUsize::new(0);

/// spwner is uninitialized
const UNINITIALIZED: usize = 0;
/// spwner is initializing but not done
const INITIALIZING: usize = 1;
/// spwner is active and ready
const INITIALIZED: usize = 2;

/// Spwner variants. Would love to use a trait w/ generics, but alas the enum is all we have
pub enum Spwner {
    Noop,
    #[cfg(feature = "tokio02")]
    Tokio,
    #[cfg(feature = "async-std16")]
    AsyncStd,
}

// pub trait GenericFuture: Future + Send + 'static {}
// pub trait GenericFutureOutput: Send + 'static {}

pub fn spwn<T>(task: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    spwner().spwn(task)
}

pub fn spwn_blk<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    spwner().spwn_blk(f)
}

/// Sets up the spwner
pub fn set_spwner(spwner: Spwner) -> Result<(), ()> {
    unsafe {
        match STATE.compare_and_swap(UNINITIALIZED, INITIALIZING, Ordering::SeqCst) {
            UNINITIALIZED => {
                SPWNER = spwner;
                STATE.store(INITIALIZED, Ordering::SeqCst);
                Ok(())
            }
            INITIALIZING => {
                while STATE.load(Ordering::SeqCst) == INITIALIZING {}
                Err(())
            }
            _ => Err(()),
        }
    }
}

/// Gets a reference to the spwner
pub(crate) fn spwner() -> &'static Spwner {
    unsafe {
        if STATE.load(Ordering::SeqCst) != INITIALIZED {
            static NOP: Spwner = Spwner::Noop;
            &NOP
        } else {
            &SPWNER
        }
    }
}

impl Spwner {
    #[allow(unused_variables)]
    pub fn spwn<T>(&self, task: T) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        match self {
            Spwner::Noop => panic!("No spwner has been initialized"),
            #[cfg(feature = "tokio02")]
            Spwner::Tokio => JoinHandle::Tokio(tokio::spawn(task)),
            #[cfg(feature = "async-std16")]
            Spwner::AsyncStd => JoinHandle::AsyncStd(async_std::task::spawn(task)),
        }
    }

    #[allow(unused_variables)]
    pub fn spwn_blk<F, R>(&self, f: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        match self {
            Spwner::Noop => panic!("No spwner has been initialized"),
            #[cfg(feature = "tokio02")]
            Spwner::Tokio => JoinHandle::Tokio(tokio::task::spawn_blocking(f)),
            #[cfg(feature = "async-std16")]
            Spwner::AsyncStd => JoinHandle::AsyncStd(async_std::task::spawn_blocking(f)),
        }
    }
}
