use spwn::Spwner;

mod util_tokio_net;
use util_tokio_net::test_tokio_net;

#[tokio::test]
async fn tcp_echo_success() {
    spwn::set_spwner(Spwner::Tokio).unwrap();

    let port = 12344;
    test_tokio_net(port).await.unwrap();
}
