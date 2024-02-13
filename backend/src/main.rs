use std::{io, env};

use actix_web::HttpServer;

mod state;
mod socket;

#[tokio::main]
async fn main() -> io::Result<()> {
  env::set_var("RUST_LOG", "INFO");
  pretty_env_logger::init();

  let mut args = env::args();
  let ip = args.nth(1).unwrap_or(String::from("0.0.0.0"));
  let port = args.next().map(|p| p.parse().unwrap_or(2137)).unwrap_or(2137);

  let state = state::State::new();

  log::info!("Starting server on {}:{}", ip, port);
  let server = HttpServer::new(move || {
    actix_web::App::new()
      .service(socket::socket)
      .app_data(state.clone())
  });

  server.bind((ip, port))?.run().await
}
