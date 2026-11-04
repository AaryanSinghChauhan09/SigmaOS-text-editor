//! SigmaOS-text-editor — Text editor component for SigmaOS
#![allow(dead_code)]

pub mod config;
pub mod error;
pub mod service;

pub use config::Config;
pub use error::{ComponentError, Result};
pub use service::Service;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
