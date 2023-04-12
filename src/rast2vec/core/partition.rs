use std::collections::{HashMap, HashSet};

use super::super::{Pos, PosGood, View};

use super::super::space::utility::pos_debug_hashset;

pub struct Partition {
    areas: Vec<Area>,
    // Index in `areas` of points (it may not exists).
    pos2area: HashMap<Pos, usize>,
}

pub struct Area {
    // Inner cells.
    pub surface: HashSet<PosGood>,
    // Near cells.
    pub borders: HashSet<PosGood>,
    // Path through knots.
    pub outline: Vec<Pos>,
}

impl Partition {
    pub fn areas(&self) -> &Vec<Area> {
        &self.areas
    }

    pub fn check(&self) {
        for (n, area) in self.areas.iter().enumerate() {
            for p in &area.surface {
                let p: Pos = Into::<Pos>::into(*p);
                assert!(self.pos2area.get(&p).unwrap() == &n)
            }
        }
    }

    pub fn new_spread_equal_neigh1(view: &impl View, min_area_size: usize) -> Self {
        let mut visited = HashSet::new();

        let mut areas = Vec::new();
        let mut pos2area = HashMap::new();
        for pos in view.rows().flat_map(|r| r) {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            let mut surface = HashSet::new();
            surface.insert(pos);
            let mut queue = vec![pos];
            let mut borders = HashSet::new();
            while let Some(cur) = queue.pop() {
                for next in view.neigh1_if(cur.into(), &|p| !visited.contains(&p)) {
                    if view.get(next).diff(view.get(cur)) < 55 {
                        // if view.get(next) == view.get(cur) {
                        surface.insert(next);
                        visited.insert(next);
                        queue.push(next)
                    } else {
                        borders.insert(next);
                    }
                }
            }
            // if surface.len() < 800 {
            //     let mut queue: Vec<_> = surface.iter().map(|&c| c).collect();
            //     borders = HashSet::new();
            //     while let Some(cur) = queue.pop() {
            //         for next in view.neigh1_if(cur.into(), &|p| !visited.contains(&p)) {
            //             if view.get(next).diff(view.get(cur)) < 100 {
            //                 // if view.get(next) == view.get(cur) {
            //                 surface.insert(next);
            //                 visited.insert(next);
            //                 queue.push(next)
            //             } else {
            //                 borders.insert(next);
            //             }
            //         }
            //     }
            // }
            // if surface.len() < 400 {
            //     let mut queue: Vec<_> = surface.iter().map(|&c| c).collect();
            //     borders = HashSet::new();
            //     while let Some(cur) = queue.pop() {
            //         for next in view.neigh1_if(cur.into(), &|p| !visited.contains(&p)) {
            //             if view.get(next).diff(view.get(cur)) < 250 {
            //                 // if view.get(next) == view.get(cur) {
            //                 surface.insert(next);
            //                 visited.insert(next);
            //                 queue.push(next)
            //             } else {
            //                 borders.insert(next);
            //             }
            //         }
            //     }
            // }
            if surface.len() < min_area_size {
                continue;
            }
            if surface.len() < min_area_size || false {
                eprintln!("surface:\n{}", &pos_debug_hashset(&surface));
                eprintln!("borders:\n{}", &pos_debug_hashset(&borders));
            }
            let knots: HashSet<_> = surface
                .iter()
                .flat_map(|&p| {
                    let p: Pos = p.into();
                    [p, p.mv_r(), p.mv_d(), p.mv_rd()]
                })
                .collect();
            let knots: HashSet<_> = knots
                .into_iter()
                .filter(|&k| {
                    let ns = view.neigh_of_knot(k);
                    if ns.len() < 4 {
                        return true;
                    }
                    ns.iter().any(|p| !surface.contains(p))
                })
                .collect();
            let pos: Pos = pos.into();
            assert!(knots.contains(&pos));
            let mut visited_knots = HashSet::new();
            let mut visit_twice_knots = HashSet::new();
            visited_knots.insert(pos);
            let mut outline = vec![pos];
            let limit = 100000;
            for i in 0..limit {
                let prev: Pos = outline[outline.len() - 1].into();
                // dbg!(next);
                if outline.len() > 2 && pos.neigh_if(1, &|p| knots.contains(&p)).contains(&prev) {
                    break;
                }
                let next = prev.neigh_if(1, &|p| {
                    knots.contains(&p)
                        && (!visited_knots.contains(&p) || visit_twice_knots.contains(&p))
                });

                // dbg!((prev, &next));
                let next = match next.len() {
                    1 => next[0],
                    l if l > 0 && outline.len() == 1 => next[0],
                    3 => {
                        // To be done: fix this.
                        let ppr = outline[outline.len() - 2];
                        dbg!(prev);
                        visit_twice_knots.insert(prev);
                        next.into_iter()
                            .filter_map(|k| view.between_knots(ppr, k).map(|p| (p, k)))
                            .find(|(p, _)| surface.contains(p))
                            // .map(|v| dbg!(v))
                            .unwrap()
                            .1
                    }
                    2 => {
                        // Hack for visiting some knots twice.
                        let twice = next
                            .iter()
                            .filter(|&k| visit_twice_knots.contains(k))
                            .count();
                        if twice != 1 {
                            dbg!(outline);
                            dbg!(next);
                            panic!()
                        }
                        next.into_iter()
                            .find(|k| !visit_twice_knots.contains(&k))
                            .unwrap()
                    }
                    _ => {
                        dbg!(outline);
                        dbg!(next);
                        dbg!(visit_twice_knots);
                        unreachable!()
                    }
                };
                visit_twice_knots.remove(&next);

                // let next = *next.last().expect("where should be cycle");
                outline.push(next);
                visited_knots.insert(next);
                assert!(i + 1 < limit);
            }
            eprintln!("found for {:?}: {}", pos, surface.len());
            for &p in &surface {
                pos2area.insert(p.into(), areas.len());
            }
            areas.push(Area {
                surface,
                borders,
                outline,
            })
        }

        eprintln!("done");

        Self { areas, pos2area }
    }
}
