use svg::node::element::{path::Data, Path};
use svg::Document;

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
            data = data.move_to((pg.frame[0].v.x, pg.frame[0].v.y));
            for p in &pg.frame[1..] {
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
}
