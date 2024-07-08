use actix_web::{middleware, App, HttpServer};

use std::{env, io};

mod response;
mod models;
mod prices;
mod performance_figures;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(prices::endpoints::get_price_history)
            .service(performance_figures::endpoints::get_daily_returns)
            .service(performance_figures::endpoints::get_ytd_return)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}