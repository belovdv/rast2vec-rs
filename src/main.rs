mod core;
mod utility;
pub mod vector;

use std::collections::HashMap;

use clap::Parser;

use self::core::{svg_conv, Type1};

fn main() {
    let args = utility::Args::parse();
    let _timer = utility::Timer::now(args.launch_timers, "all".to_string());

    let input_img = image::open(&args.input).expect("couldn't read input");
    let input_matrix = utility::Image::im2dm(&input_img.into_rgb32f());
    let mut plain = core::Plain::from(&input_matrix[0]);
    plain.log(&args, "input");

    args.log("remove lone");
    let lone = plain.filtered(&|c| plain.neigh(c).is_empty());
    plain.set_all(lone, false);
    plain.log(&args, "remove_lone");

    args.log("continue lines");
    plain.continue_lines(&args);
    plain.log(&args, "continue_lines_simple");

    args.log("remove sticks");
    let sticks = plain.filtered(&|c| {
        let neigh = plain.neigh(c);
        neigh.len() == 1 && plain.neigh(neigh[0]).len() > 3
    });
    plain.set_all(sticks, false);
    plain.log(&args, "remove_sticks");

    args.log("initialize types");
    let mut type_1: HashMap<_, _> = Default::default();
    let mut colors = HashMap::default();
    for &c in plain.all() {
        let ty = plain.determine_type_1(c);
        type_1.insert(c, ty);
        colors.insert(c, ty.into());
    }
    plain.log_colorized(&args, "knots", &colors);

    let mut type_2 = HashMap::new();
    for &c in plain.all() {
        type_2.insert(c, plain.determine_type_2(c, &type_1));
    }

    let mut colors = HashMap::default();
    for &c in plain.all() {
        colors.insert(c, (*type_2.get(&c).unwrap()).into());
    }
    plain.log_colorized(&args, "degrees", &colors);

    let (w, h) = (plain.w(), plain.h());

    args.log("graph init");
    let mut graph = core::Graph::default();
    for (&c, &ty) in &type_1 {
        if let Type1::Path(from, to) = ty {
            if !matches!(type_2.get(&from), Some(core::Type2::Island)) {
                graph.insert(c, from);
                graph.insert(c, to);
            }
        }
    }
    graph.log_colorized(&args, "edges", w, h, Some(&colors));

    args.log("graph update 1");
    for (from, ty) in &type_1 {
        if ty.is_knot() {
            for to in plain.neigh1(*from) {
                if type_1.get(&to).map(|c| c.is_knot()).unwrap_or(false) {
                    graph.insert(*from, to)
                }
            }
        }
    }
    graph.log_colorized(&args, "edges_n1", w, h, Some(&colors));

    args.log("graph update 2");
    let mut to_insert = Vec::new();
    for (&from, ty) in &type_1 {
        if ty.is_knot() && graph.degree(from) < 2 {
            for to in plain.neigh2(from) {
                if type_1.get(&to).map(|c| c.is_knot()).unwrap_or(false) {
                    // if edges.degree(to) <= 2 {
                    to_insert.push((from, to));
                    // }
                }
            }
        }
    }
    for (from, to) in to_insert {
        graph.insert(from, to)
    }
    graph.log_colorized(&args, "edges_n2", w, h, Some(&colors));

    args.log("graph update 3");
    let mut to_erase = Vec::new();
    for (&from, ty) in &type_1 {
        if ty.is_knot() && graph.degree(from) == 1 {
            to_erase.push((from, *graph.neigh(from).next().unwrap()))
        }
    }
    for (from, to) in to_erase {
        graph.erase_short(from, to)
    }
    graph.log_colorized(&args, "edges_n3", w, h, Some(&colors));

    args.log("connect ends 1");
    let to_insert = graph.connect_ends(true, &plain, &type_1, &type_2);
    for (from, to) in to_insert {
        graph.insert(from, to)
    }
    graph.log_colorized(&args, "connect_ends_only", w, h, Some(&colors));

    args.log("connect ends 2");
    let to_insert = graph.connect_ends(false, &plain, &type_1, &type_2);
    for (from, to) in to_insert {
        graph.insert(from, to)
    }
    graph.log_colorized(&args, "connect_ends", w, h, Some(&colors));

    // Probably, this draws edges from input image.
    args.log("net");
    graph.log_colorized(&args, "net", w, h, None);

    if args.do_log_images {
        let svg_1 = svg_conv::from_edges(w, h, graph.edges());
        let file = utility::Image::gen_path(args.log_dir(), "svg_1", "svg");
        svg::write(file, &svg_1).unwrap();
    }

    args.log("frame");
    for x in 1..w {
        graph.insert((0, x - 1).into(), (0, x).into());
        graph.insert((h - 1, x - 1).into(), (h - 1, x).into());
    }
    for y in 1..h {
        graph.insert((y - 1, 0).into(), (y, 0).into());
        graph.insert((y - 1, w - 1).into(), (y, w - 1).into());
    }

    args.log("areas");
    let timer_areas = utility::Timer::now(args.launch_timers, "find areas".to_string());
    let areas = graph.areas(w, h);
    let areas_amount = areas.len();
    dbg!(areas_amount);
    drop(timer_areas);

    args.log("test inner");
    let mut colors = HashMap::new();
    let areas = utility::Image::average_colors(&args.origin, areas);
    for (area, _, color) in &areas {
        if let Some(color) = color {
            for &i in area {
                colors.insert(i, *color);
            }
        }
    }
    graph.log_colorized(&args, "test_inner", w, h, Some(&colors));

    graph.remove_sticks();
    graph.log_colorized(&args, "remove_sticks", w, h, Some(&colors));

    let mut areas_map = HashMap::new();
    for (area, _, _) in &areas {
        for &c in area {
            areas_map.insert(c, area);
        }
    }
    let parts = graph.areas2polygons(w, h, &areas_map, &colors);
    let svg_2 = svg_conv::from_parts(w, h, parts);
    if args.do_log_images {
        let file = utility::Image::gen_path(args.log_dir(), "svg_2", "svg");
        svg::write(file, &svg_2).unwrap();
    }

    graph.log_colorized(&args, "mark_areas2polygons", w, h, Some(&colors));
}
