use std::path::PathBuf;

use clap::Parser;

use sudoku::Board;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the file containing the board
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("{}", &args.path.as_path().as_os_str().to_str().unwrap());
    let board = Board::from_file(&args.path.as_path().as_os_str().to_str().unwrap());
    match board.solve() {
        Ok(solved) => {
            println!("Board solved:");
            solved.print_diff(&board);
        }
        Err(_) => {
            println!("Board is not solvable:");
            board.print_complete();
        }
    }
}
