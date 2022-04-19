use ndarray::prelude::*;

use crate::Board;

pub struct Position(usize, usize);

pub enum Guess {
    Single(Position, u8),
    Multiple(Position, Vec<u8>),
}

impl Guess {
    pub fn apply_to(self, board: &mut Board) {
        match self {
            Guess::Single(position, value) => board.set(position, value),
            Guess::Multiple(_, _) => (),
        }
    }
}

impl Board {
    fn set(&mut self, position: Position, value: u8) {
        self.board[[position.0, position.1]] = value;
    }

    fn apply_guess(&mut self, guess: Guess) {
        guess.apply_to(self);
    }

    pub fn guess_row(&self, row_number: usize) -> Vec<Guess> {
        let mut guesses = Vec::new();
        let mut number_presence: [bool; 9] = [false; 9];
        let row = self.board.slice(s![row_number, ..]);

        // check every number present and put them in a sorted array number_presence
        for elem in &row {
            if *elem == 0 {
                continue;
            } else if number_presence[*elem as usize - 1] {
                // TODO: delete this in next release
                panic!("this should not happend, please correct check_row")
            } else {
                number_presence[*elem as usize - 1] = true;
            }
        }

        // iterate over position of the zeros in row
        for to_guess in row
            .iter()
            .enumerate()
            .filter_map(|(j, val)| if *val == 0 { Some(j) } else { None })
        {
            // get possible guess in that row
            let possible_guess: Vec<u8> = number_presence
                .iter()
                .enumerate()
                .filter(|(_, val)| !*val)
                .map(|(i, _)| (i + 1) as u8)
                .collect();
            let position = Position(row_number, to_guess);

            // if guess is sigle return a single guess instantly replacable
            if possible_guess.len() == 1 {
                guesses.push(Guess::Single(
                    position,
                    possible_guess.get(0).unwrap().to_owned(),
                ));
            // else return a multiple guess to aid further research
            } else {
                guesses.push(Guess::Multiple(position, possible_guess));
            }
        }
        guesses
    }

#[cfg(test)]
mod tests {
    use super::*;

    // the test below check if guess can be made right in a line
    #[test]
    fn test_guess_row() {
        let mut board_to_guess = Board::from_file("tests/guess_row.txt");
        let valid_board = Board::from_file("tests/valid_board.txt");

        for j in 0..9 {
            for guess in board_to_guess.guess_row(j) {
                board_to_guess.apply_guess(guess);
            }
        }

        assert_eq!(board_to_guess.board, valid_board.board);
    }
}
