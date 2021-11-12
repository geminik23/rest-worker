#![feature(async_await)]
pub use async_trait::async_trait;

#[async_trait]
pub trait Worker {
    async fn run(&mut self);
}

#[derive(Debug, Clone)]
pub enum Error {}
pub struct Config {}
impl Config {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct App<T>
where
    T: Worker,
{
    config: Config,
    worker: T,
}

impl<T: Worker> App<T> {
    pub fn new(config: Config, worker: T) -> Self {
        Self { config, worker }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let ret = self.worker.run();
        ret.await;

        Ok(())
    }
}
