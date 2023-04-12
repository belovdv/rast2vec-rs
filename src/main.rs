#![feature(ptr_metadata)]

mod cli;
mod rast2vec;

use self::rast2vec::partition::Partition;
use self::rast2vec::utility;
use self::rast2vec::utility::Color;
use self::rast2vec::utility::Image5;
use self::rast2vec::View;

use clap::Parser as _;

fn pipeline(config: cli::Config) {
    let name = "clean";

    let name = format!("{name}.png");
    let path = config.dir_source.unwrap().join(name);
    let img = image::open(path).unwrap().into_rgba8();
    // Upscale 2 - hack for simple work with knots.
    let nd = utility::image2ndarray(&img, 2);

    // let nd = utility::ndarray_subview(&nd, 100, 430, 300, 100);
    let img = utility::ndarray2image(&nd);
    img.save(config.workspace.join("src.png")).unwrap();

    let partition = Partition::new_spread_equal_neigh1(&nd, 40);
    partition.check();

    let mut img = Image5::new(nd.h(), nd.w());
    for area in partition.areas() {
        let pos = *area.surface.iter().next().unwrap();
        let color = View::get(&nd, pos);
        img.draw_area(&area.surface, color);
    }
    let knots = partition
        .areas()
        .iter()
        .map(|area| area.outline.clone())
        .collect();
    img.draw_knots_paths(&knots, Color::dark_red());
    eprintln!("store");
    img.store(&config.workspace.join("out.png")).unwrap();
}

fn main() {
    let config = cli::Config::parse();
    let timer_all = config.timer("all");

    match config.mode {
        cli::Mode::Interactive => cli::CLI::new(config).start(),
        cli::Mode::Normal => pipeline(config),
    }

    timer_all.stop_out()
}
