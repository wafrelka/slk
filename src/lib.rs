pub mod config;
pub mod slack;

pub use config::{read_config, Config};
pub use slack::{post_message, BasicMessage};
