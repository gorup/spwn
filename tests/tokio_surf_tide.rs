use spwn::Spwner;

mod util_surf_tide;
use util_surf_tide::test_surf_tide;

#[tokio::test]
pub async fn surf_tide() {
    spwn::set_spwner(Spwner::Tokio).unwrap();

    let port = 12348;

    test_surf_tide(port).await;
}
