use std::collections::HashSet;
use std::path::Path;

use anyhow::Result;
use image::RgbaImage;

use super::partition::Partition;

use super::Color;
use super::Rect;
use super::V;

#[derive(Clone)]
pub struct Image {
    data: Vec<Vec<Color>>,
    view: Rect,
    pad: usize,
}

impl Image {
    /// Note: actual coordinates start not from zero!
    pub fn dim(&self) -> V {
        self.view.size
    }
    pub fn size(&self) -> V {
        self.view.size + (self.pad * 2, self.pad * 2).into()
    }
    pub fn check(&self, v: V) -> Option<V> {
        self.view.check(v)
    }
    pub fn contains(&self, v: V) -> bool {
        self.check(v).is_some()
    }
    pub fn get(&self, v: V) -> Option<Color> {
        self.check(v).map(|v| self.data[v.y][v.x])
    }
    /// Return: `Some(())` on success.
    pub fn set(&mut self, v: V, c: Color) -> Option<()> {
        self.check(v).map(|v| self.data[v.y][v.x] = c)
    }
    /// Set of actual points.
    pub fn view(&self) -> Rect {
        self.view
    }

    pub fn iter(&self) -> impl Iterator<Item = (V, Color)> + '_ {
        self.view.iter().map(|v| (v, self.data[v.y][v.x]))
    }
}

impl std::ops::Index<V> for Image {
    type Output = Color;

    fn index(&self, index: V) -> &Self::Output {
        &self.data[index.y][index.x]
    }
}
impl std::ops::IndexMut<V> for Image {
    fn index_mut(&mut self, index: V) -> &mut Self::Output {
        &mut self.data[index.y][index.x]
    }
}

impl Image {
    pub fn new(size: V, pad: usize) -> Self {
        let data = vec![vec![Default::default(); size.x + pad * 2]; size.y + pad * 2];
        let view = Rect::new((pad, pad).into(), size);
        Self { data, view, pad }
    }

    pub fn load(path: &Path, pad: usize) -> Result<Self> {
        let i = image::open(path)?.into_rgba8();
        let mut r = Self::new((i.height(), i.width()).into(), pad);
        for v in r.view.iter() {
            r.set(v, i[((v.x - pad) as u32, (v.y - pad) as u32)].into());
        }
        Ok(r)
    }
    pub fn store(&self, path: &Path) -> Result<()> {
        let mut i = RgbaImage::new(self.dim().x as u32, self.dim().y as u32);
        for (v, c) in self.iter() {
            i[((v.x - self.pad) as u32, (v.y - self.pad) as u32)] = c.into();
        }
        i.save(path).map_err(|e| e.into())
    }
    pub fn store_p(&self, path: &Path) -> Result<()> {
        let mut i = RgbaImage::new(self.size().x as u32, self.size().y as u32);
        for v in Rect::new0(self.size()).iter() {
            i[(v.x as u32, v.y as u32)] = self[v].into();
        }
        i.save(path).map_err(|e| e.into())
    }
    pub fn crop(&self, from: V, size: V) -> Self {
        let mut r = Self::new(size, 0);
        for v in Rect::new0(size).iter() {
            r[v] = self[v + from];
        }
        r
    }
    pub fn pad(&self, pad: usize) -> Self {
        let mut r = Self::new(self.size(), pad);
        for v in Rect::new0(self.size()).iter() {
            r[v.pad(pad)] = self[v];
        }
        r
    }
}

impl Image {
    pub fn from_part(reference: &Image, partition: &Partition) -> Self {
        let mut r = Self::new(reference.dim(), 0);
        let colors = reference.colors(partition);
        for v in Rect::new0(r.dim()).iter() {
            if let Some(&color_ind) = partition.v2area.get(&v.pad(reference.pad)) {
                r[v] = colors[color_ind];
            }
        }
        r
    }

    pub fn colors(&self, partition: &Partition) -> Vec<Color> {
        partition
            .areas
            .iter()
            .map(|area| self.color(area))
            .collect()
    }

    pub fn color(&self, area: &HashSet<V>) -> Color {
        let sum = area.iter().fold([0; 4], |r, &v| r + self[v]);
        Color::new([
            (sum[0] / area.len()) as u8,
            (sum[1] / area.len()) as u8,
            (sum[2] / area.len()) as u8,
            (sum[3] / area.len()) as u8,
        ])
    }

    /// Expects v to be good.
    pub fn diff_l1_with(&self, v1: V, v2: V) -> u16 {
        self[v1].diff_l1(self[v2])
    }
    /// Expects v to be good.
    /// Returns sum of `diff_l1` with `neigh1`.
    pub fn diff_l1(&self, v: V) -> u16 {
        v.neigh1()
            .into_iter()
            .map(|n| self.diff_l1_with(v, n))
            .sum()
    }

    /// Expects v to be good.
    pub fn diff_li_with(&self, v1: V, v2: V) -> u16 {
        self[v1].diff_li(self[v2])
    }
    /// Expects v to be good.
    /// Returns sum of `diff_l1` with `neigh1`.
    pub fn diff_li(&self, v: V) -> u16 {
        v.neigh1()
            .into_iter()
            .map(|n| self.diff_li_with(v, n))
            .sum()
    }
}
