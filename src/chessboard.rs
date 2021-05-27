use crate::piece::{Piece, PieceType};
use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand, Result,
};
use std::io::{stdout, Write};

#[derive(Clone, Debug)]
pub struct Chessboard {
    /// Chessboard data.
    board: [[Option<Piece>; 9]; 9],
    /// Coordinate of the chosen square now.
    chosen: (usize, usize),
    /// Coordinate of the focused square now.
    focus: (usize, usize),
    reachable: Vec<(usize, usize)>,
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
    fn hightlight_square(&self, (x, y): (usize, usize), color: Color) -> Result<()> {
        let x = x as u16;
        let y = y as u16;
        let mut stdout = stdout();
        stdout.queue(SetForegroundColor(color))?;
        stdout
            // Row 1
            .queue(MoveTo(x * 5, y * 3))?
            .queue(Print(if x == 0 && y == 0 {
                "┏"
            } else if x == 0 {
                "┣"
            } else if y == 0 {
                "┳"
            } else {
                "╋"
            }))?
            .queue(Print("━━━━"))?
            .queue(Print(if x == 8 && y == 0 {
                "┓"
            } else if x == 8 {
                "┫"
            } else if y == 0 {
                "┳"
            } else {
                "╋"
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
            .queue(Print(if x == 0 && y == 8 {
                "┗"
            } else if x == 0 {
                "┣"
            } else if y == 8 {
                "┻"
            } else {
                "╋"
            }))?
            .queue(Print("━━━━"))?
            .queue(Print(if x == 8 && y == 8 {
                "┛"
            } else if x == 8 {
                "┫"
            } else if y == 8 {
                "┻"
            } else {
                "╋"
            }))?;
        stdout.flush()?;
        stdout.queue(ResetColor)?;
        Ok(())
    }

    /// Draw one square as a common square.
    fn reset_square(&self, (x, y): (usize, usize)) -> Result<()> {
        let x = x as u16;
        let y = y as u16;
        let mut stdout = stdout();
        stdout
            // Row 1
            .queue(MoveTo(x * 5, y * 3))?
            .queue(Print(if x == 0 && y == 0 {
                "┌"
            } else if x == 0 {
                "├"
            } else if y == 0 {
                "┬"
            } else {
                "┼"
            }))?
            .queue(Print("────"))?
            .queue(Print(if x == 8 && y == 0 {
                "┐"
            } else if x == 8 {
                "┤"
            } else if y == 0 {
                "┬"
            } else {
                "┼"
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
            .queue(Print(if x == 0 && y == 8 {
                "└"
            } else if x == 0 {
                "├"
            } else if y == 8 {
                "┴"
            } else {
                "┼"
            }))?
            .queue(Print("────"))?
            .queue(Print(if x == 8 && y == 8 {
                "┘"
            } else if x == 8 {
                "┤"
            } else if y == 8 {
                "┴"
            } else {
                "┼"
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
                            for square in &self.reachable {
                                self.reset_square(*square)?;
                            }

                            self.chosen.0 = self.focus.0;
                            self.chosen.1 = self.focus.1;
                            self.update_reachable_squares(self.chosen);

                            self.draw_hightlight_squares()?;
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
        self.draw_hightlight_squares()
    }

    /// Move down the coordinate of the focused square.
    fn move_down_focus(&mut self) -> Result<()> {
        self.reset_square(self.focus)?;
        if self.focus.1 != 8 {
            self.focus.1 += 1;
        }
        self.draw_hightlight_squares()
    }

    /// Move left the coordinate of the focused square.
    fn move_left_focus(&mut self) -> Result<()> {
        self.reset_square(self.focus)?;
        if self.focus.0 != 0 {
            self.focus.0 -= 1;
        }
        self.draw_hightlight_squares()
    }

    /// Move right the coordinate of the focused square.
    fn move_right_focus(&mut self) -> Result<()> {
        self.reset_square(self.focus)?;
        if self.focus.0 != 8 {
            self.focus.0 += 1;
        }
        self.draw_hightlight_squares()
    }

    fn update_reachable_squares(&mut self, (x, y): (usize, usize)) {
        self.reachable.clear();
        let piece = self.board[y as usize][x as usize];
        if let None::<Piece> = piece {
            return;
        }
        let piece = piece.unwrap();
        let direction = if piece.side { -1 } else { 1 };
        match piece.r#type {
            PieceType::Pawn => {
                self.try_push_reachable(piece.side, (x, y), (0, direction));
            }
            PieceType::Rook => {
                let mut i = 1;
                while self.try_push_reachable(piece.side, (x, y), (i, 0)) {
                    i += 1;
                }
                let mut i = 1;
                while self.try_push_reachable(piece.side, (x, y), (-i, 0)) {
                    i += 1;
                }
                let mut i = 1;
                while self.try_push_reachable(piece.side, (x, y), (0, i)) {
                    i += 1;
                }
                let mut i = 1;
                while self.try_push_reachable(piece.side, (x, y), (0, -i)) {
                    i += 1;
                }
            }
            PieceType::Bishop => {
                let mut i = 1;
                while self.try_push_reachable(piece.side, (x, y), (i, i)) {
                    i += 1;
                }
                let mut i = 1;
                while self.try_push_reachable(piece.side, (x, y), (i, -i)) {
                    i += 1;
                }
                let mut i = 1;
                while self.try_push_reachable(piece.side, (x, y), (-i, i)) {
                    i += 1;
                }
                let mut i = 1;
                while self.try_push_reachable(piece.side, (x, y), (-i, -i)) {
                    i += 1;
                }
            }
            PieceType::Lance => {
                let mut i = 1;
                while self.try_push_reachable(piece.side, (x, y), (0, direction * i)) {
                    i += 1;
                }
            }
            PieceType::Knight => {
                self.try_push_reachable(piece.side, (x, y), (-1, direction * 2));
                self.try_push_reachable(piece.side, (x, y), (1, direction * 2));
            }
            PieceType::Silver => {
                self.try_push_reachable(piece.side, (x, y), (-1, direction));
                self.try_push_reachable(piece.side, (x, y), (0, direction));
                self.try_push_reachable(piece.side, (x, y), (1, direction));
                self.try_push_reachable(piece.side, (x, y), (-1, 1));
                self.try_push_reachable(piece.side, (x, y), (1, 1));
            }
            PieceType::Gold => {
                self.try_push_reachable(piece.side, (x, y), (-1, direction));
                self.try_push_reachable(piece.side, (x, y), (0, direction));
                self.try_push_reachable(piece.side, (x, y), (1, direction));
                self.try_push_reachable(piece.side, (x, y), (-1, 0));
                self.try_push_reachable(piece.side, (x, y), (1, 0));
                self.try_push_reachable(piece.side, (x, y), (0, -direction));
            }
            PieceType::King => {
                self.try_push_reachable(piece.side, (x, y), (-1, -1));
                self.try_push_reachable(piece.side, (x, y), (0, -1));
                self.try_push_reachable(piece.side, (x, y), (1, -1));
                self.try_push_reachable(piece.side, (x, y), (-1, 0));
                self.try_push_reachable(piece.side, (x, y), (1, 0));
                self.try_push_reachable(piece.side, (x, y), (0, 1));
                self.try_push_reachable(piece.side, (x, y), (-1, 1));
                self.try_push_reachable(piece.side, (x, y), (1, 1));
            }
        }
    }

    fn try_push_reachable(
        &mut self,
        side: bool,
        (x, y): (usize, usize),
        (offset_x, offset_y): (isize, isize),
    ) -> bool {
        if x as isize + offset_x > 8
            || y as isize + offset_y > 8
            || x as isize + offset_x < 0
            || y as isize + offset_y < 0
        {
            return false;
        }
        let check_x = (x as isize + offset_x) as usize;
        let check_y = (y as isize + offset_y) as usize;
        match self.board[check_y][check_x] {
            Option::Some(piece) => {
                if piece.side != side {
                    self.reachable.push((check_x, check_y))
                }
                false
            }
            Option::None => {
                self.reachable.push((check_x, check_y));
                true
            }
        }
    }

    fn draw_hightlight_squares(&self) -> Result<()> {
        self.hightlight_square(self.chosen, Color::Red)?;
        for square in &self.reachable {
            self.hightlight_square(*square, Color::Yellow)?;
        }
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
        reachable: Vec::new(),
    }
}
