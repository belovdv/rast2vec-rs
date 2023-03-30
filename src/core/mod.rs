mod coord;
mod graph;
mod local;
mod plain;
pub mod svg;

use std::collections::{HashMap, HashSet};

pub use coord::Coord;
pub use graph::Graph;
pub use plain::Plain;

pub use self::svg as svg_conv;

use crate::utility::Args;

pub type Area = HashSet<Coord>;

impl Plain {
    /// Dists: straight 1, diagonal 2. Max dist is `dist`.
    pub fn bfs(&self, start: Coord, limit: usize) -> (Vec<Vec<Coord>>, HashMap<Coord, usize>) {
        self.bfs_cond(start, limit, &|c| self.contains(c))
    }
    /// Dists: straight 1, diagonal 2. Max dist is `dist`.
    pub fn bfs_cond(
        &self,
        start: Coord,
        limit: usize,
        filter: &dyn Fn(Coord) -> bool,
    ) -> (Vec<Vec<Coord>>, HashMap<Coord, usize>) {
        assert!(self[start]);
        let mut layers = Vec::new();
        let mut dists = HashMap::new();
        layers.push(vec![start]);
        dists.insert(start, 0);

        let mut prev = Vec::new();
        let mut curr = vec![start];
        for d in 1..limit + 1 {
            let check = |c| !dists.contains_key(&c) && filter(c);
            let next: HashSet<_> = []
                .into_iter()
                .chain(
                    curr.iter()
                        .flat_map(|&from| local::get_dist(from, 1, &check)),
                )
                .chain(
                    prev.iter()
                        .flat_map(|&from| local::get_dist(from, 2, &check)),
                )
                .collect();
            prev = curr;
            curr = next.into_iter().collect();
            layers.push(curr.clone());
            for &c in &curr {
                dists.insert(c, d);
            }
        }

        (layers, dists)
    }

    /// Some heuristics.
    pub fn continue_lines(&mut self, args: &Args) {
        self._continue_lines(false, true, 0);
        self.log(args, "continue_simple_0");

        self._continue_lines(false, true, 1);
        self.log(args, "continue_simple_1");

        self._continue_lines(false, true, 0);
        self.log(args, "continue_simple_2");
    }
    fn _continue_lines(&mut self, allow_two: bool, straight: bool, threshhold: usize) {
        let extend: Vec<_> = self
            .all()
            .filter_map(|&c| self._suggest_next(c, allow_two, straight, threshhold))
            .collect();
        for c in extend.into_iter().flat_map(|v| v.into_iter()) {
            self.insert(c);
        }
    }
    fn _suggest_next(
        &self,
        center: Coord,
        allow_two: bool,
        straight: bool,
        threshhold: usize,
    ) -> Option<Vec<Coord>> {
        assert!(threshhold <= 3);
        let neigh = match straight {
            true => self.neigh(center),
            false => self.neigh2(center),
        };
        match neigh[..] {
            [] => None,
            [root] => {
                let arrow = root - center;
                let dirs = local::rotations(arrow);
                (0..threshhold + 1).find_map(|t| {
                    let mut res = Vec::new();
                    if self.neigh(center + dirs[3 - t]).len() > 1 {
                        res.push(center + dirs[3 - t])
                    }
                    if self.neigh(center + dirs[3 + t]).len() > 1 {
                        res.push(center + dirs[3 + t])
                    }
                    match res.len() {
                        0 => None,
                        _ => Some(res),
                    }
                })
            }
            [_r1, _r2] if allow_two => unimplemented!(),
            _ => None,
        }
    }

