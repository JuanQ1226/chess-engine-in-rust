// Author: Juan Quintana

use std::fmt;

// Piece representation
#[derive(Clone, Copy, PartialEq)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

// Color representation
#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

// Piece Values
fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 1,
        Piece::Knight => 3,
        Piece::Bishop => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
        Piece::King => 0,
    }
}

// Square representation
#[derive(Copy, Clone, PartialEq)]
struct Square {
    piece: Option<Piece>,
    color: Option<Color>,
}

// Board representation
struct Board {
    squares: [[Square; 8]; 8],
    black_score: i32,
    white_score: i32,
}

impl Board {
    fn new() -> Self {
        let mut board = Board {
            squares: [[Square {
                piece: None,
                color: None,
            }; 8]; 8],
            black_score: 0,
            white_score: 0,
        };
        // Initialize the board with starting positions
        board.squares[0][1] = Square {
            piece: Some(Piece::Knight),
            color: Some(Color::White),
        };
        board.squares[0][6] = Square {
            piece: Some(Piece::Knight),
            color: Some(Color::White),
        };
        board.squares[7][1] = Square {
            piece: Some(Piece::Knight),
            color: Some(Color::Black),
        };
        board.squares[7][6] = Square {
            piece: Some(Piece::Knight),
            color: Some(Color::Black),
        };
        board.squares[0][2] = Square {
            piece: Some(Piece::Bishop),
            color: Some(Color::White),
        };
        board.squares[0][5] = Square {
            piece: Some(Piece::Bishop),
            color: Some(Color::White),
        };
        board.squares[7][2] = Square {
            piece: Some(Piece::Bishop),
            color: Some(Color::Black),
        };
        board.squares[7][5] = Square {
            piece: Some(Piece::Bishop),
            color: Some(Color::Black),
        };
        board.squares[0][3] = Square {
            piece: Some(Piece::Queen),
            color: Some(Color::White),
        };
        board.squares[7][3] = Square {
            piece: Some(Piece::Queen),
            color: Some(Color::Black),
        };
        board.squares[0][4] = Square {
            piece: Some(Piece::King),
            color: Some(Color::White),
        };
        board.squares[7][4] = Square {
            piece: Some(Piece::King),
            color: Some(Color::Black),
        };
        board.squares[0][0] = Square {
            piece: Some(Piece::Rook),
            color: Some(Color::White),
        };
        board.squares[0][7] = Square {
            piece: Some(Piece::Rook),
            color: Some(Color::White),
        };
        board.squares[7][0] = Square {
            piece: Some(Piece::Rook),
            color: Some(Color::Black),
        };
        board.squares[7][7] = Square {
            piece: Some(Piece::Rook),
            color: Some(Color::Black),
        };
        for i in 0..8 {
            board.squares[1][i] = Square {
                piece: Some(Piece::Pawn),
                color: Some(Color::White),
            };
            board.squares[6][i] = Square {
                piece: Some(Piece::Pawn),
                color: Some(Color::Black),
            };
        }
        board
    }

    fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), &'static str> {
        let piece = self.squares[from.0][from.1].piece;
        let color = self.squares[from.0][from.1].color;

        if piece.is_none() || color.is_none() {
            return Err("No piece at the starting position");
        }

        // Check if we can take a piece.
        if self.squares[to.0][to.1].piece.is_some() {
            let taken_piece = self.take_piece(to.0, to.1);
            if taken_piece.is_none() {
                return Err("Cannot take piece");
            }
            println!();
            println!("Piece Taken!");
            println!("White: {}, Black: {}", self.white_score, self.black_score)
        }

        self.squares[to.0][to.1].piece = piece;
        self.squares[to.0][to.1].color = color;
        self.squares[from.0][from.1].piece = None;
        self.squares[from.0][from.1].color = None;

        Ok(())
    }

    fn get_piece(&self, rank: usize, file: usize) -> Option<Piece> {
        self.squares[rank][file].piece
    }

    fn take_piece(&mut self, rank: usize, file: usize) -> Option<Piece> {
        let piece = self.squares[rank][file].piece;
        let color = self.squares[rank][file].color.unwrap();
        let points = piece_value(piece.unwrap());

        match color {
            Color::White => self.white_score += points,
            Color::Black => self.black_score += points,
        }

        self.squares[rank][file].piece = None;
        self.squares[rank][file].color = None;
        piece
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                match self.squares[rank][file] {
                    Square {
                        piece: Some(p),
                        color: Some(c),
                    } => {
                        let symbol = match p {
                            Piece::Pawn => 'P',
                            Piece::Knight => 'N',
                            Piece::Bishop => 'B',
                            Piece::Rook => 'R',
                            Piece::Queen => 'Q',
                            Piece::King => 'K',
                        };
                        let colored_symbol = if c == Color::White {
                            symbol.to_ascii_uppercase()
                        } else {
                            symbol.to_ascii_lowercase()
                        };
                        write!(f, "{} ", colored_symbol)?;
                    }
                    _ => write!(f, ". ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut board = Board::new();
    #[warn(while_true)]
    loop {
        let mut input = String::new();
        println!("{}", board);
        println!("Enter move (e.g. e2e4): ");
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            break;
        }
        let from = (
            8 - input.chars().nth(1).unwrap().to_digit(10).unwrap() as usize,
            (input.chars().nth(0).unwrap() as u8 - b'a') as usize,
        );
        let to = (
            8 - input.chars().nth(3).unwrap().to_digit(10).unwrap() as usize,
            (input.chars().nth(2).unwrap() as u8 - b'a') as usize,
        );

        match board.make_move(from, to) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }
}
