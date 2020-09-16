use async_std::task::sleep;
use spwn::spwn;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

pub async fn forget_spwned_task() {
    let atomic = Arc::new(AtomicBool::new(false));
    let atomicclone = atomic.clone();

    spwn!(async move {
        atomic.store(true, Ordering::SeqCst);
    });

    sleep(Duration::from_millis(100)).await;

    assert!(atomicclone.load(Ordering::SeqCst));
}
