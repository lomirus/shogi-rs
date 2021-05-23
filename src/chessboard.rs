use crate::piece::{Piece, PieceType};
use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand, Result,
};
use std::io::stdout;

#[derive(Copy, Clone, Debug)]
pub struct Chessboard {
    board: [[Option<Piece>; 9]; 9],
    chosen: (usize, usize),
}

impl Chessboard {
    fn print_background() -> Result<()> {
        let mut stdout = stdout();
        stdout
            .queue(Clear(ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;
        println!("┌────┬────┬────┬────┬────┬────┬────┬────┬────┐");
        for _row in 0..8 {
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
    fn hightlight(x: u16, y: u16) -> Result<()> {
        let mut stdout = stdout();
        stdout.queue(SetForegroundColor(Color::Red))?;
        stdout
            .queue(cursor::MoveTo(x * 5, y * 3))?
            .queue(if x == 0 && y == 0 {
                Print("┌")
            } else if x == 0 {
                Print("├")
            } else if y == 0 {
                Print("┬")
            } else {
                Print("┼")
            })?
            .queue(Print("─"))?
            .queue(Print("─"))?
            .queue(Print("─"))?
            .queue(Print("─"))?
            .queue(if x == 8 && y == 0 {
                Print("┐")
            } else if x == 8 {
                Print("┤")
            } else if y == 0 {
                Print("┬")
            } else {
                Print("┼")
            })?;
        stdout
            .queue(cursor::MoveTo(x * 5, y * 3 + 1))?
            .queue(Print("│"))?
            .queue(cursor::MoveTo(x * 5 + 5, y * 3 + 1))?
            .queue(Print("│"))?;
        stdout
            .queue(cursor::MoveTo(x * 5, y * 3 + 2))?
            .queue(Print("│"))?
            .queue(cursor::MoveTo(x * 5 + 5, y * 3 + 2))?
            .queue(Print("│"))?;
        stdout
            .queue(cursor::MoveTo(x * 5, y * 3 + 3))?
            .queue(if x == 0 && y == 8 {
                Print("└")
            } else if x == 0 {
                Print("├")
            } else if y == 8 {
                Print("┴")
            } else {
                Print("┼")
            })?
            .queue(Print("─"))?
            .queue(Print("─"))?
            .queue(Print("─"))?
            .queue(Print("─"))?
            .queue(if x == 8 && y == 8 {
                Print("┘")
            } else if x == 8 {
                Print("┤")
            } else if y == 8 {
                Print("┴")
            } else {
                Print("┼")
            })?;
        stdout.queue(ResetColor)?;
        Ok(())
    }
    pub fn print(&self) -> Result<()> {
        Chessboard::print_background()?;
        let mut stdout = stdout();
        for row in 0..self.board.len() {
            for col in 0..self.board[0].len() {
                if let Some(piece) = self.board[row][col] {
                    for (i, c) in piece.r#type.to_string().char_indices() {
                        stdout
                            .queue(cursor::MoveTo(
                                (col * 5 + 2) as u16,
                                (row * 3 + 1 + i / 3) as u16,
                            ))?
                            .queue(Print(c))?;
                    }
                }
            }
        }
        Chessboard::hightlight(self.chosen.0 as u16, self.chosen.1 as u16);
        stdout.queue(cursor::MoveTo(0, (9 * 3 + 1) as u16))?;
        Ok(())
    }
}

pub fn new() -> Chessboard {
    Chessboard {
        board: [
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
            [Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }); 9],
            [None::<Piece>; 9],
            [None::<Piece>; 9],
            [None::<Piece>; 9],
            [Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }); 9],
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
        ],
        chosen: (4, 8),
    }
}
