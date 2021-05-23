mod chessboard;
mod piece;
use crossterm::{terminal, Result};

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    let chessboard = chessboard::new();
    chessboard.print()?;
    chessboard.listen()?;

    terminal::disable_raw_mode()?;
    Ok(())
}
