use super::{Rect, V};

use super::Image;

pub trait Equivalance {
    /// Expects valid position.
    fn linked(
        &mut self,
        with: V,
        img: &Image,
        filter: &dyn Fn(V) -> bool,
    ) -> smallvec::SmallVec<[V; 4]>;

    fn name(&self) -> String;

    // fn graph(&mut self, img: &Image) -> HashMap<V, smallvec::SmallVec<[V; 4]>> {
    //     img.iter()
    //         .map(|(v, _)| (v, self.linked(v, img, &|_| true)))
    //         .collect()
    // }
}

#[derive(derive_new::new, Debug)]
pub struct L1Diff {
    diff: usize,
}

impl Equivalance for L1Diff {
    fn linked(
        &mut self,
        with: V,
        img: &Image,
        filter: &dyn Fn(V) -> bool,
    ) -> smallvec::SmallVec<[V; 4]> {
        with.neigh1f(&|v| {
            img.contains(v) && img[with].diff_l1(img[v]) <= self.diff as u16 && filter(v)
        })
    }

    fn name(&self) -> String {
        format!("l1_diff({})", self.diff)
    }
}

#[derive(derive_new::new, Debug)]
pub struct L1DiffSup {
    range: usize,
    thr: usize,
}

impl Equivalance for L1DiffSup {
    fn linked(
        &mut self,
        with: V,
        img: &Image,
        filter: &dyn Fn(V) -> bool,
    ) -> smallvec::SmallVec<[V; 4]> {
        let window = Rect::new(
            (with.y - self.range, with.x - self.range).into(),
            V::new(0, 0).pad(self.range * 2 + 1),
        );
        let sum: u32 = window.iter().map(|v| img.diff_l1(v) as u32).sum();
        let average = sum / 16 / (self.range * 2 + 1) as u32 / (self.range * 2 + 1) as u32;

        with.neigh1f(&|v| {
            img.contains(v)
                && img.diff_l1_with(with, v) < average as u16 + self.thr as u16
                && filter(v)
        })
    }

    fn name(&self) -> String {
        format!("l1_range({})_thr({})", self.range, self.thr)
    }
}

// pub struct NormDiffSup<Norm> {}
// impl<Norm> NormDiffSup<Norm> { fn f() { Norm::f(); } }
