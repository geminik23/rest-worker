// #![feature(async_await)]
#[macro_use]
extern crate log;
use async_std::prelude::*;
use rest_worker::{async_trait, App, Config, Error, Worker};

//====
#[derive(Debug)]
struct W {}

#[async_trait]
impl Worker for W {
    async fn run(&mut self) {
        info!("Hello");
    }
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Testing");
    let worker = W {};

    let config = Config::new();
    let mut app = App::new(config, worker);
    app.run().await
}
