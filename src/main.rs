mod config;
mod pipeline;

mod core;
mod utility;

use clap::Parser as _;

fn main() {
    let config = config::Config::parse();
    let timer_all = config.timer("all");

    let mut pipeline = pipeline::Pipeline::new(config);
    pipeline.run("emodji");

    timer_all.stop()
}
