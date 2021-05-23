use crate::piece::{Piece, PieceType};
use crossterm::{
    cursor,
    style::{self, Colorize},
    terminal::{Clear, ClearType},
    QueueableCommand, Result,
};
use std::io::stdout;

#[derive(Copy, Clone, Debug)]
pub struct ChessBoard([[Option<Piece>; 9]; 9]);

impl ChessBoard {
    fn print_background() -> Result<()> {
        let mut stdout = stdout();
        stdout
            .queue(Clear(ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;
        println!("┌────┬────┬────┬────┬────┬────┬────┬────┬────┐");
        for row in 0..8 {
            println!(
                "│    │    │    │    │    │    │    │    │    │\n\
                      │    │    │    │    │    │    │    │    │    │\n\
                      ├────┼────┼────┼────┼────┼────┼────┼────┼────┤"
            );
        }
        println!(
            "│    │    │    │    │    │    │    │    │    │\n\
                  │    │    │    │    │    │    │    │    │    │\n\
                  └────┴────┴────┴────┴────┴────┴────┴────┴────┘"
        );
        Ok(())
    }
    pub fn print(&self) -> Result<()> {
        ChessBoard::print_background();
        let mut stdout = stdout();
        for row in 0..self.0.len() {
            for col in 0..self.0[0].len() {
                if let Some(piece) = self.0[row][col] {
                    for (i, c) in piece.r#type.to_string().char_indices() {
                        stdout
                            .queue(cursor::MoveTo((col * 5 + 2) as u16, (row * 3 + 1 + i / 3) as u16))?
                            .queue(style::Print(c))?;
                    }
                }
            }
        }
        stdout
            .queue(cursor::MoveTo(0, (9 * 3 + 1) as u16))?;
        Ok(())
    }
}

pub fn new() -> ChessBoard {
    ChessBoard([
        [
            Some(Piece {
                r#type: PieceType::Lance,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Knight,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Silver,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Gold,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::King,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Gold,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Silver,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Knight,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Lance,
                side: false,
            }),
        ],
        [
            None::<Piece>,
            Some(Piece {
                r#type: PieceType::Rook,
                side: false,
            }),
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            Some(Piece {
                r#type: PieceType::Bishop,
                side: false,
            }),
            None::<Piece>,
        ],
        [
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
        ],
        [
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
        ],
        [
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
        ],
        [
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
        ],
        [
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
        ],
        [
            None::<Piece>,
            Some(Piece {
                r#type: PieceType::Bishop,
                side: true,
            }),
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            Some(Piece {
                r#type: PieceType::Rook,
                side: true,
            }),
            None::<Piece>,
        ],
        [
            Some(Piece {
                r#type: PieceType::Lance,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Knight,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Silver,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Gold,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::King,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Gold,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Silver,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Knight,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Lance,
                side: true,
            }),
        ],
    ])
}
