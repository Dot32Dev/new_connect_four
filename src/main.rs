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
    let mut turn = Colour::Red;

    loop {
        println!("\n    It's {}'s turn!\n", if turn == Colour::Red { "Red".red() } else { "Blue".blue() });
        board.print();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: usize = match input.trim().parse() {
            Ok(num) if num <= 7 => num,
            _ => {
                println!("{}", ">> Please select a valid column".yellow());
                continue;
            }
        };
        board.drop_piece(input -1, turn);
        match board.check_win_at(input -1) {
            Some(colour) => {
                println!("\n    {} wins!\n", if colour == Colour::Red { "Red".red() } else { "Blue".blue() });
                board.print();
                break;
            }
            _ => (),
        }

        turn = if turn == Colour::Red { Colour::Blue } else { Colour::Red };
    }
}
