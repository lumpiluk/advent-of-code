use std::error::Error;
use std::collections::HashSet;

pub fn run_day04(puzzle_input: &str) {
    let input: String = String::from(
        std::fs::read_to_string(&puzzle_input)
            .expect(format!(
                "Could not read file \"{}\".",
                &puzzle_input
            ).as_str())
            .trim()
    );
    match BingoGame::new(&input) {
        Ok(game) => {
            let (earliest_winning_round, earliest_board_id, earliest_score) = game
                .find_earliest_winning_board();
            println!("earliest round: {}, earliest board: {}, score: {}",
                     earliest_winning_round, earliest_board_id.unwrap(), earliest_score.unwrap());

            let (latest_winning_round, latest_board_id, latest_score) = game
                .find_latest_winning_board();
            println!("latest round: {}, latest board: {}, score: {}",
                     latest_winning_round, latest_board_id.unwrap(), latest_score.unwrap());

        },
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

struct BingoGame {
    boards: Vec<BingoBoard>,
    drawn_numbers: Vec<usize>
}

impl BingoGame {
    fn new(input: &str) -> Result<BingoGame, Box<dyn Error>> {
        let drawn_numbers = input.split("\n").next()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No first line to parse the drawn numbers from."
            ))?
            .split(",")
            .map(|s| s.parse::<usize>())
            .collect::<Result<_, _>>()?;
        let boards: Vec<BingoBoard> = input.split("\n\n")
            .skip(1) // We already parsed the first line.
            .map(|block| BingoBoard::new(block))
            .collect::<Result<_, _>>()?;
        Ok(BingoGame {
            boards,
            drawn_numbers
        })
    }

    fn find_earliest_winning_board(&self) -> (usize, Option<usize>, Option<usize>) {
        let mut earliest_board_id: Option<usize> = None;
        let mut earliest_winning_round: usize = usize::MAX;
        let mut earliest_score: Option<usize> = None;
        for (i, board) in self.boards.iter().enumerate() {
            let (rounds, score) = board.rounds_to_victory(&self.drawn_numbers);
            if rounds < earliest_winning_round {
                earliest_winning_round = rounds;
                earliest_score = Some(score);
                earliest_board_id = Some(i);
            }
        }
        (earliest_winning_round, earliest_board_id, earliest_score)
    }

    fn find_latest_winning_board(&self) -> (usize, Option<usize>, Option<usize>) {
        let mut latest_board_id: Option<usize> = None;
        let mut latest_winning_round: usize = 0;
        let mut latest_score: Option<usize> = None;
        for (i, board) in self.boards.iter().enumerate() {
            let (rounds, score) = board.rounds_to_victory(&self.drawn_numbers);
            if rounds > latest_winning_round {
                latest_winning_round = rounds;
                latest_score = Some(score);
                latest_board_id = Some(i);
            }
        }
        (latest_winning_round, latest_board_id, latest_score)
    }
}

struct BingoBoard {
    fields: Vec<usize>,
    width: usize,
    height: usize,
}

impl BingoBoard {
    fn new(input_block: &str) -> Result<BingoBoard, Box<dyn Error>> {
        let width = input_block.lines().next()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Invalid input for BingoBoard:\n{}", input_block)
            ))?
            .split_ascii_whitespace()
            .count();
        let mut height: usize = 0;
        let mut fields: Vec<usize> = Vec::new();
        for line in input_block.lines() {
            height += 1;
            let mut line_vals: Vec<usize> = line.split_ascii_whitespace()
                .map(|s| s.parse::<usize>())
                .collect::<Result<_, _>>()?;
            let row_len: usize = line_vals.len();
            fields.append(&mut line_vals);
            if row_len != width {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Malformed BingoBoard. Expected {} per row, but found {} in this line: {}\n\n:\n{}",
                        width,
                        row_len,
                        line,
                        input_block
                    )
                )));
            }
        }
        Ok(BingoBoard {
            fields,
            width,
            height
        })
    }

    fn get(&self, x: usize, y: usize) -> &usize {
        self.fields.get(y * self.width + x).unwrap()
    }

    fn get_score(&self, drawn_numbers_so_far: &[usize]) -> usize {
        let last_called_number: &usize = drawn_numbers_so_far.last().unwrap();
        let mut drawn_numbers: HashSet<usize> = HashSet::new();
        drawn_numbers.extend(drawn_numbers_so_far);
        let mut score: usize = 0;
        let mut has_won: bool = false;
        // Check rows:
        for row_id in 0..self.height {
            let mut num_matches: usize = 0;
            for col_id in 0..self.width {
                let val = self.get(col_id, row_id);
                if drawn_numbers.contains(val) {
                    num_matches += 1;
                } else {
                    score += val;
                }
            }
            if num_matches == self.width {
                has_won = true;
            }
        }
        // Check cols and don't compute the score a second time:
        for col_id in 0..self.width {
            let mut num_matches: usize = 0;
            for row_id in 0..self.height {
                if drawn_numbers.contains(self.get(col_id, row_id)) {
                    num_matches += 1;
                }
            }
            if num_matches == self.height { has_won = true; }
        }
        if has_won { score * last_called_number } else { 0 }
    }

    fn rounds_to_victory(&self, drawn_numbers: &[usize]) -> (usize, usize) {
        for i in 0..drawn_numbers.len() {
            let score = self.get_score(&drawn_numbers[..i+1]);
            if score > 0 { return (i+1, score); }
        }
        (usize::MAX, 0)
    }
}
