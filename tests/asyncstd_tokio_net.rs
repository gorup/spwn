use spwn::Spwner;

mod util_tokio_net;
use util_tokio_net::test_tokio_net;

#[async_std::test]
async fn tcp_echo_success() {
    spwn::set_spwner(Spwner::AsyncStd).unwrap();

    let port = 12342;
    test_tokio_net(port).await.unwrap();
}
