extern crate itertools;
use std::vec::Vec;
use itertools::Itertools;

pub fn run_day11(puzzle_input: &str) {
    let rows = SeatLayoutRuleOne::get_rows_from_str(
        std::fs::read_to_string(&puzzle_input)
            .expect(format!(
                "Could not read file \"{}\".",
                puzzle_input
            ).as_str()).as_str()
    );

    let seat_layout_rule1 = SeatLayoutRuleOne{ rows: rows.clone() };
    println!(
        "Number of occupied seats rule one: {}",
        run_to_convergence(&seat_layout_rule1).count_occupied()
    );

    let seat_layout_rule2 = SeatLayoutRuleTwo{ rows };
    println!(
        "Number of occupied seats rule two: {}",
        run_to_convergence(&seat_layout_rule2).count_occupied()
    );
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SeatPosition {
    Floor,
    Empty,
    Occupied
}

impl SeatPosition {
    fn to_string(&self) -> String {
        String::from(match *self {
            SeatPosition::Floor => ".",
            SeatPosition::Empty => "L",
            SeatPosition::Occupied => "#"
        })
    }
}

#[derive(Clone, PartialEq)]
struct SeatLayoutRuleOne {
    rows: Vec<Vec<SeatPosition>>
}

impl SeatLayout for SeatLayoutRuleOne {
    fn get_rows(&self) -> &Vec<Vec<SeatPosition>> {
        &self.rows
    }

    fn get_rows_mut(&mut self) -> &mut Vec<Vec<SeatPosition>> {
        &mut self.rows
    }
}

#[derive(Clone, PartialEq)]
struct SeatLayoutRuleTwo {
    rows: Vec<Vec<SeatPosition>>
}

impl SeatLayout for SeatLayoutRuleTwo {
    fn get_rows(&self) -> &Vec<Vec<SeatPosition>> {
        &self.rows
    }

    fn get_rows_mut(&mut self) -> &mut Vec<Vec<SeatPosition>> {
        &mut self.rows
    }
}

impl SeatingRule for SeatLayoutRuleOne {
    fn count_occupied_neighbors(&self, row: usize, col: usize) -> usize {
        let mut occupied_neighbors = 0;
        for (i, j) in (0..=2).cartesian_product(0..=2) {
            if i == 1 && j == 1 { continue; }
            if let Some(neighbor) = self.get(
                    (row + i) as i32 - 1,
                    (col + j) as i32 - 1
            ) {
                occupied_neighbors +=
                    if neighbor == &SeatPosition::Occupied {1} else {0};
            }
        }
        occupied_neighbors
    }

    fn get_new_seat_pos(
        occupied_neighbors: usize,
        sp: SeatPosition
    ) -> SeatPosition {
        if sp == SeatPosition::Empty
                && occupied_neighbors == 0 {
            return SeatPosition::Occupied;
        } else if sp == SeatPosition::Occupied
                && occupied_neighbors >= 4 {
            return SeatPosition::Empty;
        }
        sp
    }
}

impl SeatLayoutRuleTwo {
    fn get_first_visible_seat_pos(
        &self,
        row: usize,
        col: usize,
        step_x: i32,
        step_y: i32
    ) -> Option<&SeatPosition> {
        let mut x: i32 = row as i32;
        let mut y: i32 = col as i32;
        loop {
            x += step_x;
            y += step_y;
            let current_seat_pos = self.get(x, y);
            if let None = current_seat_pos { return None; }
            else if let Some(SeatPosition::Occupied)
                    | Some(SeatPosition::Empty) = current_seat_pos {
                return current_seat_pos;
            } // else will be SeatPosition::Floor
        }
    }
}

impl SeatingRule for SeatLayoutRuleTwo {
    fn count_occupied_neighbors(&self, row: usize, col: usize) -> usize {
        let mut occupied_neighbors = 0;
        for (i, j) in (0..=2).cartesian_product(0..=2) {
            if i == 1 && j == 1 { continue; }
            let step_x: i32 = i - 1;
            let step_y: i32 = j - 1;
            if let Some(neighbor) = self.get_first_visible_seat_pos(
                row, col, step_x, step_y
            ) {
                occupied_neighbors +=
                    if neighbor == &SeatPosition::Occupied {1} else {0};
            }
        }
        occupied_neighbors

    }

    fn get_new_seat_pos(
        occupied_neighbors: usize,
        sp: SeatPosition
    ) -> SeatPosition {
        if sp == SeatPosition::Empty
                && occupied_neighbors == 0 {
            return SeatPosition::Occupied;
        } else if sp == SeatPosition::Occupied
                && occupied_neighbors >= 5 {
            return SeatPosition::Empty;
        }
        sp
    }
}

trait SeatingRule {
    fn count_occupied_neighbors(&self, row: usize, col: usize) -> usize;

    fn get_new_seat_pos(
        occupied_neighbors: usize,
        sp: SeatPosition
    ) -> SeatPosition;
}

trait SeatLayout {
    fn get_rows(&self) -> &Vec<Vec<SeatPosition>>;
    fn get_rows_mut(&mut self) -> &mut Vec<Vec<SeatPosition>>;

    fn get_rows_from_str(layout_str: &str) -> Vec<Vec<SeatPosition>> {
        layout_str
            .split('\n')
            .map(|line| line.chars().map(|c| {
                match c {
                    '.' => SeatPosition::Floor,
                    'L' => SeatPosition::Empty,
                    '#' => SeatPosition::Occupied,
                    e => { panic!("Unrecognized SeatPosition {}", e); }
                }
            }).collect::<Vec<SeatPosition>>())
            .collect::<Vec<Vec<SeatPosition>>>()
    }

    fn to_string(&self) -> String {
        String::from(
            self.get_rows().iter().map(|row| {
                row.iter().enumerate().map(|(i, seat_pos)| {
                    if i == row.len() - 1 {
                        format!("{}\n", seat_pos.to_string())
                    } else {
                        seat_pos.to_string()
                    }
                }).join("")
            }).join("")
        )
    }

    fn count_occupied(&self) -> usize {
        let mut count: usize = 0;
        for row in self.get_rows() {
            for seat in row {
                if let SeatPosition::Occupied = seat {
                    count += 1;
                }
            }
        }
        count
    }

    fn get(
        &self,
        row_i: i32,
        col_i: i32
    ) -> Option<&SeatPosition> {
        if row_i < 0 || col_i < 0 { return None; }
        self.get_rows().get(row_i as usize).and_then(|row| row.get(col_i as usize))
    }
}

fn run_to_convergence<T>(
    seat_layout: &T
) -> T where
        T: SeatingRule + Clone + PartialEq + SeatLayout {
    let mut prev_layout = seat_layout.clone();
    loop {
        let new_layout = step(&prev_layout);
        if new_layout == prev_layout { return new_layout; }
        prev_layout = new_layout;
    }
}

fn step<T>(
    seat_layout: &T
) -> T where
        T: SeatingRule + Clone + SeatLayout {
    let mut new_layout: T = seat_layout.clone();
    for (row_i, row) in seat_layout.get_rows().iter().enumerate() {
        for (col_i, seat_pos) in row.iter().enumerate() {
            let occupied_neighbors = seat_layout.count_occupied_neighbors(
                row_i, col_i);
            new_layout.get_rows_mut()[row_i][col_i] =
                T::get_new_seat_pos(occupied_neighbors, *seat_pos);
        }
    }
    new_layout
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbor() {
        let sl = SeatLayoutRuleOne{rows: SeatLayoutRuleOne::get_rows_from_str(
            "LL#\n\
             LLL\n\
             LL.\n"
        )};
        assert!(sl.get(0, 2) == Some(&SeatPosition::Occupied));
        assert!(sl.get(2, 2) == Some(&SeatPosition::Floor));
        assert!(sl.get(0, 0) == Some(&SeatPosition::Empty));
        assert!(sl.get(-1, 1) == None);
    }

    #[test]
    fn test_step() {
        let step0_mini = "##.\n\
                          ###";
        let step1_mini = "#L.\n\
                          #L#";
        let sl_step0_mini = SeatLayoutRuleOne{rows: SeatLayoutRuleOne::get_rows_from_str(step0_mini)};
        let sl_step1_mini = SeatLayoutRuleOne{rows: SeatLayoutRuleOne::get_rows_from_str(step1_mini)};
        println!("step0_mini:\n{}", sl_step0_mini.to_string());
        println!("step1_mini:\n{}", step(&sl_step0_mini).to_string());

        assert!(step(&sl_step0_mini) == sl_step1_mini);

        let step0 = "L.LL.LL.LL\n\
                     LLLLLLL.LL\n\
                     L.L.L..L..\n\
                     LLLL.LL.LL\n\
                     L.LL.LL.LL\n\
                     L.LLLLL.LL\n\
                     ..L.L.....\n\
                     LLLLLLLLLL\n\
                     L.LLLLLL.L\n\
                     L.LLLLL.LL";
        let step1 = "#.##.##.##\n\
                     #######.##\n\
                     #.#.#..#..\n\
                     ####.##.##\n\
                     #.##.##.##\n\
                     #.#####.##\n\
                     ..#.#.....\n\
                     ##########\n\
                     #.######.#\n\
                     #.#####.##";
        let step2 = "#.LL.L#.##\n\
                     #LLLLLL.L#\n\
                     L.L.L..L..\n\
                     #LLL.LL.L#\n\
                     #.LL.LL.LL\n\
                     #.LLLL#.##\n\
                     ..L.L.....\n\
                     #LLLLLLLL#\n\
                     #.LLLLLL.L\n\
                     #.#LLLL.##";
        let sl_step0 = SeatLayoutRuleOne{rows: SeatLayoutRuleOne::get_rows_from_str(step0)};
        let sl_step1 = SeatLayoutRuleOne{rows: SeatLayoutRuleOne::get_rows_from_str(step1)};
        let sl_step2 = SeatLayoutRuleOne{rows: SeatLayoutRuleOne::get_rows_from_str(step2)};
        println!("step1:\n{}", step(&sl_step0).to_string());
        assert!(step(&sl_step0) == sl_step1);
        println!("step2:\n{}", step(&sl_step1).to_string());
        assert!(step(&sl_step1) == sl_step2);
    }
}
