mod cli;
mod guess;
mod utils;

use ndarray::prelude::*;
use std::{fs::File, io::Read};

#[derive(Clone)]
pub struct Board {
    values: Array2<u8>,
}

impl Board {
    pub fn empty() -> Self {
        let board = Array2::from_elem((9, 9), 0_u8);
        Board { values: board }
    }

    pub fn new(values: Array2<u8>) -> Self {
        Board { values }
    }

    pub fn from_file(path: &str) -> Self {
        let mut board = Array2::from_elem((9, 9), 0_u8);
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut row = 0;
        for line in contents.lines() {
            let mut col = 0;
            for c in line.chars() {
                if c.is_digit(10) {
                    board[[row, col]] = c.to_digit(10).unwrap() as u8;
                }
                col += 1;
            }
            row += 1;
        }
        Board { values: board }
    }

    pub fn from_board_dir(path: &str) -> Self {
        let board_path = utils::get_board_dir().unwrap();
        Self::from_file(board_path.as_path().join(path).to_str().unwrap())
    }

    fn check_row(&self, row: usize) -> bool {
        let mut elem_checked: [bool; 9] = [false; 9];
        for elem in self.values.slice(s![row, ..]).iter() {
            if *elem == 0 {
                continue;
            } else if !elem_checked[*elem as usize - 1] {
                elem_checked[*elem as usize - 1] = true;
            } else {
                return false;
            }
        }
        true
    }

    fn check_column(&self, col: usize) -> bool {
        let mut elem_checked: [bool; 9] = [false; 9];
        for elem in self.values.slice(s![.., col]).iter() {
            if *elem == 0 {
                continue;
            } else if !elem_checked[*elem as usize - 1] {
                elem_checked[*elem as usize - 1] = true;
            } else {
                return false;
            }
        }
        true
    }

    fn check_square(&self, row: usize, col: usize) -> bool {
        let mut elem_checked: [bool; 9] = [false; 9];
        let row_start = row * 3;
        let col_start = col * 3;
        for i in 0..3 {
            for j in 0..3 {
                let elem = self.values[(row_start + i, col_start + j)];
                if elem == 0 {
                    continue;
                } else if !elem_checked[elem as usize - 1] {
                    elem_checked[elem as usize - 1] = true;
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn check_complete(&self) -> bool {
        for i in 0..9 {
            if !self.check_row(i) {
                return false;
            }
            if !self.check_column(i) {
                return false;
            }
            if !self.check_square(i / 3, i % 3) {
                return false;
            }
        }
        true
    }

    fn is_complete(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self.values[(i, j)] == 0 {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_valid_board() -> Board {
        Board::from_board_dir("tests/valid_board.txt")
    }

    // test creational functions
    #[test]
    fn test_new() {
        let board = Board::new(Array2::from_elem((9, 9), 1_u8));
        board.print_simple();
    }

    // test read from file
    #[test]
    fn test_from_file() {
        let file_board = Board::from_board_dir("empty_board.txt");
        let empty_board = Board::empty();
        assert_eq!(file_board.values, empty_board.values);
    }

    // test check functions
    #[test]
    fn test_check_row() {
        let valid_board = init_valid_board();
        let empty_board = Board::empty();
        for i in 0..9 {
            assert!(valid_board.check_row(i));
            assert!(empty_board.check_row(i));
        }

        // two 1s in the first row
        let invalid_board = Board::from_board_dir("tests/two_ones_in_a_row.txt");
        assert!(!invalid_board.check_row(0));
        for i in 1..9 {
            assert!(valid_board.check_row(i));
        }
    }

    #[test]
    fn test_check_column() {
        let valid_board = init_valid_board();
        let empty_board = Board::empty();
        for i in 0..9 {
            assert!(valid_board.check_column(i));
            assert!(empty_board.check_column(i));
        }

        // two 1s in the first column
        let invalid_board = Board::from_board_dir("tests/two_ones_in_a_col.txt");
        assert!(!invalid_board.check_column(0));
        for i in 1..9 {
            assert!(valid_board.check_column(i));
        }
    }

    #[test]
    fn check_square() {
        let valid_board = init_valid_board();
        let empty_board = Board::empty();
        for i in 0..3 {
            for j in 0..3 {
                assert!(valid_board.check_square(i, j));
                assert!(empty_board.check_square(i, j));
            }
        }

        // two 1s in the first square
        let invalid_board = Board::from_board_dir("tests/two_ones_in_a_square.txt");
        assert!(!invalid_board.check_square(0, 0));
        for i in 1..3 {
            for j in 1..3 {
                assert!(valid_board.check_square(i, j));
            }
        }
    }

    #[test]
    fn test_check_complete() {
        let valid_board = init_valid_board();
        let empty_board = Board::empty();
        assert!(valid_board.check_complete());
        assert!(empty_board.check_complete());

        // two 1s in the first row
        let invalid_board = Board::from_board_dir("tests/two_ones_in_a_row.txt");
        assert!(!invalid_board.check_complete());

        // two 1s in the first column
        let invalid_board = Board::from_board_dir("tests/two_ones_in_a_col.txt");
        assert!(!invalid_board.check_complete());

        // two 1s in the first square
        let invalid_board = Board::from_board_dir("tests/two_ones_in_a_square.txt");
        assert!(!invalid_board.check_complete());
    }
}
