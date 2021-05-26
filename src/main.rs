mod chessboard;
mod piece;
use crossterm::{terminal, cursor, Result, ExecutableCommand};
use std::io::{stdout};

fn main() -> Result<()> {
    before_main()?;

    let chessboard = chessboard::new();
    chessboard.print()?;
    chessboard.listen()?;

    after_main()
}

fn before_main() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(cursor::Hide)?;
    terminal::enable_raw_mode()?;
    Ok(())
}

fn after_main() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
