use svg::node::element::{path::Data, Path};
use svg::Document;

use super::polygon::Knot;
use super::{Polygon, V};

pub struct SVG {
    pub doc: Document,
}

impl SVG {
    pub fn new(size: V) -> Self {
        let doc = Document::new();
        let doc = doc.set("viewBox", (0, 0, size.x, size.y));
        SVG { doc }
    }

    pub fn extend_with_polygons(&mut self, pgs: &Vec<Polygon>) {
        let mut doc = self.doc.clone();
        for pg in pgs {
            let color = pg.color.format_svg();
            let mut data = Data::new();
            let path = Self::simplify_path(&pg.frame);
            data = data.move_to((path[0].v.x, path[0].v.y));
            for p in &path[1..] {
                data = data.line_to((p.v.x, p.v.y));
            }
            data = data.close();
            let part = Path::new()
                .set("fill", color)
                .set("stroke", "none")
                .set("d", data);
            doc = doc.add(part);
        }
        self.doc = doc
    }

    pub fn simplify_path(path: &Vec<Knot>) -> Vec<Knot> {
        let mut good = vec![true; path.len()];

        // Simplify straight lines.
        for i in 2..good.len() {
            let (a, b, c) = (path[i - 2], path[i - 1], path[i - 0]);
            let dx1 = b.v.x as isize - a.v.x as isize;
            let dy1 = b.v.y as isize - a.v.y as isize;
            let dx2 = c.v.x as isize - b.v.x as isize;
            let dy2 = c.v.y as isize - b.v.y as isize;
            if dx1 == dx2 && dy1 == dy2 {
                good[i - 1] = false;
            }
        }

        (0..good.len())
            .filter(|&n| good[n])
            .map(|n| path[n])
            .collect()
    }
}
