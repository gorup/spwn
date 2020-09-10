use spwn::Spwner;
use spwn::spwn;
use std::io::Error as IoError;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::net::ToSocketAddrs;

#[async_std::test]
async fn basic_async_fn() {
    spwn::set_spwner(Spwner::AsyncStd).unwrap();
    assert_eq!(8, spwn(returns_eight()).await);
}

async fn returns_eight() -> usize {
    8
}

#[async_std::test]
async fn tcp_returns_error() {
    spwn::set_spwner(Spwner::AsyncStd).unwrap();
    let r = spwn(say_hello_tcp_8080()).await;

    assert!(r.is_err());
}

async fn say_hello_tcp_8080() -> Result<(), IoError> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(b"hello world!").await?;
    Ok(())
}

