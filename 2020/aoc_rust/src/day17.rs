use std::collections::{HashMap};
use itertools::Itertools;

pub fn run_day17(puzzle_input: &str) {
    let init_state = std::fs::read_to_string(&puzzle_input)
        .expect(format!(
            "Could not read file \"{}\"",
            &puzzle_input
        ).as_str());

    let mut grid = InfGrid3D::new(&init_state);
    for _ in 0..6 {
        grid = grid.step();
    }
    println!("Active in 3D after 6: {}", grid.count_active());

    let mut grid = InfGrid4D::new(&init_state);
    for _ in 0..6 {
        grid = grid.step();
    }
    println!("Active in 4D after 6: {}", grid.count_active());

}

#[derive(Clone)]
struct InfGrid3D {
    grid: HashMap<i64, HashMap<i64, HashMap<i64, bool>>>
}

impl InfGrid3D {
    fn new(init_state: &str) -> InfGrid3D {
        let mut g = InfGrid3D { grid: HashMap::new() };
        for (row, line) in init_state.split('\n').enumerate() {
            for (col, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    g.set(row as i64, col as i64, 0, true);
                }
            }
        }
        g
    }

    fn get(&self, x: i64, y: i64, z: i64) -> bool {
        if let Some(ys) = self.grid.get(&x) {
            if let Some(zs) = ys.get(&y) {
                return zs.contains_key(&z);
            }
        }
        false
    }

    fn set(&mut self, x: i64, y: i64, z: i64, enabled: bool) {
        if enabled {
            if let Some(ys) = self.grid.get_mut(&x) {
                if let Some(zs) = ys.get_mut(&y) {
                    zs.insert(z, true);
                } else {
                    let mut zs = HashMap::new();
                    zs.insert(z, true);
                    ys.insert(y, zs);
                }
            } else {
                let mut ys = HashMap::new();
                let mut zs = HashMap::new();
                zs.insert(z, true);
                ys.insert(y, zs);
                self.grid.insert(x, ys);
            }
        } else {  // disabled
            if let Some(ys) = self.grid.get_mut(&x) {
                if let Some(zs) = ys.get_mut(&y) {
                    zs.remove(&z);
                    if zs.is_empty() { ys.remove(&y); }
                }
                if ys.is_empty() { self.grid.remove(&x); }
            }
        }
    }

    fn count_neighbors(&self, x: i64, y: i64, z: i64) -> usize {
        let mut acc: usize = 0;
        for ((i, j), k) in (0..=2).cartesian_product(0..=2)
                .cartesian_product(0..=2) {
            if i == 1 && j == 1 && k == 1 { continue; }
            if self.get(
                x + i as i64 - 1,
                y + j as i64 - 1,
                z + k as i64 - 1
            ) { acc += 1; }
        }
        acc
    }

    fn count_active(&self) -> usize {
        let mut acc: usize = 0;
        for ys in self.grid.values() {
            for zs in ys.values() {
                for _ in zs { acc += 1; }
            }
        }
        acc
    }

    fn step(&self) -> InfGrid3D {
        let mut next = self.clone();
        // Iterate over all cells that are relevant for the current cycle;
        // i.e., all currently active cells and their direct neighbors.
        for (x, ys) in &self.grid {
            for (y, zs) in ys {
                for z in zs.keys() {
                    // Neighbors + current cell:
                    for ((i, j), k) in (0..=2).cartesian_product(0..=2)
                            .cartesian_product(0..=2) {
                        let xc = x + i as i64 - 1;
                        let yc = y + j as i64 - 1;
                        let zc = z + k as i64 - 1;
                        let nbs = self.count_neighbors(xc, yc, zc);
                        let enabled = self.get(xc, yc, zc);
                        next.set(
                            xc, yc, zc,
                            (enabled && (nbs == 2 || nbs == 3))
                            || (!enabled && nbs == 3)
                        );
                    }
                }
            }
        }
        next
    }
}

#[derive(Clone)]
struct InfGrid4D {
    grid: HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, bool>>>>
}

