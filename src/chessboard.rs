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
    /// Chessboard data.
    board: [[Option<Piece>; 9]; 9],
    /// Coordinate of the chosen square now.
    chosen: (u16, u16),
    /// Coordinate of the focused square now.
    focus: (u16, u16),
}

impl Chessboard {
    /// Print the grid of the chessboard.
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

    /// Print the name and side of all pieces at their corresponding square.
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

    /// Draw one square with specific color, while using the bold symbol.
    fn hightlight_square(&self, (x, y): (u16, u16), color: Color) -> Result<()> {
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

    /// Draw one square as a common square.
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

    /// Print the chessboard.
    pub fn print(&self) -> Result<()> {
        self.print_background()?;
        self.print_pieces()?;
        self.hightlight_square(self.chosen, Color::Red)?;
        self.hightlight_square(self.focus, Color::Green)?;
        let mut stdout = stdout();
        stdout.flush()?;
        Ok(())
    }

    /// Listen the keyboard input events.
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
                        KeyCode::Up => self.move_up_focus()?,
                        KeyCode::Down => self.move_down_focus()?,
                        KeyCode::Left => self.move_left_focus()?,
                        KeyCode::Right => self.move_right_focus()?,
                        KeyCode::Char('w') => self.move_up_focus()?,
                        KeyCode::Char('s') => self.move_down_focus()?,
                        KeyCode::Char('a') => self.move_left_focus()?,
                        KeyCode::Char('d') => self.move_right_focus()?,
                        KeyCode::Enter => {
                            self.reset_square(self.chosen)?;
                            self.reset_square(self.focus)?;
                            self.chosen.0 = self.focus.0;
                            self.chosen.1 = self.focus.1;
                            self.hightlight_square(self.chosen, Color::Red)?;
                            self.hightlight_square(self.focus, Color::Green)?;
                        }
                        _ => (),
                    }
                }
            }
        }
        Ok(())
    }

    /// Move up the coordinate of the focused square.
    fn move_up_focus(&mut self) -> Result<()> {
        self.reset_square(self.focus)?;
        if self.focus.1 != 0 {
            self.focus.1 -= 1;
        }
        self.hightlight_square(self.chosen, Color::Red)?;
        self.hightlight_square(self.focus, Color::Green)?;
        Ok(())
    }

    /// Move down the coordinate of the focused square.
    fn move_down_focus(&mut self) -> Result<()> {
        self.reset_square(self.focus)?;
        if self.focus.1 != 8 {
            self.focus.1 += 1;
        }
        self.hightlight_square(self.chosen, Color::Red)?;
        self.hightlight_square(self.focus, Color::Green)?;
        Ok(())
    }

    /// Move left the coordinate of the focused square.
    fn move_left_focus(&mut self) -> Result<()> {
        self.reset_square(self.focus)?;
        if self.focus.0 != 0 {
            self.focus.0 -= 1;
        }
        self.hightlight_square(self.chosen, Color::Red)?;
        self.hightlight_square(self.focus, Color::Green)?;
        Ok(())
    }

    /// Move right the coordinate of the focused square.
    fn move_right_focus(&mut self) -> Result<()> {
        self.reset_square(self.focus)?;
        if self.focus.0 != 8 {
            self.focus.0 += 1;
        }
        self.hightlight_square(self.chosen, Color::Red)?;
        self.hightlight_square(self.focus, Color::Green)?;
        Ok(())
    }
}

/// Returns the default chessboard.
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

/// The position of a square in the chessboard.
enum PosType {
    /// Top left corner.
    TopLeft,
    /// Top right corner.
    TopRight,
    /// Bottom left corner.
    BottomLeft,
    /// Bottom right corner.
    BottomRight,
    /// Top edge.
    Top,
    /// Bottom edge.
    Bottom,
    /// Left edge.
    Left,
    /// Right edge.
    Right,
    /// Other positons (in the chessboard).
    Other,
}

/// Returns the `PosType` of the square of given coordinate. 
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