    // To be done: generate matrixes to compare with.
    pub fn determine_type_1(&self, center: Coord) -> Type1 {
        let (layers, _) = self.bfs(center, 5);

        if layers[4].len() == 0 && layers[5].len() == 0 {
            return Type1::Island;
        }

        let neigh = self.neigh(center);
        match neigh.len() {
            0 => return Type1::Lone,
            1 => {
                // To be done: check `Sprout`.
                return Type1::End;
            }
            2 => match local::angle(neigh[0], center, neigh[1]) {
                a if a == 3 || a == -3 => {
                    let n1 = self.neigh1(center);
                    assert!(n1.len() == 1);
                    let n2 = self.neigh2(center);
                    assert!(n2.len() == 1);

                    let a = n1[0] - center;
                    let b = n2[0] - n1[0];
                    /*
                       ---
                       -+-5
                       -++4
                       0123
                    */
                    let ns = [
                        self[center + a + a - b],
                        self[center + a + a],
                        self[center + a + a + b],
                        self[center + a + a + b + b],
                        self[center + a + b + b],
                        self[center + b + b],
                    ];
                    match ns.iter().map(|&b| b as usize).sum::<usize>() {
                        0 => unreachable!(), // Island.
                        1 => match ns.iter().position(|&a| a).unwrap() {
                            0 | 1 => return Type1::Sprout,
                            _ => return Type1::End,
                        },
                        2 => {
                            let nsi: Vec<_> = ns
                                .iter()
                                .enumerate()
                                .filter_map(|(n, &b)| match b {
                                    true => Some(n),
                                    false => None,
                                })
                                .collect();
                            assert!(nsi.len() == 2);
                            assert!(nsi[1] > nsi[0]);
                            match (nsi[0], nsi[1]) {
                                (0, 1) => return Type1::Sprout,
                                (1, 2) => return Type1::Path(neigh[0], neigh[1]),
                                (a, b) if b == a + 1 => return Type1::End,
                                (a, _) if a <= 1 => return Type1::Path(neigh[0], neigh[1]),
                                _ => return Type1::Sprout,
                            }
                        }
                        // To be done: 3 - `Path` or `Sprout`?
                        _ => return Type1::Path(neigh[0], neigh[1]),
                    }
                }
                _ => return Type1::Path(neigh[0], neigh[1]),
            },
            3 => {
                let n0 = neigh[0];
                let n1 = neigh[1];
                let n2 = neigh[2];
                let a01 = local::angle(n0, center, n1).abs();
                let a12 = local::angle(n1, center, n2).abs();
                let a20 = local::angle(n2, center, n0).abs();

                let closer: Vec<_> = [a01, a12, a20]
                    .into_iter()
                    .enumerate()
                    .filter(|(_, a)| *a == 3)
                    .collect();
                if closer.len() == 1 {
                    let lone = match closer[0].0 {
                        0 => neigh[2],
                        1 => neigh[0],
                        2 => neigh[1],
                        _ => unreachable!(),
                    };
                    let other = self
                        .neigh1(center)
                        .iter()
                        .find(|&&o| o != lone)
                        .map(|&c| c)
                        .unwrap();
                    return Type1::Path(other, lone);
                }
                // if [a01, a12, a20].iter().filter(|&&a| a == 3).count() > 0 {
                //     return Type1::Path;
                // }
                // To be done: more precise.
                return Type1::KnotStrong;
            }
            4 => {
                let n0 = neigh[0];
                let a01 = local::angle(n0, center, neigh[1]).abs();
                let a02 = local::angle(n0, center, neigh[2]).abs();
                let a03 = local::angle(n0, center, neigh[3]).abs();
                let n0near = if a01 == 3 {
                    1
                } else if a02 == 3 {
                    2
                } else if a03 == 3 {
                    3
                } else {
                    4
                };
                if n0near != 4 {
                    let n0near = neigh[n0near];
                    let other: Vec<_> = neigh[1..].iter().filter(|&&n| n != n0near).collect();
                    if !other.iter().any(|&&o| {
                        self.neigh1(o)
                            .into_iter()
                            .any(|on| on == n0 || on == n0near)
                    }) && local::angle(*other[0], center, *other[1]).abs() == 3
                    {
                        let near = match self.neigh1(center).contains(&n0) {
                            true => n0,
                            false => n0near,
                        };
                        assert!(other.len() == 2);
                        let other = match self.neigh1(center).contains(other[0]) {
                            true => other[0],
                            false => other[1],
                        };
                        return Type1::Path(near, *other);
                    }
                }

                let a12 = local::angle(neigh[1], center, neigh[2]).abs();
                let a13 = local::angle(neigh[1], center, neigh[3]).abs();
                let a23 = local::angle(neigh[2], center, neigh[3]).abs();
                let na: Vec<_> = [a01, a02, a03, a12, a13, a23]
                    .iter()
                    .enumerate()
                    .filter(|(_, &a)| a == 3)
                    .map(|(n, _)| n)
                    .collect();
                if na.len() == 2 {
                    let (lone, enclosed) = match na[0..2] {
                        [0, 1] => (3, 0),
                        [0, 2] => (2, 0),
                        [0, 3] => (3, 1),
                        [0, 4] => (2, 1),
                        [0, 5] => unreachable!(), // Should be handled before.
                        [1, 2] => (1, 0),
                        [1, 3] => (3, 2),
                        [1, 4] => unreachable!(), // Should be handled before.
                        [1, 5] => (1, 2),
                        [2, 3] => unreachable!(), // Should be handled before.
                        [2, 4] => (2, 3),
                        [2, 5] => (1, 3),
                        [3, 4] => (0, 1),
                        [3, 5] => (0, 2),
                        [4, 5] => (0, 3),
                        _ => unreachable!(),
                    };
                    match center.dist_sq(neigh[enclosed]) {
                        1 => return Type1::Path(neigh[lone], neigh[enclosed]),
                        2 => return Type1::Knot,
                        _ => unreachable!(),
                    }
                }

                // if [a01, a02, a03, a12, a13, a23]
                //     .iter()
                //     .filter(|&&a| a == 3)
                //     .count()
                //     == 2
                // {
                //     return Type1::Path;
                // }
            }
            _ => {}
        }

        // To be done: more precise.
        Type1::KnotMany
    }

