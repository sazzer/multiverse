use super::postgres::Postgres;
use lazy_static::lazy_static;
use testcontainers::{clients::Cli, Container, Docker};

lazy_static! {
    static ref DOCKER: Cli = Cli::default();
}

/// Wrapper around a Docker container that runs Postgres for our tests
pub struct TestDatabase {
    #[allow(dead_code)]
    node: Container<'static, Cli, Postgres>,
    pub host: String,
    pub port: u16,
    pub url: String,
}

impl Default for TestDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl TestDatabase {
    #[must_use]
    pub fn new() -> Self {
        log::info!("Starting Postgres database");
        let node = DOCKER.run(Postgres::default());

        let host = std::env::var("DOCKER_HOSTNAME").unwrap_or_else(|_| "localhost".to_owned());
        let port = node.get_host_port(5432).unwrap();
        let url = format!("postgres://postgres@{}:{}", host, port);
        log::info!("Running postgres on {}", url);

        TestDatabase {
            node,
            host,
            port,
            url,
        }
    }
}
