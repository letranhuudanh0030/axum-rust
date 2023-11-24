// #![allow(unused)] // For early development.

// region:    --- Modules
mod config;
mod ctx;
mod error;
mod model;
mod pwd;
mod rpc;
mod token;
mod utils;
mod web;
// #[cfg(test)] // Commented during early development.
pub mod _dev_utils;

pub use self::error::{Error, Result};

use tracing_subscriber::EnvFilter;
// endRegion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- For dev only
    _dev_utils::init_dev().await;

    // Initialize ModelManager

    Ok(())
}
