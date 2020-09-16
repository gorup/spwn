use async_std::task::sleep;
use spwn::spwn;
use std::time::Duration;

pub async fn test_surf_tide(port: usize) {
    spwn!(server(port));
    sleep(Duration::from_millis(100)).await;

    let res = surf::get(format!("http://127.0.0.1:{}/", port))
        .await
        .unwrap();
    assert!(res.status().is_success());
}

pub async fn server(port: usize) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.listen(format!("127.0.0.1:{}", port)).await?;
    Ok(())
}
