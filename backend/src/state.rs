use std::sync::Arc;

use tokio::sync::RwLock;

pub struct State {}

impl State {
  pub fn new() -> Arc<RwLock<Self>> {
    Arc::new(RwLock::new(Self {}))
  }
}