use crate::piece::{Piece, PieceType};
use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand, Result,
};
use std::io::{stdout, Write};

#[derive(Copy, Clone, Debug)]
pub struct Chessboard {
    board: [[Option<Piece>; 9]; 9],
    chosen: (usize, usize),
    focus: (usize, usize),
}

impl Chessboard {
    fn print_background(&self) -> Result<()> {
        let mut stdout = stdout();
        stdout
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?
            .queue(Print("┌────┬────┬────┬────┬────┬────┬────┬────┬────┐"))?;
        for row in 0..8 {
            stdout
                .queue(MoveTo(0, row * 3 + 1))?
                .queue(Print("│    │    │    │    │    │    │    │    │    │"))?
                .queue(MoveTo(0, row * 3 + 2))?
                .queue(Print("│    │    │    │    │    │    │    │    │    │"))?
                .queue(MoveTo(0, row * 3 + 3))?
                .queue(Print("├────┼────┼────┼────┼────┼────┼────┼────┼────┤"))?;
        }
        stdout
            .queue(MoveTo(0, 8 * 3 + 1))?
            .queue(Print("│    │    │    │    │    │    │    │    │    │"))?
            .queue(MoveTo(0, 8 * 3 + 2))?
            .queue(Print("│    │    │    │    │    │    │    │    │    │"))?
            .queue(MoveTo(0, 8 * 3 + 3))?
            .queue(Print("└────┴────┴────┴────┴────┴────┴────┴────┴────┘"))?;
        Ok(())
    }
    fn print_pieces(&self) -> Result<()> {
        let mut stdout = stdout();
        for row in 0..self.board.len() {
            for col in 0..self.board[0].len() {
                if let Some(piece) = self.board[row][col] {
                    if piece.side {
                        stdout
                            .queue(MoveTo((col * 5 + 1) as u16, (row * 3 + 1) as u16))?
                            .queue(Print("╱  ╲"))?;
                    } else {
                        stdout
                            .queue(MoveTo((col * 5 + 1) as u16, (row * 3 + 2) as u16))?
                            .queue(Print("╲  ╱"))?;
                    }
                    for (i, c) in piece.r#type.to_string().char_indices() {
                        stdout
                            .queue(MoveTo((col * 5 + 2) as u16, (row * 3 + 1 + i / 3) as u16))?
                            .queue(Print(c))?;
                    }
                }
            }
        }
        Ok(())
    }
    fn hightlight(&self, x: u16, y: u16, color: Color) -> Result<()> {
        let mut stdout = stdout();
        stdout.queue(SetForegroundColor(color))?;
        self.print_square(x, y)?;
        stdout.queue(ResetColor)?;
        Ok(())
    }
    fn print_square(&self, x: u16, y: u16) -> Result<()> {
        let mut stdout = stdout();
        stdout
            .queue(MoveTo(x * 5, y * 3))?
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
            .queue(MoveTo(x * 5, y * 3 + 1))?
            .queue(Print("│"))?
            .queue(MoveTo(x * 5 + 5, y * 3 + 1))?
            .queue(Print("│"))?;
        stdout
            .queue(MoveTo(x * 5, y * 3 + 2))?
            .queue(Print("│"))?
            .queue(MoveTo(x * 5 + 5, y * 3 + 2))?
            .queue(Print("│"))?;
        stdout
            .queue(MoveTo(x * 5, y * 3 + 3))?
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
        stdout.flush()?;
        Ok(())
    }
    pub fn print(&self) -> Result<()> {
        self.print_background()?;
        self.print_pieces()?;
        self.hightlight(self.chosen.0 as u16, self.chosen.1 as u16, Color::Red)?;
        self.hightlight(self.focus.0 as u16, self.focus.1 as u16, Color::Green)?;
        let mut stdout = stdout();
        stdout.flush()?;
        Ok(())
    }
    pub fn listen(mut self) -> Result<()> {
        loop {
            if let Event::Key(event) = read()? {
                if matches!(event.code, KeyCode::Char('c'))
                    && matches!(event.modifiers, KeyModifiers::CONTROL)
                {
                    let mut stdout = stdout();
                    stdout.queue(MoveTo(0, 9 * 3 + 1))?;
                    break;
                } else {
                    match event.code {
                        KeyCode::Up => {
                            self.print_square(self.focus.0 as u16, self.focus.1 as u16)?;
                            if self.focus.1 != 0 {
                                self.focus.1 -= 1;
                            }
                            self.hightlight(
                                self.chosen.0 as u16,
                                self.chosen.1 as u16,
                                Color::Red,
                            )?;
                            self.hightlight(
                                self.focus.0 as u16,
                                self.focus.1 as u16,
                                Color::Green,
                            )?;
                        }
                        KeyCode::Down => {
                            self.print_square(self.focus.0 as u16, self.focus.1 as u16)?;
                            if self.focus.1 != 8 {
                                self.focus.1 += 1;
                            }
                            self.hightlight(
                                self.chosen.0 as u16,
                                self.chosen.1 as u16,
                                Color::Red,
                            )?;
                            self.hightlight(
                                self.focus.0 as u16,
                                self.focus.1 as u16,
                                Color::Green,
                            )?;
                        }
                        KeyCode::Left => {
                            self.print_square(self.focus.0 as u16, self.focus.1 as u16)?;
                            if self.focus.0 != 0 {
                                self.focus.0 -= 1;
                            }
                            self.hightlight(
                                self.chosen.0 as u16,
                                self.chosen.1 as u16,
                                Color::Red,
                            )?;
                            self.hightlight(
                                self.focus.0 as u16,
                                self.focus.1 as u16,
                                Color::Green,
                            )?;
                        }
                        KeyCode::Right => {
                            self.print_square(self.focus.0 as u16, self.focus.1 as u16)?;
                            if self.focus.0 != 8 {
                                self.focus.0 += 1;
                            }
                            self.hightlight(
                                self.chosen.0 as u16,
                                self.chosen.1 as u16,
                                Color::Red,
                            )?;
                            self.hightlight(
                                self.focus.0 as u16,
                                self.focus.1 as u16,
                                Color::Green,
                            )?;
                        }
                        _ => (),
                    }
                }
            }
        }
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
        focus: (4, 8),
    }
}
