extern crate env_logger;
extern crate http;
extern crate serde;
extern crate simple_proxy;

use simple_proxy::middlewares::{Health, Logger};
use simple_proxy::{Environment, SimpleProxy};
use oas_middleware::OASMiddleware;

use std::path::PathBuf;
use http::uri::Authority;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "OAS Proxy", about = "A Proxy for OpenAPI validation")]
/// OAS Proxy provides a proxy to validate requests and responses
/// based on the description of an OpenAPI file.
struct Config {
    #[structopt(
        short,
        env = "OAS_BACKEND",
        default_value = "localhost:3000",
        parse(try_from_str)
    )]
    /// The URI where requests will be proxied to.
    backend: Authority,

    #[structopt(short, env = "OAS_PORT", default_value = "5000")]
    /// The port where the proxy is running.
    port: u16,

    #[structopt(
        short,
        env = "OAS_FILENAME",
        default_value = "/tmp/openapi.yaml",
        parse(from_os_str)
    )]
    /// The path to the openapi file describing the API.
    input: PathBuf,
}

fn main() {
    env_logger::init();
    let config = Config::from_args();
    println!("{:?}", config);

    let mut proxy = SimpleProxy::new(config.port, config.backend, Environment::Development);
    let health = Health::new("/health", "OK !");
    let logger = Logger::new();
    let oas_validator = OASMiddleware::new(&config.input);

    // Order matters
    proxy.add_middleware(Box::new(health));
    proxy.add_middleware(Box::new(oas_validator));
    proxy.add_middleware(Box::new(logger));

    // Start proxy
    proxy.run();
}
