use ndarray::prelude::*;

struct Board {
    board: Array2<u8>,
}

impl Board {
    fn init() -> Self {
        let board = Array2::from_elem((9, 9), 0_u8);
        Board { board }
    }

    fn print_simple(&self) {
        for i in 0..9 {
            for j in 0..9 {
                print!("{} ", self.board[(i, j)]);
            }
            println!();
        }
    }
}

fn main() {
    let board = Board::init();
    board.print_simple();
}
