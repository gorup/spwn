use spwn::Spwner;
use spwn::spwn;

#[tokio::test]
async fn basic_async_fn() {
    spwn::set_spwner(Spwner::Tokio).unwrap();
    assert_eq!(8, spwn(returns_eight()).await.unwrap());
}

async fn returns_eight() -> usize {
    8
}
