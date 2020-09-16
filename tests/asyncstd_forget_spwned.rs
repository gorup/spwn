use spwn::Spwner;

mod util_forget_spwned;
use util_forget_spwned::forget_spwned_task;

#[async_std::test]
async fn test_forget_spwned_task() {
    spwn::set_spwner(Spwner::AsyncStd).unwrap();

    forget_spwned_task().await;
}
