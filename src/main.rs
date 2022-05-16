#[macro_use]
extern crate actix_web;

use {
    actix_web::{middleware, App, HttpServer},
    std::{env, io},
};

mod miner;
mod miner_controller;
mod util;
mod wallet;
mod wallet_controller;


#[actix_rt::main]
async fn main() -> io::Result<()> {

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await 


}

