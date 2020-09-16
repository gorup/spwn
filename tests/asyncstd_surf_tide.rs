use spwn::Spwner;

mod util_surf_tide;
use util_surf_tide::test_surf_tide;

#[async_std::test]
pub async fn surf_tide() {
    spwn::set_spwner(Spwner::AsyncStd).unwrap();

    let port = 12348;

    test_surf_tide(port).await;
}
