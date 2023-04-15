use std::path::PathBuf;

use crate::config::Config;

use crate::core::equivalance::*;
use crate::core::svg::SVG;
use crate::core::Polygon;
use crate::core::{Image, Partition};

#[derive(derive_new::new)]
pub struct Pipeline {
    config: Config,
    #[new(default)]
    counter: usize,
}

impl Pipeline {
    pub fn run(&mut self, name: &str) {
        let name = format!("{name}.png");
        let img = self.load(&name);

        // let img = img.crop(V::new(0, 500), V::new(50, 50));
        let img = img.pad(10);
        img.store(&self.path_w(&name)).unwrap();

        // let pn_huge = self.try_part(&img, &mut L1DiffSup::new(4, 10), (250, None));
        let pn_large = self.try_part(&img, &mut L1DiffSup::new(4, 10), (18000, None));
        let pn_medium = self.try_part(&img, &mut L1DiffSup::new(2, 20), (500, Some(18000)));
        let pn_small = self.try_part(&img, &mut L1DiffSup::new(2, 25), (5, Some(500)));

        let pg_large = Polygon::from_partition(&pn_large, &img);
        let pg_medium = Polygon::from_partition(&pn_medium, &img);
        let pg_small = Polygon::from_partition(&pn_small, &img);

        let mut r = SVG::new(img.size());
        r.extend_with_polygons(&pg_large);
        let mut out = std::fs::File::create(&self.path_w("r1.svg")).unwrap();
        svg::write(&mut out, &r.doc).unwrap();
        r.extend_with_polygons(&pg_medium);
        let mut out = std::fs::File::create(&self.path_w("r2.svg")).unwrap();
        svg::write(&mut out, &r.doc).unwrap();
        r.extend_with_polygons(&pg_small);
        let mut out = std::fs::File::create(&self.path_w("r3.svg")).unwrap();
        svg::write(&mut out, &r.doc).unwrap();
    }
}

impl Pipeline {
    fn try_part(
        &mut self,
        img: &Image,
        eq: &mut impl Equivalance,
        range: (usize, Option<usize>),
    ) -> Partition {
        let name_f = match range.1 {
            Some(m) => format!("f({},{})", range.0, m),
            None => format!("f({})", range.0),
        };
        self.counter += 1;
        let name = format!("{}_part_eq_{}_{}.png", self.counter, eq.name(), name_f);

        let path = self.path_w(&name);
        let t = self.config.timer(&name);
        let pn = Partition::from_eq(img, eq);
        t.stop();

        let t = self.config.timer("filter_by_size");
        let pn = pn.filter_by_size(range);
        t.stop();

        let t = self.config.timer("from_part");
        let r = Image::from_part(img, &pn);
        t.stop();

        let t = self.config.timer("store");
        r.store(&path).unwrap();
        t.stop();
        pn
    }

    fn load(&self, name: &str) -> Image {
        Image::load(&self.config.dir_source.as_ref().unwrap().join(name), 0).unwrap()
    }

    fn path_w(&self, name: &str) -> PathBuf {
        self.config.workspace.join(name)
    }
}
