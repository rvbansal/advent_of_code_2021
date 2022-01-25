#[derive(PartialEq)]
enum BoardItem {
    Called,
    Uncalled(usize),
}

impl Default for BoardItem {
    fn default() -> Self {
        BoardItem::Uncalled(0)
    }
}

use BoardItem::*;

struct Board([[BoardItem; 5]; 5]);

impl Board {
    fn from_str(input: &str) -> Self {
        let mut board: [[BoardItem; 5]; 5] = Default::default();
        for (x, line) in input.split('\n').enumerate() {
            let numbers = line
                .trim()
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap());
            for (y, number) in numbers.enumerate() {
                board[x][y] = Uncalled(number)
            }
        }
        Board(board)
    }

    fn check_bingo(&self) -> bool {
        for x in 0..5 {
            if (0..5).all(|y| self.0[x][y] == Called) {
                return true;
            }
        }

        for y in 0..5 {
            if (0..5).all(|x| self.0[x][y] == Called) {
                return true;
            }
        }
        false
    }

    fn mark_number(&mut self, number: usize) {
        for x in 0..5 {
            for y in 0..5 {
                if self.0[x][y] == Uncalled(number) {
                    self.0[x][y] = Called;
                }
            }
        }
    }

    fn sum_unmarked_numbers(&self) -> usize {
        let rows = &self.0;
        rows.iter()
            .map(|row| {
                row.iter()
                    .map(|item| match item {
                        Called => 0,
                        Uncalled(v) => *v,
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn print(&self) {
        for row in &self.0 {
            println!(
                "{}",
                row.iter()
                    .map(|v| {
                        format!(
                            "{}",
                            match v {
                                Called => "XX".to_string(),
                                Uncalled(v) => format!("{:2}", v),
                            }
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
    }
}

fn play_squid(input: &str) -> usize {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let picked_numbers = parts[0]
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = parts[1..]
        .iter()
        .map(|b| Board::from_str(b))
        .collect::<Vec<Board>>();

    for number in picked_numbers {
        for board in boards.iter_mut() {
            board.mark_number(number)
        }

        for i in 0..boards.len() {
            let board = boards.get(i).unwrap();
            if board.check_bingo() {
                return board.sum_unmarked_numbers() * number;
            }
        }
    }

    panic!("No bingo found.")
}

fn let_squid_win(input: &str) -> usize {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let picked_numbers = parts[0]
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = parts[1..]
        .iter()
        .map(|b| Board::from_str(b))
        .collect::<Vec<Board>>();

    for number in picked_numbers {
        for board in boards.iter_mut() {
            board.mark_number(number)
        }

        if boards.len() == 1 {
            if boards[0].check_bingo() {
                return boards[0].sum_unmarked_numbers() * number;
            }
        } else {
            boards.retain(|b| !b.check_bingo())
        }
    }

    panic!("No bingo found.")
}

fn main() {
    println!(
        "Play vs squid result: {}",
        play_squid(include_str!("input.txt"))
    );
    println!(
        "Let squid win result: {}",
        let_squid_win(include_str!("input.txt"))
    );
}
