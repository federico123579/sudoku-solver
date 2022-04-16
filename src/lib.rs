use ndarray::prelude::*;

pub struct Board {
    board: Array2<u8>,
}

impl Board {
    pub fn init() -> Self {
        let board = Array2::from_elem((9, 9), 0_u8);
        Board { board }
    }

    pub fn new(values: Array2<u8>) -> Self {
        Board { board: values }
    }

    fn check_row(&self, row: usize) -> bool {
        let mut elem_checked: [bool; 9] = [false; 9];
        for elem in self.board.slice(s![row, ..]).iter() {
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
        for elem in self.board.slice(s![.., col]).iter() {
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
                let elem = self.board[(row_start + i, col_start + j)];
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

    // print utility functions

    pub fn print_simple(&self) {
        for i in 0..9 {
            for j in 0..9 {
                print!("{} ", self.board[(i, j)]);
            }
            println!();
        }
    }

    pub fn print_complete(&self) {
        for i in 0..10 {
            match i {
                0 => println!("┌───────┬───────┬───────┐"),
                3 | 6 => println!("├───────┼───────┼───────┤"),
                // last line
                9 => {
                    println!("└───────┴───────┴───────┘");
                    break;
                }
                _ => (),
            }
            for j in 0..10 {
                match j {
                    0 | 3 | 6 => print!("│ "),
                    // last column
                    9 => {
                        print!("│");
                        break;
                    }
                    _ => (),
                }
                print!("{} ", self.board[(i, j)])
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_valid_board() -> Board {
        Board::new(array![
            [1, 4, 7, 2, 5, 8, 3, 6, 9],
            [2, 5, 8, 3, 6, 9, 4, 7, 1],
            [3, 6, 9, 4, 7, 1, 5, 8, 2],
            [4, 7, 1, 5, 8, 2, 6, 9, 3],
            [5, 8, 2, 6, 9, 3, 7, 1, 4],
            [6, 9, 3, 7, 1, 4, 8, 2, 5],
            [7, 1, 4, 8, 2, 5, 9, 3, 6],
            [8, 2, 5, 9, 3, 6, 1, 4, 7],
            [9, 3, 6, 1, 4, 7, 2, 5, 8]
        ])
    }

    // test creational functions
    #[test]
    fn test_new() {
        let board = Board::new(array![
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1]
        ]);
        board.print_simple();
    }

    // test check functions
    #[test]
    fn test_check_row() {
        let valid_board = init_valid_board();
        let empty_board = Board::init();
        for i in 0..9 {
            assert!(valid_board.check_row(i));
            assert!(empty_board.check_row(i));
        }

        // two 1s in the first row
        let invalid_board = Board::new(array![
            [1, 4, 7, 2, 5, 1, 3, 6, 9],
            [2, 5, 8, 3, 6, 9, 4, 7, 1],
            [3, 6, 9, 4, 7, 0, 5, 8, 2],
            [4, 7, 1, 5, 8, 2, 6, 9, 3],
            [5, 8, 2, 6, 9, 3, 7, 1, 4],
            [6, 9, 3, 7, 1, 4, 8, 2, 5],
            [7, 1, 4, 8, 2, 5, 9, 3, 6],
            [8, 2, 5, 9, 3, 6, 1, 4, 7],
            [9, 3, 6, 1, 4, 7, 2, 5, 8]
        ]);
        assert!(!invalid_board.check_row(0));
        for i in 1..9 {
            assert!(valid_board.check_row(i));
        }
    }

    #[test]
    fn test_check_column() {
        let valid_board = init_valid_board();
        let empty_board = Board::init();
        for i in 0..9 {
            assert!(valid_board.check_column(i));
            assert!(empty_board.check_column(i));
        }

        // two 1s in the first column
        let invalid_board = Board::new(array![
            [1, 4, 7, 2, 5, 8, 3, 6, 9],
            [2, 5, 8, 3, 6, 9, 4, 7, 1],
            [3, 6, 9, 4, 7, 1, 5, 8, 2],
            [1, 7, 0, 5, 8, 2, 6, 9, 3],
            [5, 8, 2, 6, 9, 3, 7, 1, 4],
            [6, 9, 3, 7, 1, 4, 8, 2, 5],
            [7, 1, 4, 8, 2, 5, 9, 3, 6],
            [8, 2, 5, 9, 3, 6, 1, 4, 7],
            [9, 3, 6, 1, 4, 7, 2, 5, 8]
        ]);
        assert!(!invalid_board.check_column(0));
        for i in 1..9 {
            assert!(valid_board.check_column(i));
        }
    }

    #[test]
    fn check_square() {
        let valid_board = init_valid_board();
        let empty_board = Board::init();
        for i in 0..3 {
            for j in 0..3 {
                assert!(valid_board.check_square(i, j));
                assert!(empty_board.check_square(i, j));
            }
        }

        // two 1s in the first square
        let invalid_board = Board::new(array![
            [1, 4, 7, 2, 5, 8, 3, 6, 9],
            [2, 5, 8, 3, 6, 9, 4, 7, 1],
            [3, 6, 1, 4, 7, 0, 5, 8, 2],
            [4, 7, 0, 5, 8, 2, 6, 9, 3],
            [5, 8, 2, 6, 9, 3, 7, 1, 4],
            [6, 9, 3, 7, 1, 4, 8, 2, 5],
            [7, 1, 4, 8, 2, 5, 9, 3, 6],
            [8, 2, 5, 9, 3, 6, 1, 4, 7],
            [9, 3, 6, 1, 4, 7, 2, 5, 8]
        ]);
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
        let empty_board = Board::init();
        assert!(valid_board.check_complete());
        assert!(empty_board.check_complete());

        // two 1s in the first row
        let invalid_board = Board::new(array![
            [1, 4, 7, 2, 5, 1, 3, 6, 9],
            [2, 5, 8, 3, 6, 9, 4, 7, 1],
            [3, 6, 9, 4, 7, 0, 5, 8, 2],
            [4, 7, 1, 5, 8, 2, 6, 9, 3],
            [5, 8, 2, 6, 9, 3, 7, 1, 4],
            [6, 9, 3, 7, 1, 4, 8, 2, 5],
            [7, 1, 4, 8, 2, 5, 9, 3, 6],
            [8, 2, 5, 9, 3, 6, 1, 4, 7],
            [9, 3, 6, 1, 4, 7, 2, 5, 8]
        ]);
        assert!(!invalid_board.check_complete());

        // two 1s in the first column
        let invalid_board = Board::new(array![
            [1, 4, 7, 2, 5, 8, 3, 6, 9],
            [2, 5, 8, 3, 6, 9, 4, 7, 1],
            [3, 6, 9, 4, 7, 1, 5, 8, 2],
            [4, 7, 1, 5, 8, 2, 6, 9, 3],
            [1, 8, 2, 6, 9, 3, 7, 0, 4],
            [6, 9, 3, 7, 1, 4, 8, 2, 5],
            [7, 1, 4, 8, 2, 5, 9, 3, 6],
            [8, 2, 5, 9, 3, 6, 1, 4, 7],
            [9, 3, 6, 1, 4, 7, 2, 5, 8]
        ]);
        assert!(!invalid_board.check_complete());

        // two 1s in the first square
        let invalid_board = Board::new(array![
            [1, 4, 7, 2, 5, 8, 3, 6, 9],
            [2, 5, 8, 3, 6, 9, 4, 7, 1],
            [3, 6, 1, 4, 7, 0, 5, 8, 2],
            [4, 7, 0, 5, 8, 2, 6, 9, 3],
            [5, 8, 2, 6, 9, 3, 7, 1, 4],
            [6, 9, 3, 7, 1, 4, 8, 2, 5],
            [7, 1, 4, 8, 2, 5, 9, 3, 6],
            [8, 2, 5, 9, 3, 6, 1, 4, 7],
            [9, 3, 6, 1, 4, 7, 2, 5, 8]
        ]);
        assert!(!invalid_board.check_complete());
    }
}