    pub fn find_nearests(
        &self,
        center: Coord,
        limit: usize,
        good: &dyn Fn(Coord) -> bool,
    ) -> Option<Vec<Coord>> {
        for d in 1..limit + 1 {
            let layer = local::get_dist(center, d, good);
            if layer.len() > 0 {
                return Some(layer);
            }
        }
        None
    }

    pub fn determine_degree_1(&self, c: Coord) -> usize {
        self.neigh1(c).len()
    }
    pub fn determine_degree_2(&self, c: Coord) -> usize {
        self.neigh(c).len()
    }
    pub fn determine_type_2(&self, c: Coord, ty1: &HashMap<Coord, Type1>) -> Type2 {
        let (_, dists) = self.bfs(c, 3);
        if dists
            .iter()
            .any(|(c, _)| matches!(ty1.get(c), Some(Type1::Island)))
        {
            return Type2::Island;
        }
        match ty1.get(&c).expect("expected existed vert") {
            Type1::Lone | Type1::Island => Type2::Island,
            Type1::End => Type2::End,
            Type1::NearEnd => {
                if self.determine_degree_2(c) <= 1 {
                    return Type2::End;
                }
                for (c, _) in dists {
                    if self.determine_degree_1(c) == 1 {
                        return Type2::Path;
                    }
                }
                Type2::End
            }
            Type1::Path(..) => Type2::Path,
            Type1::Knot if self.determine_degree_2(c) == 2 => Type2::Path,
            Type1::Knot if self.determine_degree_2(c) <= 3 => Type2::Knot,
            // To be done: do smth.
            _ if self.determine_degree_2(c) == 2 => Type2::Path,
            _ => Type2::Big,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Type1 {
    Lone,
    Island,
    Sprout,
    End,
    NearEnd,
    Path(Coord, Coord),
    Knot,
    KnotStrong,
    KnotMany,
}

impl Type1 {
    pub fn is_knot(&self) -> bool {
        match self {
            Type1::Knot | Type1::KnotStrong | Type1::KnotMany => true,
            Type1::Sprout
            | Type1::Lone
            | Type1::Island
            | Type1::End
            | Type1::NearEnd
            | Type1::Path(_, _) => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Type2 {
    Island,
    End,
    Path,
    Knot,
    Big,
}

impl Into<[f32; 3]> for Type1 {
    fn into(self) -> [f32; 3] {
        match self {
            Type1::Lone => [1., 0., 0.],
            Type1::Island => [0.5, 0., 0.],
            Type1::Sprout => [1., 0., 1.],
            Type1::End => [0., 0., 1.],
            Type1::NearEnd => [0., 0.2, 0.4],
            Type1::Path(..) => [0., 0.4, 0.],
            Type1::Knot => [0.4, 0.4, 0.],
            Type1::KnotStrong => [1., 0.7, 0.],
            Type1::KnotMany => [1., 1., 1.],
        }
    }
}

impl Into<[f32; 3]> for Type2 {
    fn into(self) -> [f32; 3] {
        match self {
            Type2::Island => [1., 0., 0.],
            Type2::End => [0., 0., 1.],
            Type2::Path => [0., 0.4, 0.],
            Type2::Knot => [0.4, 0.4, 0.],
            Type2::Big => [1., 0.9, 0.5],
        }
    }
}

impl Graph {
    /// # Returns: (inner, border).
    pub fn bfs_inner(&self, start: Coord, on_l: &Area) -> (Area, Area) {
        let mut inner = HashSet::new();
        let mut border = HashSet::new();

        let mut active = HashSet::new();
        active.insert(start);
        while !active.is_empty() {
            let mut next = HashSet::new();
            for &from in &active {
                let d1 = local::get_dist(from, 1, &|c| !inner.contains(&c) && !on_l.contains(&c));
                for d1 in d1 {
                    if self.contains(d1) {
                        border.insert(d1);
                    } else {
                        next.insert(d1);
                    }
                }
                let d2 = local::get_dist(from, 2, &|c| !inner.contains(&c) && !on_l.contains(&c));
                for d2 in d2 {
                    let d = d2 - from;
                    let dy = (d.y, 0);
                    let dx = (0, d.x);
                    let a = from + dx;
                    let b = from + dy;
                    if !self
                        .edges()
                        .get(&a)
                        .map(|s| s.contains(&b))
                        .unwrap_or(false)
                    {
                        if !self.contains(d2) {
                            next.insert(d2);
                        }
                        border.insert(d2);
                    }
                }
            }
            inner.extend(active.into_iter());
            active = next
        }

        for i in &inner {
            border.remove(i);
        }

        (inner, border)
    }

    /// Expects borders.
    pub fn areas(&self, w: usize, h: usize) -> Vec<(Area, Area)> {
        let mut on_long = HashSet::new();
        for y in 1..h {
            for x in 1..w {
                let c = (y, x).into();
                if self.intersects_long(c) {
                    on_long.insert(c);
                }
            }
        }

        let mut visited = HashSet::new();
        let mut areas = Vec::new();
        for y in 1..h {
            for x in 1..w {
                let c = (y, x).into();
                if self.contains(c) || visited.contains(&c) {
                    continue;
                }
                let (inner, border) = self.bfs_inner(c, &on_long);
                for &i in &inner {
                    visited.insert(i);
                }
                areas.push((inner, border))
            }
        }
        areas
    }

    fn intersects_long(&self, c: Coord) -> bool {
        for (a, b) in self.edges_l() {
            let ab = a.dist_sq(*b) as f32;
            let ca = c.dist_sq(*a) as f32;
            let cb = c.dist_sq(*b) as f32;
            if ca.sqrt() + cb.sqrt() - ab.sqrt() < 2. {
                return true;
            }
        }
        false
    }

    pub fn _nearest_right(from: Coord, to: Coord, next: Vec<Coord>) -> Coord {
        let angle_base = (from - to)._angle();
        let angles: Vec<_> = next
            .iter()
            .filter(|&&c| c != from)
            .map(|&c| (c - to)._angle())
            .collect();

        // dbg!(from, to, angle_base, &next, &angles);
        match angles.iter().find(|&&a| a >= angle_base) {
            Some(_) => {
                let mut min = 9.;
                for &a in angles.iter().filter(|&&a| a >= angle_base) {
                    if min > a {
                        min = a
                    }
                }
                next.into_iter()
                    .find(|&c| (c - to)._angle() == min)
                    .unwrap()
            }
            None => {
                let mut min = 9.;
                for &a in angles.iter() {
                    if min > a {
                        min = a
                    }
                }
                if next.iter().find(|&&c| (c - to)._angle() == min).is_none() {
                    dbg!(from, to, angle_base, &next, &angles);
                }
                next.into_iter()
                    .find(|&c| (c - to)._angle() == min)
                    .unwrap()
            }
        }
    }

    pub fn remove_sticks(&mut self) {
        let mut candidats: HashSet<_> = self.edges().iter().map(|(&from, _)| from).collect();
        let mut to_remove = Vec::new();

        loop {
            for from in candidats {
                match self.edges().get(&from).unwrap().len() {
                    0 => self.erase_v(from),
                    1 => to_remove.push(from),
                    _ => {}
                }
            }
            if to_remove.is_empty() {
                return;
            }
            candidats = HashSet::new();
            for r in to_remove {
                let l = *self.edges().get(&r).unwrap().iter().next().unwrap();
                self.erase_short(r, l);
                self.erase_v(r);
                candidats.insert(l);
            }
            to_remove = Vec::new()
        }
    }

    /// Expects frame.
    pub fn areas2polygons(
        &mut self,
        w: usize,
        h: usize,
        areas: &HashMap<Coord, &HashSet<Coord>>,
        colors: &HashMap<Coord, [f32; 3]>,
    ) -> Vec<(Vec<Coord>, [f32; 3])> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        let area_1_1 = areas.get(&(1, 1).into()).unwrap();
        let area_37_180 = areas.get(&(37, 180).into()).unwrap();
        dbg!(area_1_1.contains(&(37, 180).into()));
        dbg!(area_37_180.contains(&(37, 180).into()));
        dbg!(area_37_180.len());
        for (y, x) in (0..h).flat_map(|y| (0..w).map(move |x| (y, x))) {
            if area_1_1.contains(&(y, x).into()) != area_37_180.contains(&(y, x).into()) {
                dbg!(Coord::from((y, x)));
            }
        }
        self.mark((213, 120).into(), [1., 0.5, 0.3]);

        let mut border: Coord = (0, 0).into();
        for (y, x) in (0..h).flat_map(|y| (0..w).map(move |x| (y, x))) {
            let c = (y, x).into();
            if self.contains(c) {
                assert!(self.neigh(c).next().is_some());
                border = (y, x).into();
                continue;
            }
            if visited.contains(&c) {
                continue;
            }
            if !areas.contains_key(&c) {
                continue;
            }
            if !local::get_dist(c, 1, &|c| visited.contains(&c)).is_empty() {
                dbg!(c);
                continue;
            }
            if areas.get(&c).unwrap().contains(&(1, 1).into()) {
                // dbg!(c);
                continue;
            }
            // if areas.get(&c).unwrap().len() >= area_1_1.len() - 10 {
            //     dbg!(c);
            //     continue;
            // }
            if areas.get(&c).unwrap().len() < 10 {
                dbg!(c);
                continue;
            }
            self.mark(c, [0.1, 0.1, 1.]);
            self.mark(border, [0.1, 0.6, 0.2]);
            dbg!(c);
            let mut path = vec![border];
            let cur = border;
            let neigh: Vec<_> = self.neigh(cur).map(|&c| c).collect();
            assert!(neigh.len() >= 2);
            let next = Self::_nearest_right(cur - Coord::from((0, -1)), cur, neigh);
            path.push(next);
            for _ in 0..10000 {
                let len = path.len();
                let cur = path[len - 1];
                let prev = path[len - 2];
                let neigh: Vec<_> = self.neigh(cur).map(|&c| c).collect();
                if neigh.len() < 2 {
                    dbg!(c);
                    // dbg!(&path);
                    dbg!(prev, cur, &neigh);
                    assert!(neigh.len() >= 2);
                }
                let next = Self::_nearest_right(prev, cur, neigh);
                // dbg!(cur);
                if self.neigh(cur).count() > 2 {
                    // dbg!(cur);
                    // dbg!(next);
                    // dbg!(path.len());
                }
                if next == border {
                    break;
                }
                path.push(next)
            }
            if path.len() > 10000 {
                dbg!(&path[..30]);
                self.mark(c, [1., 0.5, 0.3]);
                continue;
            }

            // dbg!(c, colors.get(&c));
            match colors.get(&c) {
                Some(&c) => result.push((path, c)),
                None => {
                    dbg!(path);
                    eprintln!("couldn't find color for {:?}", c)
                }
            }
            visited.extend(areas.get(&c).unwrap().iter().map(|&c| c));
            dbg!(areas.get(&c).unwrap().len());
            dbg!(areas.get(&c).unwrap().contains(&(37, 180).into()));

            // if y > 38 {
            //     break;
            // }
        }

        result
    }
}

#[cfg(test)]
mod test;
