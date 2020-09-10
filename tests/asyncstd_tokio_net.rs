use spwn::Spwner;
use spwn::spwn;
use std::io::Error as IoError;
use tokio::net::TcpStream;
use tokio::prelude::*;

#[async_std::test]
async fn tcp_returns_error() {
    spwn::set_spwner(Spwner::AsyncStd).unwrap();
    let r = spwn(say_hello_tcp_8080()).await.unwrap();

    assert!(r.is_err());
}

async fn say_hello_tcp_8080() -> Result<(), IoError> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(b"hello world!").await?;
    Ok(())
}
