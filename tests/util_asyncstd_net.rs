use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task::sleep;
use spwn::spwn;
use std::{io::Error as IoError, time::Duration};

pub async fn test_asyncstd_net(port: usize) -> Result<(), IoError> {
    spwn!(server(port));

    sleep(Duration::from_millis(100)).await;

    let r = spwn!(say_hello_tcp(port)).await.unwrap()?;

    assert!(r);
    Ok(())
}

async fn server(port: usize) -> Result<(), IoError> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        spwn!(handle_stream(socket));
    }
}

async fn handle_stream(mut stream: TcpStream) -> Result<(), IoError> {
    let mut buf: [u8; 55] = [0; 55];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break Ok(());
        }
        stream.write_all(&buf[0..n]).await?;
    }
}

async fn say_hello_tcp(port: usize) -> Result<bool, IoError> {
    let writeme: [u8; 512] = [3; 512];

    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;
    stream.write_all(&writeme).await?;

    let mut readme: [u8; 512] = [0; 512];
    stream.read_exact(&mut readme).await?;

    Ok(readme[..].eq(&writeme[..]))
}
