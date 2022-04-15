use ndarray::prelude::*;

struct Board {
    board: Array2<u8>,
}

impl Board {
    fn init() -> Self {
        let board = Array2::from_elem((9, 9), 0_u8);
        Board { board }
    }

    // print utility functions

    fn print_simple(&self) {
        for i in 0..9 {
            for j in 0..9 {
                print!("{} ", self.board[(i, j)]);
            }
            println!();
        }
    }

    fn print_complete(&self) {
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

fn main() {
    let board = Board::init();
    board.print_complete();
}
