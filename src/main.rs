use colored::*;

#[derive(Clone, Copy)]
enum Piece {
	None,
	Red,
	Blue
}

struct Board {
    board: [[Piece; 6]; 7],
    highlights: [[bool; 6]; 7],
}

impl Board {
    fn new() -> Board {
        Board {
            board: [[Piece::None; 6]; 7],
            highlights: [[false; 6]; 7],
        }
    }

    fn print(&self) {
        println!(" ╭ {} ╮", "1  2  3  4  5  6  7");
        for y in 0..6 {
            print!(" │");
            for x in 0..7 {
                match self.board[x][y] {
                    Piece::None => print!("{}", " ▿ ".dimmed()),
                    Piece::Red => {
                        match self.highlights[x][y] {
                            true => print!("{}", " ◈ ".red()),
                            false => print!("{}", " ◉ ".red())
                        }
                    },
                    Piece::Blue => {
                        match self.highlights[x][y] {
                            true => print!("{}", " ◈ ".blue()),
                            false => print!("{}", " ◉ ".blue())
                        }
                    },
                }
            }
            print!("│");
            println!();
        }
        println!(" ╰─────────────────────╯");
    }

    fn drop_piece(&mut self, x: usize, piece: Piece) {
        for y in (0..6).rev() {
            match self.board[x][y] {
                Piece::None => {
                    self.board[x][y] = piece;
                    break;
                },
                _ => continue,
            }
        }
    }

    fn highlight_piece(&mut self, x: usize, y: usize) {
        self.highlights[x][y] = true;
    }
}

fn main() {
    let mut board = Board::new();

    board.drop_piece(4, Piece::Red);
    board.drop_piece(4, Piece::Blue);
    board.drop_piece(5, Piece::Blue);

    board.highlight_piece(5, 5);

    println!("\n    It's {}'s turn!\n", "Red".red());
    board.print();
}
