mod board;
use board::*;
use colored::*;
use unicode_segmentation::UnicodeSegmentation;

use termion::raw::IntoRawMode;
use termion::terminal_size;
use std::io::{Read, Write, stdout, stdin};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colour {
    None,
    Red,
    Blue,
}

fn main() {
    let mut board = Board::new();
    let mut turn = Colour::Red;
    
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut bytes = stdin.bytes();

    let size = terminal_size().unwrap();
    loop {
        redraw_game(&mut stdout, &board, turn);

        let mut input = 8;
        loop {
            let b = bytes.next().unwrap().unwrap();

            match b {
                    // Quit ctrl+c
                    3 => return,
                    // Enter
                    13 => if input != 8 {break},
                    // 1-7
                    49..=55 => {
                        input = (b - 49) as usize;
                        board.highlighted_column = Some(input);
                        redraw_game(&mut stdout, &board, turn);
                    },
                    // a => {write!(stdout, "{}", a).unwrap();},
                    _ => {
                        board.highlighted_column = None;
                        redraw_game(&mut stdout, &board, turn);
                    },
                }

            stdout.flush().unwrap();
        }
        board.drop_piece(input, turn);
        match board.check_win_at(input) {
            Some(colour) => {
                centred_print(&mut stdout, &format!("{}{}", termion::clear::All, termion::cursor::Hide), None, 1);
                centred_print(&mut stdout, &format!("{} wins!", if colour == Colour::Red { "Red".red() } else { "Blue".blue() }), Some(4), size.1/2-6);
                centred_print(&mut stdout, &format!("{}", board.to_string()), Some(11), size.1/2-4);
                centred_print(&mut stdout, &format!("{}{}", "Press any key to quit.".dimmed(), termion::cursor::Goto(0, terminal_size().unwrap().1)), Some(10), size.1/2+5);
                let _ = bytes.next();
                break;
            }
            _ => (),
        }
        turn = if turn == Colour::Red { Colour::Blue } else { Colour::Red };
    }
}

fn centred_print(
    stdout: &mut termion::raw::RawTerminal<std::io::StdoutLock>, 
    strings: &str,
    x: Option<u16>,
    y: u16,
) {
    let size = terminal_size().unwrap();

    for (i, string) in strings.lines().enumerate() {
        write!(stdout,
            "{}{}",
            match x {
                Some(x) => termion::cursor::Goto(size.0/2 - x, y + i as u16),
                None => termion::cursor::Goto(size.0/2 - (string.graphemes(true).count()/2) as u16, y + i as u16),
            },
            string,)
             .unwrap();
        stdout.flush().unwrap();
    }
}

fn redraw_game(
    stdout: &mut termion::raw::RawTerminal<std::io::StdoutLock>, 
    board: &Board,
    turn: Colour,
) {
    let size = terminal_size().unwrap();

    centred_print(stdout, &format!("{}{}", termion::clear::All, termion::cursor::Hide), None, 1);        
    centred_print(stdout, &format!("It's {}'s turn!", if turn == Colour::Red { "Red".red() } else { "Blue".blue() }), Some(8), size.1/2-6);
    centred_print(stdout, &format!("{}", board.to_string()), Some(11), size.1/2-4);
    centred_print(stdout,  &format!("Select column: {}", match board.highlighted_column {
        Some(column) => format!("{}", (column + 1).to_string().yellow()),
        None => "".to_string(),
    }), Some(8), size.1/2+5);
    match board.highlighted_column {
        Some(_) => centred_print(stdout, &format!("{}", "Press enter to confirm."), None, size.1/2+6),
        None => (),
    }
    centred_print(stdout, &format!("{}", "Press Ctrl+C to quit at any time.".dimmed()), Some(16), size.1/2+8);
}