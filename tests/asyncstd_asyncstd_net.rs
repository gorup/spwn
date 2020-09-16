use spwn::Spwner;

mod util_asyncstd_net;
use util_asyncstd_net::test_asyncstd_net;

#[async_std::test]
async fn tcp_echo_success() {
    spwn::set_spwner(Spwner::AsyncStd).unwrap();

    let port = 12341;
    test_asyncstd_net(port).await.unwrap();
}