// Copy and pasted; could probably be made to support arbitrary
// dimensionalities…
impl InfGrid4D {
    fn new(init_state: &str) -> InfGrid4D {
        let mut g = InfGrid4D { grid: HashMap::new() };
        for (row, line) in init_state.split('\n').enumerate() {
            for (col, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    g.set(row as i64, col as i64, 0, 0, true);
                }
            }
        }
        g
    }

    fn get(&self, x1: i64, x2: i64, x3: i64, x4: i64) -> bool {
        if let Some(x2s) = self.grid.get(&x1) {
            if let Some(x3s) = x2s.get(&x2) {
                if let Some(x4s) = x3s.get(&x3) {
                    return x4s.contains_key(&x4);
                }
            }
        }
        false
    }

    fn set(&mut self, x1: i64, x2: i64, x3: i64, x4: i64, enabled: bool) {
        if enabled {
            if let Some(x2s) = self.grid.get_mut(&x1) {
                if let Some(x3s) = x2s.get_mut(&x2) {
                    if let Some(x4s) = x3s.get_mut(&x3) {
                        x4s.insert(x4, true);
                    } else {
                        let mut x4s = HashMap::new();
                        x4s.insert(x4, true);
                        x3s.insert(x3, x4s);
                    }
                } else {
                    let mut x4s = HashMap::new();
                    let mut x3s = HashMap::new();
                    x4s.insert(x4, true);
                    x3s.insert(x3, x4s);
                    x2s.insert(x2, x3s);
                }
            } else {
                let mut x2s = HashMap::new();
                let mut x3s = HashMap::new();
                let mut x4s = HashMap::new();
                x4s.insert(x4, true);
                x3s.insert(x3, x4s);
                x2s.insert(x2, x3s);
                self.grid.insert(x1, x2s);
            }
        } else {  // disabled
            if let Some(x2s) = self.grid.get_mut(&x1) {
                if let Some(x3s) = x2s.get_mut(&x2) {
                    if let Some(x4s) = x3s.get_mut(&x3) {
                        x4s.remove(&x4);
                        if x4s.is_empty() { x3s.remove(&x3); }
                    }
                    if x3s.is_empty() { x2s.remove(&x2); }
                }
                if x2s.is_empty() { self.grid.remove(&x1); }
            }
        }
    }

    fn count_neighbors(&self, x1: i64, x2: i64, x3: i64, x4: i64) -> usize {
        let mut acc: usize = 0;
        for (((i, j), k), l) in (0..=2).cartesian_product(0..=2)
                .cartesian_product(0..=2).cartesian_product(0..=2) {
            if i == 1 && j == 1 && k == 1  && l == 1 { continue; }
            if self.get(
                x1 + i as i64 - 1,
                x2 + j as i64 - 1,
                x3 + k as i64 - 1,
                x4 + l as i64 - 1
            ) { acc += 1; }
        }
        acc
    }

    fn count_active(&self) -> usize {
        let mut acc: usize = 0;
        for x2s in self.grid.values() {
            for x3s in x2s.values() {
                for x4s in x3s.values() {
                    for _ in x4s { acc += 1; }
                }
            }
        }
        acc
    }

    fn step(&self) -> InfGrid4D {
        let mut next = self.clone();
        // Iterate over all cells that are relevant for the current cycle;
        // i.e., all currently active cells and their direct neighbors.
        for (x1, x2s) in &self.grid {
            for (x2, x3s) in x2s {
                for (x3, x4s) in x3s {
                    for x4 in x4s.keys() {
                        // Neighbors + current cell:
                        for (((i, j), k), l) in (0..=2)
                                .cartesian_product(0..=2)
                                .cartesian_product(0..=2)
                                .cartesian_product(0..=2) {
                            let x1c = x1 + i as i64 - 1;
                            let x2c = x2 + j as i64 - 1;
                            let x3c = x3 + k as i64 - 1;
                            let x4c = x4 + l as i64 - 1;
                            let nbs = self.count_neighbors(x1c, x2c, x3c, x4c);
                            let enabled = self.get(x1c, x2c, x3c, x4c);
                            next.set(
                                x1c, x2c, x3c, x4c,
                                (enabled && (nbs == 2 || nbs == 3))
                                || (!enabled && nbs == 3)
                            );
                        }
                    }
                }
            }
        }
        next
    }
}
