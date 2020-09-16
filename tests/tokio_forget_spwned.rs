use spwn::Spwner;

mod util_forget_spwned;
use util_forget_spwned::forget_spwned_task;

#[tokio::test]
async fn test_forget_spwned_task() {
    spwn::set_spwner(Spwner::Tokio).unwrap();

    forget_spwned_task().await;
}
