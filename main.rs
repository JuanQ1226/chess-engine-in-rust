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

// Square representation
#[derive(Copy, Clone, PartialEq)]
struct Square {
    piece: Option<Piece>,
    color: Option<Color>,
}

// Board representation
struct Board {
    squares: [[Square; 8]; 8],
}

impl Board {
    fn new() -> Self {
        let mut board = Board {
            squares: [[Square {
                piece: None,
                color: None,
            }; 8]; 8],
        };
        // Initialize the board with starting positions
        // (This is a simplified version, you'd need to add all pieces)
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
        // Add more piece initializations here...
        board
    }

    fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), &'static str> {
        let piece = self.squares[from.0][from.1].piece;
        let color = self.squares[from.0][from.1].color;

        if piece.is_none() || color.is_none() {
            return Err("No piece at the starting position");
        }

        // Here you would add logic to check if the move is legal
        // For simplicity, we're just moving the piece without any checks

        self.squares[to.0][to.1].piece = piece;
        self.squares[to.0][to.1].color = color;
        self.squares[from.0][from.1].piece = None;
        self.squares[from.0][from.1].color = None;

        Ok(())
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
    println!("Initial board:");
    println!("{}", board);

    // Make a move
    match board.make_move((0, 0), (3, 0)) {
        Ok(()) => println!("Move made successfully"),
        Err(e) => println!("Error making move: {}", e),
    }

    println!("Board after move:");
    println!("{}", board);
}
