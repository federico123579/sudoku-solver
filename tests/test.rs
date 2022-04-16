use sudoku::Board;

#[test]
fn test_print() {
    let board = Board::init();
    board.print_complete();
}
