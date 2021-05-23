#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum PieceType {
    Lance,  // 香車
    Knight, // 桂馬
    Silver, // 銀將
    Gold,   // 金將
    King,   // 王將
    Rook,   // 飛車
    Bishop, // 角行
    Pawn,   // 步兵
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Piece {
    pub r#type: PieceType,
    pub side: bool,
}

