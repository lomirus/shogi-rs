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
    chosen: (u16, u16),
    focus: (u16, u16),
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
    fn hightlight(&self, (x, y): (u16, u16), color: Color) -> Result<()> {
        let mut stdout = stdout();
        stdout.queue(SetForegroundColor(color))?;
        stdout
            // Row 1
            .queue(MoveTo(x * 5, y * 3))?
            .queue(Print(match get_pos_type(x, y) {
                PosType::TopLeft => "┏",
                PosType::Left => "┣",
                PosType::Top => "┳",
                _ => "╋",
            }))?
            .queue(Print("━━━━"))?
            .queue(Print(match get_pos_type(x, y) {
                PosType::TopRight => "┓",
                PosType::Right => "┫",
                PosType::Top => "┳",
                _ => "╋",
            }))?
            // Row 2
            .queue(MoveTo(x * 5, y * 3 + 1))?
            .queue(Print("┃"))?
            .queue(MoveTo(x * 5 + 5, y * 3 + 1))?
            .queue(Print("┃"))?
            // Row 3
            .queue(MoveTo(x * 5, y * 3 + 2))?
            .queue(Print("┃"))?
            .queue(MoveTo(x * 5 + 5, y * 3 + 2))?
            .queue(Print("┃"))?
            // Row 4
            .queue(MoveTo(x * 5, y * 3 + 3))?
            .queue(Print(match get_pos_type(x, y) {
                PosType::BottomLeft => "┗",
                PosType::Left => "┣",
                PosType::Bottom => "┻",
                _ => "╋",
            }))?
            .queue(Print("━━━━"))?
            .queue(Print(match get_pos_type(x, y) {
                PosType::BottomRight => "┛",
                PosType::Right => "┫",
                PosType::Bottom => "┻",
                _ => "╋",
            }))?;
        stdout.flush()?;
        stdout.queue(ResetColor)?;
        Ok(())
    }
    fn reset_square(&self, (x, y): (u16, u16)) -> Result<()> {
        let mut stdout = stdout();
        stdout
            // Row 1
            .queue(MoveTo(x * 5, y * 3))?
            .queue(Print(match get_pos_type(x, y) {
                PosType::TopLeft => "┌",
                PosType::Left => "├",
                PosType::Top => "┬",
                _ => "┼",
            }))?
            .queue(Print("────"))?
            .queue(Print(match get_pos_type(x, y) {
                PosType::TopRight => "┐",
                PosType::Right => "┤",
                PosType::Top => "┬",
                _ => "┼",
            }))?
            // Row 2
            .queue(MoveTo(x * 5, y * 3 + 1))?
            .queue(Print("│"))?
            .queue(MoveTo(x * 5 + 5, y * 3 + 1))?
            .queue(Print("│"))?
            // Row 3
            .queue(MoveTo(x * 5, y * 3 + 2))?
            .queue(Print("│"))?
            .queue(MoveTo(x * 5 + 5, y * 3 + 2))?
            .queue(Print("│"))?
            // Row 4
            .queue(MoveTo(x * 5, y * 3 + 3))?
            .queue(Print(match get_pos_type(x, y) {
                PosType::BottomLeft => "└",
                PosType::Left => "├",
                PosType::Bottom => "┴",
                _ => "┼",
            }))?
            .queue(Print("────"))?
            .queue(Print(match get_pos_type(x, y) {
                PosType::BottomRight => "┘",
                PosType::Right => "┤",
                PosType::Bottom => "┴",
                _ => "┼",
            }))?;
        stdout.flush()?;
        Ok(())
    }
    pub fn print(&self) -> Result<()> {
        self.print_background()?;
        self.print_pieces()?;
        self.hightlight(self.chosen, Color::Red)?;
        self.hightlight(self.focus, Color::Green)?;
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
                            self.reset_square(self.focus)?;
                            if self.focus.1 != 0 {
                                self.focus.1 -= 1;
                            }
                            self.hightlight(self.chosen, Color::Red)?;
                            self.hightlight(self.focus, Color::Green)?;
                        }
                        KeyCode::Down => {
                            self.reset_square(self.focus)?;
                            if self.focus.1 != 8 {
                                self.focus.1 += 1;
                            }
                            self.hightlight(self.chosen, Color::Red)?;
                            self.hightlight(self.focus, Color::Green)?;
                        }
                        KeyCode::Left => {
                            self.reset_square(self.focus)?;
                            if self.focus.0 != 0 {
                                self.focus.0 -= 1;
                            }
                            self.hightlight(self.chosen, Color::Red)?;
                            self.hightlight(self.focus, Color::Green)?;
                        }
                        KeyCode::Right => {
                            self.reset_square(self.focus)?;
                            if self.focus.0 != 8 {
                                self.focus.0 += 1;
                            }
                            self.hightlight(self.chosen, Color::Red)?;
                            self.hightlight(self.focus, Color::Green)?;
                        }
                        KeyCode::Enter => {
                            self.reset_square(self.chosen)?;
                            self.reset_square(self.focus)?;
                            self.chosen.0 = self.focus.0;
                            self.chosen.1 = self.focus.1;
                            self.hightlight(self.chosen, Color::Red)?;
                            self.hightlight(self.focus, Color::Green)?;
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

enum PosType {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
    Other,
}

fn get_pos_type(x: u16, y: u16) -> PosType {
    if x == 0 && y == 0 {
        PosType::TopLeft
    } else if x == 0 && y == 8 {
        PosType::BottomLeft
    } else if x == 8 && y == 0 {
        PosType::TopRight
    } else if x == 8 && y == 8 {
        PosType::BottomRight
    } else if x == 0 {
        PosType::Left
    } else if x == 8 {
        PosType::Right
    } else if y == 0 {
        PosType::Top
    } else if y == 8 {
        PosType::Bottom
    } else {
        PosType::Other
    }
}
