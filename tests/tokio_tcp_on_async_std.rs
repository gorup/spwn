use spwn::Spwner;
use spwn::spwn;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::net::ToSocketAddrs;

#[async_std::main]
async fn main() {
    spwn::set_spwner(Spwner::AsyncStd).unwrap();
    spwn(say_hello()).await.unwrap();
}

async fn say_hello() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    stream.write_all(b"hello world!").await.unwrap();

    println!("Writing complete");
}
