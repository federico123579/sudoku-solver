use crate::Board;

impl Board {
    pub fn print_simple(&self) {
        for i in 0..9 {
            for j in 0..9 {
                match self.values[(i, j)] {
                    0 => print!(". "),
                    _ => print!("{} ", self.values[(i, j)]),
                }
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
                match self.values[(i, j)] {
                    0 => print!(". "),
                    _ => print!("{} ", self.values[(i, j)]),
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::prelude::*;

    fn partially_filled_board() -> Board {
        Board::new(array![
            [0, 1, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 1, 0, 1, 1],
            [0, 1, 0, 1, 0, 0, 1, 0, 1],
            [0, 1, 0, 0, 0, 1, 0, 0, 1],
            [0, 0, 1, 0, 1, 0, 1, 0, 1],
            [0, 1, 0, 1, 1, 1, 0, 0, 1],
            [1, 1, 1, 0, 0, 1, 1, 0, 0],
            [0, 0, 0, 1, 1, 1, 0, 0, 1],
            [0, 1, 1, 0, 1, 0, 0, 1, 1]
        ])
    }

    #[test]
    fn test_empty_print_simple() {
        let empty_board = Board::empty();
        empty_board.print_simple();
    }

    #[test]
    fn test_regular_board_print_simple() {
        let partial_full_board = partially_filled_board();
        partial_full_board.print_simple();
    }

    #[test]
    fn test_empty_print_complete() {
        let empty_board = Board::empty();
        empty_board.print_complete();
    }

    #[test]
    fn test_regular_board_print_complete() {
        let partial_full_board = partially_filled_board();
        partial_full_board.print_complete();
    }
}
