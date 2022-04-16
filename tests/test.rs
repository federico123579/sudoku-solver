use sudoku::Board;

#[test]
fn test_init() {
    let board = Board::init();
    board.print_complete();
}
