mod chessboard;
mod piece;
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode},Result};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let chessboard = chessboard::new();
    chessboard.print()?;
    disable_raw_mode()?;
    Ok(())
}
