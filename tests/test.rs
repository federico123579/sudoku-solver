use sudoku::Board;

#[test]
fn test_print_complete() {
    let board = Board::init();
    board.print_complete();
}

#[test]
fn test_print_simple() {
    let board = Board::init();
    board.print_simple();
}
