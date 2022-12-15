mod board;
use board::*;
use colored::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colour {
    None,
    Red,
    Blue,
}

fn main() {
    let mut board = Board::new();

    board.drop_piece(4, Colour::Red);
    board.drop_piece(4, Colour::Blue);

    board.drop_piece(5, Colour::Blue);
    board.drop_piece(5, Colour::Blue);
    board.drop_piece(5, Colour::Red);
    board.drop_piece(5, Colour::Blue);

    board.drop_piece(1, Colour::Red);
    board.drop_piece(2, Colour::Blue);
    board.drop_piece(3, Colour::Red);

    board.drop_piece(3, Colour::Blue);
    board.drop_piece(4, Colour::Blue);

    board.drop_piece(2, Colour::Red);
    board.drop_piece(1, Colour::Blue);
    board.drop_piece(1, Colour::Red);

    board.drop_piece(0, Colour::Blue);
    board.drop_piece(0, Colour::Red);
    board.drop_piece(0, Colour::Blue);
    board.drop_piece(0, Colour::Red);

    board.highlight_piece(5, 2);
    board.highlight_piece(3, 5);

    println!("{:?}", board.check_win_at(5, 2));
    println!("{:?}", board.check_win_at(0, 2));
    // println!("{:?}", board.check_win_at(3, 5));

    println!("\n    It's {}'s turn!\n", "Red".red());
    board.print();
}
