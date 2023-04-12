mod cli;
mod commands;
mod config;
mod timer;

pub use cli::CLI;
pub use config::{Config, Mode};

use commands::{Manager, Status};
