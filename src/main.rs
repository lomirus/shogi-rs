mod chessboard;
mod piece;

fn main() {
    let chessboard = chessboard::new();
    chessboard.print().unwrap();
}
