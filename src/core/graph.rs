use std::collections::{HashMap, HashSet};

use super::Coord;
use super::{Type1, Type2};
use crate::utility;

#[derive(Debug, Default)]
pub struct Graph {
    edges: HashMap<Coord, HashSet<Coord>>,
    edges_l: HashSet<(Coord, Coord)>,
    markers: HashMap<Coord, [f32; 3]>,
}

impl Graph {
    pub fn insert(&mut self, from: Coord, to: Coord) {
        match from.dist_sq(to) {
            0 => panic!(),
            1 | 2 => {}
            _ => {
                self.edges_l.insert((from, to));
                self.edges_l.insert((to, from));
            }
        }
        self.link_add(from, to)
    }
    fn link_add(&mut self, from: Coord, to: Coord) {
        self.link_add_dir(from, to);
        self.link_add_dir(to, from)
    }
    fn link_add_dir(&mut self, from: Coord, to: Coord) {
        if let Some(v) = self.edges.get_mut(&from) {
            v.insert(to);
        } else {
            self.edges
                .insert(from, HashSet::from_iter([to].into_iter()));
        }
    }
    pub fn erase_v(&mut self, v: Coord) {
        self.edges.remove(&v);
    }

    pub fn degree(&self, c: Coord) -> usize {
        self.edges.get(&c).map(|v| v.len()).unwrap_or(0)
    }

    pub fn mark(&mut self, c: Coord, m: [f32; 3]) {
        self.markers.insert(c, m);
    }

    pub fn log_colorized(
        &self,
        args: &utility::Args,
        name: &str,
        w: usize,
        h: usize,
        colors: Option<&HashMap<Coord, [f32; 3]>>,
    ) {
        args.log(name);
        if args.do_log_images {
            let drawn = utility::Image::edges2dm(w, h, true, &self.edges, colors, &self.markers);
            utility::Image::store_f32(args.log_dir(), drawn, name)
        }
    }

    pub fn erase_short(&mut self, from: Coord, to: Coord) {
        match from.dist_sq(to) {
            1 | 2 => {}
            _ => {
                // To be done: this may take much time.
                eprintln!("{:?} {:?}", from, to);
                let removed_1 = self.edges_l.remove(&(from, to));
                let removed_2 = self.edges_l.remove(&(to, from));
                assert!(removed_1 || removed_2)
            }
        }
        self.edges.get_mut(&from).unwrap().remove(&to);
        self.edges.get_mut(&to).unwrap().remove(&from);
    }

    pub fn contains(&self, c: Coord) -> bool {
        self.edges.get(&c).is_some()
    }
    pub fn neigh(&self, from: Coord) -> impl Iterator<Item = &Coord> {
        self.edges.get(&from).unwrap().iter()
    }

    pub fn bfs(&self, start: Coord, limit: usize) -> (Vec<Vec<Coord>>, HashMap<Coord, usize>) {
        let mut layers = vec![vec![start]];
        let mut dists = HashMap::new();
        dists.insert(start, 0);

        for d in 1..limit + 1 {
            let next: Vec<_> = layers
                .last()
                .unwrap()
                .iter()
                .filter(|&&c| self.contains(c))
                .flat_map(|&c| self.neigh(c))
                .filter_map(|n| match dists.get(n) {
                    Some(_) => None,
                    None => Some(*n),
                })
                .collect();
            layers.push(next);
            for n in layers.last().unwrap() {
                dists.insert(*n, d);
            }
        }

        (layers, dists)
    }

    pub fn connect_ends(
        &self,
        only_ends: bool,
        plain: &super::Plain,
        type_1: &HashMap<Coord, Type1>,
        type_2: &HashMap<Coord, Type2>,
    ) -> Vec<(Coord, Coord)> {
        let mut to_insert = Vec::new();
        for (&from, ty) in type_1 {
            if matches!(ty, Type1::End)
                && !matches!(type_2.get(&from), Some(Type2::Island))
                && self.contains(from)
                && self.neigh(from).count() == 1
            {
                let (_, near) = self.bfs(from, 10);
                let good = |c| {
                    plain.contains(c)
                        && (!only_ends || matches!(type_2.get(&c), Some(Type2::End)))
                        && !near.contains_key(&c)
                        && self.contains(c)
                        && self.neigh(c).next().is_some()
                };
                if let Some(add) = plain.find_nearests(from, 20, &good) {
                    for to in add {
                        to_insert.push((from, to))
                    }
                }
            }
        }
        to_insert
    }

    pub fn edges(&self) -> &HashMap<Coord, HashSet<Coord>> {
        &self.edges
    }
    pub fn edges_l(&self) -> &HashSet<(Coord, Coord)> {
        &self.edges_l
    }
}
