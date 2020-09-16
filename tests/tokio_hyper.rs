use spwn::Spwner;

mod util_hyper;
use util_hyper::hyper_test;

#[tokio::test]
pub async fn hyper_hello() {
    spwn::set_spwner(Spwner::Tokio).unwrap();
    let port = 12345;

    hyper_test(port).await.unwrap();
}
