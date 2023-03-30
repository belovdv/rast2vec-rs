#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::path::{Path, PathBuf};

use image::{DynamicImage, ImageOutputFormat, Rgb, Rgb32FImage, RgbImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;

use crate::core::Coord;

use super::vector;

use vector::{DMatrix, Matrix};

#[derive(Debug, clap::Parser, Clone)]
pub struct Args {
    #[arg(long)]
    pub input: std::path::PathBuf,
    #[arg(long)]
    pub origin: std::path::PathBuf,
    #[arg(long)]
    pub intermediate: Option<std::path::PathBuf>,
    #[arg(long)]
    pub output: Option<std::path::PathBuf>,
    #[arg(long)]
    pub log_directory: Option<std::path::PathBuf>,
    #[arg(short = 'i', default_value_t = false)]
    pub do_log_images: bool,
    #[arg(short = 't', default_value_t = false)]
    pub do_log_text: bool,
    #[arg(short = 'T', default_value_t = false)]
    pub launch_timers: bool,
}
impl Args {
    pub fn log(&self, message: &str) {
        if self.do_log_text {
            eprintln!("LOG: {}", message)
        }
    }
    pub fn log_img(&self, name: &str, img: &DMatrix<3>) {
        if self.do_log_images {
            Image::store_f32(self.log_dir(), Image::dm2im(img), name)
        }
    }
    pub fn log_matrix(&self, name: &str, img: &Matrix) {
        if self.do_log_images {
            let mut img = Image::dm2im(&DMatrix::<3>::new_copies(img));
            Image::store_f32(&self.log_dir(), img, name)
        }
    }
    pub fn log_matrix_colorized(
        &self,
        name: &str,
        img: &Matrix,
        colors: &HashMap<Coord, [f32; 3]>,
    ) {
        if self.do_log_images {
            let mut dm = DMatrix::<3>::new_copies(img);
            for y in 0..img.h() {
                for x in 0..img.w() {
                    // To be done: handle upscale adequately.
                    let c = match (img[(y, x)], colors.get(&(y / 2, x / 2).into())) {
                        (i, _) if i < 0.5 => [0., 0., 0.],
                        (_, Some(c)) => *c,
                        _ => [0.25, 0.25, 0.25],
                    };
                    dm[0][(y, x)] = c[0];
                    dm[1][(y, x)] = c[1];
                    dm[2][(y, x)] = c[2];
                }
            }
            let img = Image::dm2im(&dm);
            Image::store_f32(&self.log_dir(), img, name)
        }
    }

    pub fn log_dir(&self) -> &Path {
        self.log_directory.as_ref().expect("expected log dir")
    }
}

pub struct Image;
static mut COUNTER: u32 = 0;
impl Image {
    pub fn im2dm(image: &Rgb32FImage) -> DMatrix<3> {
        let (w, h) = (image.width() as usize, image.height() as usize);
        let raw = image.as_raw();
        let raw: Vec<_> = raw.chunks_exact(3).collect();
        let c0 = raw.iter().map(|s| s[0]).collect();
        let c1 = raw.iter().map(|s| s[1]).collect();
        let c2 = raw.iter().map(|s| s[2]).collect();
        DMatrix::new([
            Matrix::new(w, h, c0),
            Matrix::new(w, h, c1),
            Matrix::new(w, h, c2),
        ])
    }

    pub fn dm2im(dm: &DMatrix<3>) -> Rgb32FImage {
        let raw_dm = dm.as_raw();

        let raw: Vec<_> = (0..raw_dm[0].buffer_len())
            .map(|ind| [raw_dm[0][ind], raw_dm[1][ind], raw_dm[2][ind]].into_iter())
            .flatten()
            .collect();

        debug_assert_eq!(raw.len(), raw_dm[0].buffer_len() * 3);

        Rgb32FImage::from_raw(raw_dm[0].w() as u32, raw_dm[0].h() as u32, raw).unwrap()
    }

    pub fn store_f32(dir: &Path, image: Rgb32FImage, stage: &str) {
        let image = DynamicImage::ImageRgb32F(image).into_rgba8();
        Image::store(dir, &image, stage);
    }
    pub fn gen_path(dir: &Path, stage: &str, ext: &str) -> File {
        let fname = unsafe { format!("core_{}_{}.{}", COUNTER, stage, ext) };
        unsafe { std::fs::File::create(dir.join(fname)) }.unwrap()
    }
    pub fn store(dir: &Path, image: &RgbaImage, stage: &str) {
        // let fname = unsafe { format!("core_{}_{}.png", COUNTER, stage) };
        // let out = unsafe { std::fs::File::create(dir.join(fname)) };
        let mut out = Self::gen_path(dir, stage, "png");
        unsafe { COUNTER += 1 };
        image.write_to(&mut out, ImageOutputFormat::Png).unwrap();
    }

    fn dm2img_<const D: usize>(dm: &DMatrix<D>) -> Rgb32FImage {
        let w = dm.as_raw()[0].w();
        let h = dm.as_raw()[0].h();
        let w_h = 2;
        let w_w = (D + w_h - 1) / w_h;

        let mut image = Rgb32FImage::new((w * w_w) as u32, (h * w_h) as u32);
        for x in 0..w {
            for y in 0..h {
                for z in 0..D {
                    let o_x = ((z % w_w) * w) as u32;
                    let o_y = ((z / w_w) * h) as u32;
                    let b = dm.as_raw()[z][(x, y)];
                    let color = image::Rgb::<f32>([b, b, b]);
                    image[(o_x + x as u32, o_y + y as u32)] = color;
                }
            }
        }
        image
    }

    pub fn edges2dm(
        w: usize,
        h: usize,
        scale_img: bool,
        edges: &HashMap<Coord, HashSet<Coord>>,
        colors: Option<&HashMap<Coord, [f32; 3]>>,
        markers: &HashMap<Coord, [f32; 3]>,
    ) -> Rgb32FImage {
        let scale: isize = match scale_img {
            true => 5,
            false => 1,
        };
        let pad = match scale_img {
            true => 2.,
            false => 0.,
        };
        let w = w * scale as usize;
        let h = h * scale as usize;
        let mut img = DynamicImage::new_rgb8(w as u32, h as u32);

        if let Some(colors) = colors {
            for x in 0..w {
                for y in 0..h {
                    if let Some(color) =
                        colors.get(&(x / scale as usize, y / scale as usize).into())
                    {
                        let start = (x as f32, y as f32);
                        let end = start;
                        let r = (255. * color[0]) as u8;
                        let g = (255. * color[1]) as u8;
                        let b = (255. * color[2]) as u8;
                        let color = Rgba([r, g, b, 25]);
                        draw_line_segment_mut(&mut img, start, end, color)
                    }
                }
            }
        }

        for (c, m) in markers {
            let (y, x) = ((c.y * scale) as f32, (c.x * scale) as f32);
            let r = (255. * m[0]) as u8;
            let g = (255. * m[1]) as u8;
            let b = (255. * m[2]) as u8;
            for dx in [0., 1., 2., 3., 4.] {
                for dy in [0., 1., 2., 3., 4.] {
                    if dx == dy || dx + dy == 4. || dy + dx == 2. || dy + dx == 6. {
                        continue;
                    }
                    let a = (y + dy, x + dx);
                    draw_line_segment_mut(&mut img, a, a, Rgba([r, g, b, 200]))
                }
            }
        }

        for (&from, tos) in edges {
            for &to in tos {
                let from = ((from.y * scale) as f32 + pad, (from.x * scale) as f32 + pad);
                let to = ((to.y * scale) as f32 + pad, (to.x * scale) as f32 + pad);

                draw_line_segment_mut(&mut img, from, to, Rgba([100, 100, 100, 255]));
            }
        }
        img.into_rgb32f()
    }

    pub fn average_colors(
        origin: &Path,
        areas: Vec<(HashSet<Coord>, HashSet<Coord>)>,
    ) -> Vec<(HashSet<Coord>, HashSet<Coord>, Option<[f32; 3]>)> {
        let origin_img = image::open(origin).expect("couldn't read origin");
        let origin_matrix = Image::im2dm(&origin_img.into_rgb32f());

        areas
            .into_iter()
            .map(|area| {
                let mut color = [0., 0., 0.];
                let (inner, border) = area;
                for i in &inner {
                    color[0] += origin_matrix[0][(i.y as usize, i.x as usize)];
                    color[1] += origin_matrix[1][(i.y as usize, i.x as usize)];
                    color[2] += origin_matrix[2][(i.y as usize, i.x as usize)];
                }
                color[0] /= inner.len() as f32;
                color[1] /= inner.len() as f32;
                color[2] /= inner.len() as f32;
                let mut bad_count = 0;
                for i in &inner {
                    let d0 = (color[0] - origin_matrix[0][(i.y as usize, i.x as usize)]).abs();
                    let d1 = (color[1] - origin_matrix[1][(i.y as usize, i.x as usize)]).abs();
                    let d2 = (color[2] - origin_matrix[2][(i.y as usize, i.x as usize)]).abs();
                    if d0 > 0.3 {
                        bad_count += 1
                    }
                    if d1 > 0.3 {
                        bad_count += 1
                    }
                    if d2 > 0.3 {
                        bad_count += 1
                    }
                }
                match bad_count {
                    _ => (inner, border, Some(color)),
                }
            })
            .collect()
    }
}

pub struct Timer {
    stage_name: String,
    start: std::time::Instant,
}
impl Timer {
    pub fn now<T: ToString>(launch: bool, stage_name: T) -> Option<Self> {
        match launch {
            true => Some(Self {
                stage_name: stage_name.to_string(),
                start: std::time::Instant::now(),
            }),
            false => None,
        }
    }
}
impl Drop for Timer {
    fn drop(&mut self) {
        let Timer { stage_name, start } = self;
        let duration = start.elapsed();
        eprintln!("Stage {:10} taken {:?}", stage_name, duration);
    }
}

#[derive(Default)]
pub struct DSU<T: Hash + Copy + Eq> {
    parents_: HashMap<T, T>,
    priority_: HashMap<T, usize>,
}
impl<T: Hash + Copy + Eq> DSU<T> {
    pub fn insert(&mut self, val: T) {
        self.parents_.insert(val, val);
        self.priority_.insert(val, 0);
    }
    pub fn root(&mut self, val: T) -> T {
        match self.parents_.get(&val).unwrap() {
            p if *p == val => *p,
            p => {
                let r = self.root(*p);
                *self.parents_.get_mut(&val).unwrap() = r;
                r
            }
        }
    }
    pub fn same(&mut self, a: T, b: T) -> bool {
        self.root(a) == self.root(b)
    }
    pub fn unite(&mut self, a: T, b: T) {
        let mut a = self.root(a);
        let mut b = self.root(b);
        if self.priority_[&a] < self.priority_[&b] {
            (a, b) = (b, a)
        }
        *self.parents_.get_mut(&b).unwrap() = a;
    }
}
