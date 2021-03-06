use std::fs;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Position {
    number: u32,
    // 'marked' means that this position has been marked as a drawn number already
    marked: bool,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Board {
    values: Vec<Position>,
    index: usize,
}

impl Board {
    fn mark_number(self, number: u32) -> Board {
        Board {
            values: self
                .values
                .iter()
                .map(|position| match number == position.number {
                    true => Position {
                        marked: true,
                        ..(*position)
                    },
                    false => Position { ..(*position) },
                })
                .collect(),
            ..self
        }
    }

    fn mark_number_pt2(self, number: u32, index: usize) -> Board {
        if is_winning_board(&self) {
            return self;
        }
        let new_board = Board {
            values: self
                .values
                .iter()
                .map(|position| match number == position.number {
                    true => Position {
                        marked: true,
                        ..(*position)
                    },
                    false => Position { ..(*position) },
                })
                .collect(),
            ..self
        };
        if is_winning_board(&new_board) && self.index == 0 {
            Board { index, ..new_board }
        } else {
            new_board
        }
    }
}

fn get_data() -> (Vec<u32>, Vec<Board>) {
    let raw_string =
        fs::read_to_string("src/data/day_4.txt").expect("some problem reading the file! :O");

    let (raw_moves, raw_boards) = raw_string
        .split_once("\n\n")
        .expect("this should split just fine");

    let moves = raw_moves
        .split(",")
        .map(|numeral| numeral.parse().unwrap())
        .collect();

    let boards = raw_boards
        .split("\n\n")
        .map(|raw_board| Board {
            values: raw_board
                .split("\n")
                .flat_map(|row| row.split(" "))
                .filter(|&str| str != "")
                .map(|numeral| Position {
                    number: numeral.parse().unwrap(),
                    marked: false,
                })
                .collect(),
            index: 0,
        })
        .collect();
    (moves, boards)
}

fn mark_boards(boards: Vec<Board>, number: u32) -> Vec<Board> {
    boards
        .iter()
        .map(|board| (*board).clone().mark_number(number))
        .collect()
}

fn mark_boards_pt2(boards: Vec<Board>, number: u32, index: usize) -> Vec<Board> {
    boards
        .iter()
        .map(|board| (*board).clone().mark_number_pt2(number, index))
        .collect()
}

fn is_winning_board(board: &Board) -> bool {
    // If all numbers in any row or any column of a board are marked, that board wins. (Diagonals
    // don't count.)
    //
    // Here are the indices in a board:
    //
    // 0  1  2  3  4
    // 5  6  7  8  9
    // 10 11 12 13 14
    // 15 16 17 18 19
    // 20 21 22 23 24
    //
    // Any vertical or horizontal set is a winning set of indices.
    let winning_indices: Vec<Vec<usize>> = vec![
        (0..5).collect(),
        (5..10).collect(),
        (10..15).collect(),
        (15..20).collect(),
        (20..25).collect(),
        vec![0, 5, 10, 15, 20],
        vec![1, 6, 11, 16, 21],
        vec![2, 7, 12, 17, 22],
        vec![3, 8, 13, 18, 23],
        vec![4, 9, 14, 19, 24],
    ];

    winning_indices
        .iter()
        .any(|indices| indices.iter().all(|&index| board.values[index].marked))
}

fn get_score_for_board(board: &Board, number: u32) -> u32 {
    // The score of the winning board can now be calculated. Start by finding the sum of
    // all unmarked numbers on that board; in this case, the sum is 188. Then, multiply
    // that sum by the number that was just called when the board won, 24, to get the final
    // score, 188 * 24 = 4512.
    let score: u32 = board
        .values
        .iter()
        .fold(0, |acc, position| match position.marked {
            true => acc,
            false => acc + position.number,
        });
    return score * number;
}

pub fn part_one() -> u32 {
    let (moves, mut boards) = get_data();

    for number in moves {
        boards = mark_boards(boards, number);
        let winning_boards: Vec<&Board> = boards
            .iter()
            .filter(|board| is_winning_board(board))
            .collect();

        if winning_boards.len() > 0 {
            let winning_board = winning_boards[0];
            return get_score_for_board(winning_board, number);
        }
    }

    // for 'type safety'
    32
}

pub fn part_two() -> u32 {
    let (moves, mut boards) = get_data();
    for (index, number) in moves.iter().enumerate() {
        boards = mark_boards_pt2(boards, *number, index);
    }
    boards.sort_by(|a, b| a.index.cmp(&b.index));
    let last_winning_board = boards.last().unwrap();
    get_score_for_board(last_winning_board, moves[last_winning_board.index])
}
