use spwn::Spwner;
use spwn::spwn;

#[async_std::test]
async fn basic_async_fn() {
    spwn::set_spwner(Spwner::AsyncStd).unwrap();
    assert_eq!(8, spwn(returns_eight()).await.unwrap());
}

async fn returns_eight() -> usize {
    8
}
