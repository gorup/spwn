use spwn::Spwner;

mod util_asyncstd_net;
use util_asyncstd_net::test_asyncstd_net;

#[tokio::test]
async fn tcp_echo_success() {
    spwn::set_spwner(Spwner::Tokio).unwrap();

    let port = 12341;
    test_asyncstd_net(port).await.unwrap();
}
