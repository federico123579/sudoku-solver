use std::borrow::Borrow;

use ndarray::prelude::*;

use crate::Board;

#[derive(Clone, Copy, Debug)]
enum Guess {
    Prior(u8),
    Hit(u8),
    Match([bool; 9]),
}

impl Guess {
    fn check(&self, number: &u8) -> bool {
        match self {
            Guess::Hit(n) | Guess::Prior(n) => n == number,
            Guess::Match(arr) => arr[*number as usize],
        }
    }

    fn can_be_resolved(&self) -> bool {
        match self {
            Guess::Hit(_) | Guess::Prior(_) => false,
            Guess::Match(arr) => arr.map(|x| if x { 1 } else { 0 }).iter().sum::<usize>() == 1,
        }
    }

    /// this one reolve a Mathc into an Hit if only one number is matched
    fn resolve(self) -> Self {
        match self {
            Guess::Hit(_) | Guess::Prior(_) => panic!("this should not occur!"),
            Guess::Match(arr) => Self::Hit(
                (arr.iter().position(|&x| x).unwrap() + 1)
                    .try_into()
                    .unwrap(),
            ),
        }
    }

    fn exclude(&mut self, ix: usize) {
        match self {
            Guess::Match(arr) => arr[ix] = false,
            _ => panic!("this should not occur"),
        }
    }
}

struct BoardSolver {
    board_to_solve: Board,
    solving_board: Board,
    guess_board: Array2<Guess>,
}

impl BoardSolver {
    fn get_dim(&self) -> (usize, usize) {
        self.guess_board.dim()
    }

    fn from_board(board: Board) -> Self {
        let multiple_match: [bool; 9] = [true; 9];
        let mut guess_array: Array2<Guess> =
            Array2::from_elem((9, 9), Guess::Match(multiple_match));
        let (rows, cols) = board.values.dim();
        for i in 0..rows {
            for j in 0..cols {
                match board.values[[i, j]] {
                    0 => guess_array[[i, j]] = Guess::Match(multiple_match),
                    el @ 1..=9 => guess_array[[i, j]] = Guess::Prior(el),
                    _ => panic!("number higher than 9 are not accepted!"),
                };
            }
        }
        Self {
            solving_board: board.clone(),
            board_to_solve: board,
            guess_board: guess_array,
        }
    }

    fn apply_hits(&mut self) {
        let (rows, cols) = self.get_dim();
        for i in 0..rows {
            for j in 0..cols {
                match self.guess_board[[i, j]] {
                    Guess::Hit(n) => self.solving_board.values[[i, j]] = n,
                    _ => (),
                };
            }
        }
    }

    fn can_contain(&self, cell_position: (usize, usize), cell_value: u8) -> bool {
        let values = &self.solving_board.values;
        let check_row = values.row(cell_position.0).iter().all(|&x| x != cell_value);
        let check_col = values
            .column(cell_position.1)
            .iter()
            .all(|&x| x != cell_value);
        let square_row = (cell_position.0 / 3) * 3;
        let square_col = (cell_position.1 / 3) * 3;
        let check_square = values
            .slice(s![square_row..square_row + 3, square_col..square_col + 3])
            .iter()
            .all(|&x| x != cell_value);
        if check_row && check_col && check_square {
            true
        } else {
            false
        }
    }

    fn exclude_matches(&mut self, row: usize, col: usize) {
        if let Guess::Match(arr) = &self.guess_board[[row, col]] {
            for (i, is_a_match) in arr.to_owned().iter().enumerate() {
                if *is_a_match && !self.can_contain((row, col), (i + 1) as u8) {
                    self.guess_board[[row, col]].exclude(i);
                }
            }
            self.guess_board[[row, col]] = self.guess_board[[row, col]].resolve();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_exlcude_matches_from_file(file_path: &str) {
        let board = Board::from_file(file_path);
        board.print_simple();
        let mut solver = BoardSolver::from_board(board);
        let (row, col) = solver.get_dim();
        for i in 0..row {
            for j in 0..col {
                solver.exclude_matches(i, j);
            }
        }
        solver.apply_hits();
        println!();
        solver.solving_board.print_simple();
        assert_ne!(solver.board_to_solve.values, solver.solving_board.values);
        assert!(solver.solving_board.check_complete());
    }

    #[test]
    fn test_exlcude_matches_on_row() {
        test_exlcude_matches_from_file("tests/guess_row.txt")
    }

    #[test]
    fn test_exlcude_matches_on_column() {
        test_exlcude_matches_from_file("tests/guess_column.txt")
    }

    #[test]
    fn test_exlcude_matches_on_square() {
        test_exlcude_matches_from_file("tests/guess_square.txt")
    }
}
