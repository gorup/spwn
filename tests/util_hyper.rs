use async_std::task::sleep;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use spwn::spwn;
use std::{convert::Infallible, time::Duration};

pub async fn hyper_test(port: usize) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    spwn!(server(port));
    sleep(Duration::from_millis(100)).await;

    let client = Client::new();
    let res = client
        .get(format!("http://127.0.0.1:{}/", port).parse().unwrap())
        .await
        .unwrap();
    assert!(res.status().is_success());
    Ok(())
}

pub async fn server(port: usize) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([127, 0, 0, 1], port as u16).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;
    Ok(())
}

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}
