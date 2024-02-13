use std::{io, env};

#[tokio::main]
async fn main() -> io::Result<()> {
  env::set_var("RUST_LOG", "INFO");
  pretty_env_logger::init();

  log::info!("Hello, world!");
  Ok(())
}
