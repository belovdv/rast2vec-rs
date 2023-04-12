use std::str::FromStr;

use super::timer::MaybeTimer;

#[derive(Debug, clap::Parser, Clone)]
pub struct Config {
    #[arg(long)]
    pub workspace: std::path::PathBuf,

    #[arg(long)]
    pub dir_source: Option<std::path::PathBuf>,

    #[arg(short = 't', default_value_t = false)]
    pub timers: bool,

    #[arg(long, value_parser = Mode::from_str)]
    pub mode: Mode,
}

impl Config {
    #[must_use]
    pub fn timer<T: ToString>(&self, name: T) -> MaybeTimer {
        MaybeTimer::start(self.timers, name)
    }
}

#[derive(Debug, Clone, Copy, strum_macros::EnumString)]
pub enum Mode {
    #[strum(ascii_case_insensitive)]
    Interactive,
    #[strum(ascii_case_insensitive)]
    Normal,
}
