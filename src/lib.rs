#![feature(async_await)]
pub use async_trait::async_trait;

// API
// {API_ENDPOINT}/:id <-task id
// 1. req task [POST]
// 2. task [GET}
// 3. task [DELETE]
// 4.

#[async_trait]
pub trait Worker {
    async fn get(&mut self, task_id: &str, body_json: serde_json::Value);
    async fn post(&mut self, task_id: &str, body_json: serde_json::Value);
    async fn put(&mut self, task_id: &str, body_json: serde_json::Value);
    async fn delete(&mut self, task_id: &str, body_json: serde_json::Value);
}

#[derive(Debug, Clone)]
pub enum Error {}

pub struct Config {
    host: String,
    port: i16,
    endpoint: String,
}

impl Config {
    fn new(host: &str, port: i16) -> Self {
        Self {
            host: host.into(),
            port,
            endpoint: "/".to_string(),
        }
    }

    fn set_endpoint(&mut self, ep: &str) {
        self.endpoint = ep.into();
    }
}

pub struct ConfigBuilder {
    _config: Config,
}

impl ConfigBuilder {
    pub fn create_with(host: &str, port: i16) -> Self {
        Self {
            _config: Config::new(host, port),
        }
    }

    pub fn endpoint(mut self, ep: &str) -> Self {
        self._config.set_endpoint(ep);
        self
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
        // let ret = self.worker.run();
        // ret.await;

        Ok(())
    }
}
