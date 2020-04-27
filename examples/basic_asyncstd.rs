use spwn::Spwner;
use spwn::spwn;

#[async_std::main]
async fn main() {
    spwn::set_spwner(Spwner::AsyncStd).unwrap();
    spwn(say_hello()).await.unwrap();
}

async fn say_hello() {
    println!("Hello, world!");
}
