use spwn::Spwner;
use spwn::spwn;
use hyper::{Client, Uri};

#[async_std::main]
async fn main() {
    spwn::set_spwner(Spwner::AsyncStd).unwrap();
    spwn(say_hello()).await.unwrap();
}

async fn say_hello() {
    let client = Client::new();
    let resp = client.get(Uri::from_static("http://httpbin.org/ip")).await.unwrap();

    println!("Response: {}", resp.status());
}
