mod board;
use colored::*;
use board::*;

fn main() {
    let mut board = Board::new();

    board.drop_piece(4, Piece::Red);
    board.drop_piece(4, Piece::Blue);
    board.drop_piece(5, Piece::Blue);

    board.highlight_piece(5, 5);

    println!("\n    It's {}'s turn!\n", "Red".red());
    board.print();
}
