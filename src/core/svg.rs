use std::collections::{HashMap, HashSet};

use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

use super::Coord;

pub fn from_edges(w: usize, h: usize, edges: &HashMap<Coord, HashSet<Coord>>) -> Document {
    let mut document = Document::new();
    let back = Data::new()
        .move_to((0, 0))
        .line_to((w, 0))
        .line_to((w, h))
        .line_to((0, h))
        .line_to((0, 0))
        .close();
    let back = Path::new()
        .set("fill", "black")
        .set("stroke", "none")
        .set("d", back);
    document = document.add(back);
    for (from, tos) in edges {
        for to in tos {
            let data = Data::new()
                .move_to((from.y, from.x))
                .line_to((to.y, to.x))
                .close();
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "white")
                .set("stroke-width", 0.5)
                .set("d", data);

            document = document.add(path);
        }
    }
    document = document.set("viewBox", (0, 0, w, h));
    document
}

pub fn from_parts(w: usize, h: usize, areas: Vec<(Vec<Coord>, [f32; 3])>) -> Document {
    let mut document = Document::new();
    document = document.set("viewBox", (0, 0, w, h));
    for (path, color) in areas {
        let r = (255. * color[0]) as u8;
        let g = (255. * color[1]) as u8;
        let b = (255. * color[2]) as u8;
        let color = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let mut data = Data::new();
        data = data.move_to((path[0].y, path[0].x));
        for p in &path[1..] {
            data = data.line_to((p.y as f32 + 0.5, p.x as f32 + 0.5));
        }
        data = data.close();
        let part = Path::new()
            .set("fill", color)
            .set("stroke", "none")
            .set("d", data);
        document = document.add(part);
    }
    document
}
