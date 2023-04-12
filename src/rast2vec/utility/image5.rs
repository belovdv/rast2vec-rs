use std::{collections::HashSet, path::Path};

use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::drawing::draw_line_segment_mut;

use crate::rast2vec::Pos;

use super::Color;

fn scale() -> usize {
    3
}

pub struct Image5 {
    data: RgbaImage,
}

impl Image5 {
    pub fn new(h: usize, w: usize) -> Self {
        let data = RgbaImage::new(w as u32 * scale() as u32 + 1, h as u32 * scale() as u32 + 1);
        Self { data }
    }

    /// Safety: assumes area is in provided size.
    pub fn draw_area<P: Copy + Into<Pos>>(&mut self, area: &HashSet<P>, color: Color) {
        for &p in area {
            let pos: Pos = p.into();
            let (y, x) = unsafe { pos.get() };
            let (y, x) = (y * scale(), x * scale());
            for dy in 0..scale() {
                for dx in 0..scale() {
                    let x = x as u32 + dx as u32;
                    let y = y as u32 + dy as u32;
                    self.data[(x, y)] = Rgba(color.rgba);
                }
            }
        }
    }

    pub fn draw_knots_paths(&mut self, knots: &Vec<Vec<Pos>>, color: Color) {
        let mut di = DynamicImage::ImageRgba8(self.data.clone());
        for knots in knots {
            let mut knots = knots.clone();
            knots.push(knots[0]);
            for pair in knots.windows(2) {
                let (fy, fx) = unsafe { pair[0].get() };
                let (ty, tx) = unsafe { pair[1].get() };
                let (fy, fx) = (fy as f32 * scale() as f32, fx as f32 * scale() as f32);
                let (ty, tx) = (ty as f32 * scale() as f32, tx as f32 * scale() as f32);
                draw_line_segment_mut(&mut di, (fx, fy), (tx, ty), Rgba(color.rgba));
            }
        }
        self.data = di.into_rgba8();
    }

    pub fn store(&self, path: &Path) -> Result<(), image::ImageError> {
        self.data.save(path)
    }
}
