extern crate regex;

use regex::Regex;

pub fn run_day05(puzzle_input: &str) {
    let input: String = String::from(
        std::fs::read_to_string(&puzzle_input)
            .expect(format!(
                "Could not read file \"{}\".",
                &puzzle_input
            ).as_str())
            .trim()
    );
    let mut field = Field::new(&input);
    let num_intersections_hor_or_vert = field.evaluate_intersections(true);
    println!("Number of intersections of horizontal or vertical lines: {}", num_intersections_hor_or_vert);

    let mut field = Field::new(&input);
    let num_intersections = field.evaluate_intersections(false);
    println!("Number of intersections total: {}", num_intersections);

}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn new(s: &str) -> Line {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(\d+),(\d+) -> (\d+),(\d+)"
            ).unwrap();
        }
        let caps = RE.captures(s).expect(
            &format!("No line found in '{}'", s)
        );
        Line {
            // caps[0] is the full string
            start: Point {
                x: caps[1].parse::<usize>()
                    .expect(format!("Invalid number in captures: {:?}", caps).as_str()),
                y: caps[2].parse::<usize>()
                    .expect(format!("Invalid number in captures: {:?}", caps).as_str()),
            },
            end: Point {
                x: caps[3].parse::<usize>()
                    .expect(format!("Invalid number in captures: {:?}", caps).as_str()),
                y: caps[4].parse::<usize>()
                    .expect(format!("Invalid number in captures: {:?}", caps).as_str()),
            }
        }
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

struct Field {
    cells: Vec<usize>,
    lines: Vec<Line>,
    width: usize,
    height: usize
}

impl Field {
    fn new(input: &str) -> Field {
        let mut lines: Vec<Line> = Vec::new();
        for text_line in input.lines() {
            lines.push(Line::new(text_line));
        }
        let width: usize = std::cmp::max(
            lines.iter().max_by_key(|line| line.start.x).unwrap().start.x,
            lines.iter().max_by_key(|line| line.end.x).unwrap().end.x
        ) + 1;
        let height: usize = std::cmp::max(
            lines.iter().max_by_key(|line| line.start.y).unwrap().start.y,
            lines.iter().max_by_key(|line| line.end.y).unwrap().end.y
        ) + 1;

        let cells: Vec<usize> = vec![0; width * height];
        Field {
            cells,
            lines,
            width,
            height
        }
    }

    fn evaluate_intersections(&mut self, exclude_diagonal: bool) -> usize {
        Self::evaluate_intersections_(&self.lines, &mut self.cells, self.width, exclude_diagonal)
    }

    fn evaluate_intersections_(lines: &[Line], cells: &mut [usize], width: usize, exclude_diagonal: bool) -> usize {
        for line in lines {
            let start_x = std::cmp::min(line.start.x, line.end.x);
            let end_x = std::cmp::max(line.start.x, line.end.x);
            if line.is_horizontal_or_vertical() {
                let start_y = std::cmp::min(line.start.y, line.end.y);
                let end_y = std::cmp::max(line.start.y, line.end.y);
                // The following would be a rectangle if we didn't
                // already know that we're only looking at orthogonal
                // lines.
                for x in start_x..end_x+1 {
                    for y in start_y..end_y+1 {
                        let prev = cells.get(y * width + x).expect(
                            format!(
                                "({}, {}) out of bounds for width {} and {} cells",
                                x, y, width, cells.len()
                            ).as_str()
                        );
                        cells[y * width + x] = prev + 1;
                    }
                }
            } else if !exclude_diagonal {
                let slope = (line.end.y as i64 - line.start.y as i64) / (line.end.x as i64 - line.start.x as i64);
                let start_y = if slope > 0 { std::cmp::min(line.start.y, line.end.y) } else { std::cmp::max(line.start.y, line.end.y) };
                for (i, x) in (start_x..end_x+1).enumerate() {
                    // We know that diagonal lines will always be at 45 degree angles.
                    let y = if slope > 0 { start_y + i } else { start_y - i };
                    let prev = cells.get(y * width + x).unwrap();
                    cells[y * width + x] = prev + 1;
                }
            }
        }

        cells.iter().fold(0, |acc, cell| {
            if *cell > 1 { acc + 1 } else { acc }
        })
    }
}
