use std::path::PathBuf;

use clap::{Parser, Subcommand};

use sudoku::Board;

/// Simple program to greet a person
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Solve {
        /// Path to the file containing the board
        #[clap(parse(from_os_str), value_name = "FILE")]
        path: PathBuf,
    },
    Show {
        /// Path to the file containing the board
        #[clap(parse(from_os_str), value_name = "FILE")]
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Solve { path } => {
            println!("{}", path.as_path().as_os_str().to_str().unwrap());
            let board = Board::from_file(path.as_path().as_os_str().to_str().unwrap());
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
        Commands::Show { path } => {
            let board = Board::from_file(path.as_path().as_os_str().to_str().unwrap());
            board.print_complete();
        }
    }
}
