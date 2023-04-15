use std::collections::{HashMap, HashSet};

use super::V;

use super::equivalance::Equivalance;
use super::Image;

pub struct Partition {
    pub areas: Vec<HashSet<V>>,
    pub v2area: HashMap<V, usize>,
}

impl Partition {
    pub fn from_eq(img: &Image, eq: &mut impl Equivalance) -> Self {
        let mut visited = HashSet::new();
        let mut areas = Vec::new();
        let mut v2area = HashMap::new();
        for (start, _) in img.iter() {
            if visited.contains(&start) {
                continue;
            }
            let mut queue = vec![start];
            let mut surface = HashSet::from_iter([start].into_iter());
            while let Some(cur) = queue.pop() {
                let next = eq.linked(cur, img, &|v| !surface.contains(&v));
                for next in next {
                    let nn = eq.linked(next, img, &|_| true);
                    if nn.contains(&cur) {
                        surface.insert(next);
                        queue.push(next);
                    }
                }
            }
            visited.extend(surface.iter());
            for &v in &surface {
                v2area.insert(v, areas.len());
                visited.insert(v);
            }
            areas.push(surface);
        }
        Self { areas, v2area }
    }

    pub fn check(&self) {
        for (v, &n) in &self.v2area {
            assert!(self.areas[n].contains(v));
        }
        for (n, area) in self.areas.iter().enumerate() {
            for v in area {
                assert_eq!(self.v2area.get(v), Some(&n));
            }
        }
    }

    pub fn filter_by_size(&self, range: (usize, Option<usize>)) -> Self {
        let (from, to) = (range.0, range.1.unwrap_or(usize::MAX));
        let areas: Vec<_> = self
            .areas
            .iter()
            .enumerate()
            .filter(|(_, area)| from <= area.len() && area.len() < to)
            .map(|(old, _)| self.areas[old].clone())
            .collect();
        let mut v2area = HashMap::new();
        for (n, area) in areas.iter().enumerate() {
            for &v in area {
                v2area.insert(v, n);
            }
        }

        Self { areas, v2area }
    }
}
