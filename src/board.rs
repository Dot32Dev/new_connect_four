use std::cmp;
use crate::Colour;
use colored::*;

pub struct Board {
    pub board: [[Colour; 6]; 7],
    pub highlights: [[bool; 6]; 7],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[Colour::None; 6]; 7],
            highlights: [[false; 6]; 7],
        }
    }

    pub fn print(&self) {
        println!(" ╭ {} ╮", "1  2  3  4  5  6  7");
        for y in 0..6 {
            print!(" │");
            for x in 0..7 {
                match self.board[x][y] {
                    Colour::None => print!("{}", " ▿ ".dimmed()),
                    Colour::Red => match self.highlights[x][y] {
                        true => print!("{}", " ⦻ ".red()),
                        false => print!("{}", " ◉ ".red()),
                    },
                    Colour::Blue => match self.highlights[x][y] {
                        true => print!("{}", " ⦻ ".blue()),
                        false => print!("{}", " ◉ ".blue()),
                    },
                }
            }
            print!("│");
            println!();
        }
        println!(" ╰─────────────────────╯");
    }

    pub fn drop_piece(&mut self, x: usize, piece: Colour) {
		self.unhighlight_board();
        for y in (0..6).rev() {
            match self.board[x][y] {
                Colour::None => {
                    self.board[x][y] = piece;
					self.highlight_piece(x, y);
                    break;
                }
                _ => continue,
            }
        }
    }

    fn highlight_piece(&mut self, x: usize, y: usize) {
        self.highlights[x][y] = true;
    }

	fn unhighlight_board(&mut self) {
		self.highlights = [[false; 6]; 7];
	}

	pub fn check_win_at(&mut self, x: usize) -> Option<Colour> {
		// Calculate the y position of the piece
		let mut y = 0;
		for i in 0..6 {
			if !(self.board[x][i] == Colour::None) {
				y = i;
				break;
			}
		}

		// println!("x: {}, y: {}", x, y);

		let colour = self.board[x][y];
		// println!("Colour: {:?}", colour);

		// VERTICAL
		if y < 3 {
			for i in y..=y+3 {
				match self.board[x][i] {
					// Match Guard (https://doc.rust-lang.org/reference/expressions/match-expr.html#match-guards)
					col if col == colour => {
						if i == y+3 {
							return Some(colour);
						}
					},
					_ => break,
				}
			}
		}

		// HORIZONTAL
		let mut count = 0;
		for i in cmp::max(x as i32 - 3, 0) as usize..cmp::min(x as i32 + 3, 7 - 1)  as usize {
			match self.board[i][y] {
				col if col == colour => {
					count += 1;
					if count == 4 {
						return Some(colour);
					}
				},
				_ => count = 0,
			}
		}

		// DIAGONAL ╱
		let mut count = 0;
		for i in (cmp::max(y as i8-3 as i8, 0) as usize..=y+3).rev() {
			// println!("x: {}, y: {}", x as i32 - (y as i32-i as i32), i);
			if cmp::max(x as i32 - (y as i32-i as i32), 0) == x as i32 - (y as i32-i as i32) && cmp::min(i, 6-1) == i {
				match self.board[(x as i32 - (y as i32-i as i32)) as usize][cmp::min(i, 6-1)] {
					col if col == colour => {
						count += 1;
						if count == 4 {
							return Some(colour);
						}
					},
					_ => count = 0,
				}
			}
		}

		// DIAGONAL ╲
		let mut count = 0;
		for i in (cmp::max(y as i8-3 as i8, 0) as usize..=y+3).rev() {
			if cmp::max(x as i32 + (y as i32-i as i32), 0) == x as i32 + (y as i32-i as i32) && cmp::min(i, 6-1) == i {
				match self.board[cmp::min(cmp::max(x as i32 + (y as i32-i as i32), 0), 7-1) as usize][cmp::min(i, 6-1)] {
					col if col == colour => {
						count += 1;
						if count == 4 {
							return Some(colour);
						}
					},
					_ => count = 0,
				}
			}
		}

		None
	}
}
