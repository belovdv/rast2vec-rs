use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Index;

use crate::utility;
use crate::vector;

use super::local;
use super::Coord;

#[derive(Debug, Default)]
pub struct Plain {
    points_vec: Vec<Vec<bool>>,
    points_set: HashSet<Coord>,
}

impl Plain {
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            points_vec: vec![vec![false; w]; h],
            points_set: Default::default(),
        }
    }

    pub fn new_test_vsp(points_str: &Vec<&'static str>) -> Self {
        Self::new_test(
            points_str
                .iter()
                .map(|s| s.chars().map(|c| c != ' ').collect())
                .collect(),
        )
    }
    pub fn new_test(points_vec: Vec<Vec<bool>>) -> Self {
        let mut points_set = HashSet::new();
        (0..points_vec.len())
            .flat_map(|y| (0..points_vec[0].len()).map(move |x| (y, x)))
            .filter(|&(y, x)| points_vec[y][x])
            .for_each(|c| {
                points_set.insert(c.into());
            });
        Self {
            points_vec,
            points_set,
        }
    }

    pub fn insert(&mut self, c: Coord) -> bool {
        if c == (27, 136).into() {
            dbg!(c);
        }
        self.set(c, true)
    }
    pub fn erase(&mut self, c: Coord) -> bool {
        self.set(c, false)
    }
    pub fn set_all(&mut self, c: Vec<Coord>, will_be: bool) {
        for c in c {
            self.set(c, will_be);
        }
    }
    pub fn set(&mut self, c: Coord, will_be: bool) -> bool {
        self.points_vec[c.y as usize][c.x as usize] = will_be;
        match will_be {
            true => self.points_set.insert(c),
            false => self.points_set.remove(&c),
        }
    }

    pub fn neigh1(&self, c: Coord) -> Vec<Coord> {
        local::get_dist(c, 1, &|c| self.contains(c))
    }
    pub fn neigh2(&self, c: Coord) -> Vec<Coord> {
        local::get_dist(c, 2, &|c| self.contains(c))
    }
    pub fn neigh(&self, c: Coord) -> Vec<Coord> {
        let mut result = self.neigh1(c);
        result.extend(self.neigh2(c).into_iter());
        result
    }

    pub fn get<T>(&self, coord: T) -> Option<bool>
    where
        Coord: From<T>,
    {
        let c = Coord::from(coord);
        match (c.y, c.x) {
            (y, _) if y < 0 || y >= self.h() as isize => None,
            (_, x) if x < 0 || x >= self.w() as isize => None,
            (y, x) => Some(self.points_vec[y as usize][x as usize]),
        }
    }
    pub fn contains<T>(&self, coord: T) -> bool
    where
        Coord: From<T>,
    {
        self.get(coord).unwrap_or(false)
    }

    pub fn log(&self, args: &utility::Args, name: &str) {
        args.log_matrix(name, &self.into())
    }
    pub fn log_colorized(
        &self,
        args: &utility::Args,
        name: &str,
        colors: &HashMap<Coord, [f32; 3]>,
    ) {
        args.log_matrix_colorized(name, &self.into(), colors)
    }

    pub fn w(&self) -> usize {
        self.points_vec[0].len()
    }
    pub fn h(&self) -> usize {
        self.points_vec.len()
    }

    pub fn all(&self) -> impl Iterator<Item = &Coord> {
        self.points_set.iter()
    }
    pub fn filtered(&self, filter: &dyn Fn(Coord) -> bool) -> Vec<Coord> {
        self.points_set
            .iter()
            .map(|&c| c)
            .filter(|&c| filter(c))
            .collect()
    }
}

impl<T> Index<T> for Plain
where
    Coord: From<T>,
{
    type Output = bool;

    fn index(&self, index: T) -> &Self::Output {
        let c = Coord::from(index);
        &self.points_vec[c.y as usize][c.x as usize]
    }
}

impl From<&vector::Matrix> for Plain {
    fn from(value: &vector::Matrix) -> Self {
        let (w, h) = (value.w(), value.h());
        let mut res = Plain::new(value.w(), value.h());
        (0..h)
            .flat_map(|y| (0..w).map(move |x| (y, x)))
            .filter(|&(y, x)| value[(y, x)] > 0.9)
            .for_each(|c| {
                res.insert(c.into());
            });
        res
    }
}

// Upscale x2.
impl Into<vector::Matrix> for &Plain {
    fn into(self) -> vector::Matrix {
        let (w, h) = (self.w(), self.h());
        let mut res = vector::Matrix::new_black(2 * w, 2 * h);
        (0..h)
            .flat_map(|y| (0..w).map(move |x| (y, x)))
            .filter(|c| self[c])
            .for_each(|(y, x)| {
                let yy = 2 * y;
                let xx = 2 * x;
                for c in [(yy, xx), (yy + 1, xx), (yy, xx + 1), (yy + 1, xx + 1)] {
                    match (y, x) {
                        (27, 141) => res[c] = 1.,
                        (23, 101) => res[c] = 1.,
                        (177, 169) => res[c] = 1.,
                        _ => res[c] = 0.6, // 0.6
                    }
                }
            });
        res
    }
}
