use ndarray::prelude::*;

use crate::Board;

#[derive(Debug, Clone)]
struct GuessWrongError {
    pos: (usize, usize),
    wrong_guess_ix: usize,
}

#[derive(Debug, Clone)]
struct UnsolvableError;

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

    fn first_match_ix(&self) -> usize {
        match self {
            Guess::Hit(_) | Guess::Prior(_) => {
                panic!("Cannot get first match index from Guess::Hit or Guess::Prior")
            }
            Guess::Match(arr) => arr.iter().position(|x| *x).unwrap(),
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

    fn is_unsolvable(&self) -> bool {
        match self {
            Guess::Hit(_) | Guess::Prior(_) => false,
            Guess::Match(arr) => arr.iter().all(|x| !x),
        }
    }
}

#[derive(Clone)]
struct BoardSolver {
    board_to_solve: Board,
    solving_board: Board,
    guess_board: Array2<Guess>,
}

impl BoardSolver {
    fn from_board(board: Board) -> Self {
        let multiple_match: [bool; 9] = [true; 9];
        let mut guess_array: Array2<Guess> =
            Array2::from_elem((9, 9), Guess::Match(multiple_match));
        for i in 0..9 {
            for j in 0..9 {
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

    /// apply_hits will return the number of hits applied
    fn apply_hits(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                if let Guess::Hit(n) = self.guess_board[[i, j]] {
                    self.solving_board.values[[i, j]] = n;
                }
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

    fn exclude_matches(&mut self, row: usize, col: usize) -> Result<i32, UnsolvableError> {
        let mut counter = 0;
        if let Guess::Match(arr) = &self.guess_board[[row, col]] {
            let mut guess = self.guess_board[[row, col]];
            for (i, is_a_match) in arr.to_owned().iter().enumerate() {
                if *is_a_match && !self.can_contain((row, col), (i + 1) as u8) {
                    guess.exclude(i);
                    counter += 1;
                }
            }
            if guess.can_be_resolved() {
                self.guess_board[[row, col]] = guess.resolve();
            } else {
                self.guess_board[[row, col]] = guess;
            }
            if guess.is_unsolvable() {
                return Err(UnsolvableError);
            }
        }
        Ok(counter)
    }

    fn simulate_path(&mut self) -> Result<Board, GuessWrongError> {
        let mut new_solver = self.clone();

        // look for the first mathc with minimun guesses
        let mut record: (usize, usize) = (9, 0);
        for i in 0..81 {
            if let Guess::Match(arr) = &mut new_solver.guess_board[[i / 9, i % 9]] {
                let possible_matches = arr.iter().filter(|&x| *x).count();
                if record.0 > possible_matches {
                    record = (possible_matches, i);
                }
            }
        }
        // apply a the first hit on the most likely match
        let first_match_ix = new_solver.guess_board[[record.1 / 9, record.1 % 9]].first_match_ix();
        new_solver.guess_board[[record.1 / 9, record.1 % 9]] =
            Guess::Hit((first_match_ix + 1) as u8);
        new_solver.apply_hits();

        match new_solver.solve() {
            Ok(solved_board) => Ok(solved_board),
            Err(_) => Err(GuessWrongError {
                pos: (record.1 / 9, record.1 % 9),
                wrong_guess_ix: first_match_ix,
            }),
        }
    }

    fn solve(mut self) -> Result<Board, UnsolvableError> {
        // flags
        let mut can_be_reduced: bool = true;

        while !self.solving_board.is_complete() {
            let mut counter = 0;
            if can_be_reduced {
                for i in 0..9 {
                    for j in 0..9 {
                        counter += self.exclude_matches(i, j)?;
                    }
                }
                self.apply_hits();
                if counter == 0 {
                    can_be_reduced = false;
                }
            } else {
                match self.simulate_path() {
                    Ok(board) => {
                        self.solving_board = board;
                        break;
                    }
                    Err(e) => {
                        self.guess_board[[e.pos.0, e.pos.1]].exclude(e.wrong_guess_ix);
                        can_be_reduced = true;
                    }
                }
            }
        }

        if self.solving_board.check_complete() {
            Ok(self.solving_board)
        } else {
            Err(UnsolvableError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod complete {
        use super::*;

        macro_rules! test_board {
            ($test_name: ident, $file_num: expr) => {
                #[test]
                fn $test_name() {
                    let board_to_solve = Board::from_file(
                        format!(
                            "{}/boards/complete/quiz-{:0>2}.txt",
                            env!("CARGO_MANIFEST_DIR"),
                            $file_num
                        )
                        .as_str(),
                    );

                    println!("board to solve:");
                    board_to_solve.print_simple();

                    let expected_board = Board::from_file(
                        format!(
                            "{}/boards/complete/solution-{:0>2}.txt",
                            env!("CARGO_MANIFEST_DIR"),
                            $file_num
                        )
                        .as_str(),
                    );
                    let solver = BoardSolver::from_board(board_to_solve);
                    let solved_board = solver.solve().unwrap();
                    assert!(solved_board.check_complete());

                    println!("solved board:");
                    solved_board.print_simple();

                    assert_eq!(solved_board.values, expected_board.values);
                }
            };
        }

        test_board!(test_00, 0);
        test_board!(test_01, 1);
        test_board!(test_02, 2);
        test_board!(test_03, 3);
        test_board!(test_04, 4);
        test_board!(test_05, 5);
        test_board!(test_06, 6);
        test_board!(test_07, 7);
        test_board!(test_08, 8);
        test_board!(test_09, 9);

        // for these the else of the main loop will be taken
        test_board!(test_hard_00, 10);
        test_board!(test_hard_01, 11);
    }

    fn test_exlcude_matches_from_file(file_path: &str) {
        let board = Board::from_file(file_path);
        board.print_simple();
        let mut solver = BoardSolver::from_board(board);
        for i in 0..9 {
            for j in 0..9 {
                solver.exclude_matches(i, j).unwrap();
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
