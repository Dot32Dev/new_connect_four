use std::cmp;
use crate::Colour;
use colored::*;

#[derive(Clone, Copy)]
pub struct Board {
    pub board: [[Colour; 6]; 7],
	pub highlighted_column: Option<usize>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[Colour::None; 6]; 7],
			highlighted_column: None,
        }
    }

	// I would use this if i weren't using Termion
	// This is just a wrapper for the to_string() function, that prints the string.
	#[allow(dead_code)]
    pub fn print(&self) {
        println!("{}", self.to_string());
    }

	pub fn to_string(&self) -> String {
		let mut string = String::new();
		string.push_str("╭ 1  2  3  4  5  6  7 ╮\n");
		for y in 0..6 {
			string.push_str("│");
			for x in 0..7 {
				let border = match self.highlighted_column {
					Some(column) => if column == x { "┊" } else { " " },
					None => " ",
				};
				match self.board[x][y] {
					Colour::None => string.push_str(&format!("{1}{0}{1}", "▿".dimmed(), border)),
					Colour::Red => string.push_str(&format!("{1}{0}{1}", "◉".red(), border)),
					Colour::Blue => string.push_str(&format!("{1}{0}{1}", "◉".blue(), border)),
				}
			}
			string.push_str("│\n");
		}
		string.push_str("╰");
		for x in 0..7 {
			match self.highlighted_column {
				Some(column) => if column == x {
					string.push_str("┴─┴")
				} else {
					string.push_str("───")
				},
				None => string.push_str("───"),
			}
		}
		string.push_str("╯");
		string
	}

    pub fn drop_piece(&mut self, x: usize, piece: Colour) -> Result<usize, &str> {
        for y in (0..6).rev() {
            match self.board[x][y] {
                Colour::None => {
                    self.board[x][y] = piece;
					self.highlighted_column = Some(x);
					return Ok(y); // Return Ok if the piece was dropped, and the y position of the piece
                }
                _ => continue,
            }
        }
		Err("Column is full") // Return Err if the column is full
    }

	// This took so long to write, but it works! (I think)
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
				match self.board[cmp::min(cmp::max(x as i32 - (y as i32-i as i32), 0), 7-1) as usize][cmp::min(i, 6-1)] {
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

	// Used for animating the drop of a piece
	pub fn animation_frame(&mut self, frame: usize) -> Board {
		// Calculate the y position of the piece
		let mut y = 0;
		for i in 0..6 {
			if !(self.board[self.highlighted_column.unwrap()][i] == Colour::None) {
				y = i;
				break;
			}
		}
		// Finds the colour of the piece
		let colour = self.board[self.highlighted_column.unwrap()][y];
		// Creates a new board based off the current board
		let mut board = self.clone();
		// Removes the piece from the new board
		board.board[self.highlighted_column.unwrap()][y] = Colour::None;
		// Adds the piece to the new board, at the Y index specified by frame
		board.board[self.highlighted_column.unwrap()][frame] = colour;
		// Returns the new board
		board
	}
}