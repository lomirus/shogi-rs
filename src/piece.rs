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

impl PieceType {
    pub fn to_string(&self) -> String {
        match self {
            PieceType::Lance => String::from("香車"),
            PieceType::Knight => String::from("桂馬"),
            PieceType::Silver => String::from("銀將"),
            PieceType::Gold => String::from("金將"),
            PieceType::King => String::from("王將"),
            PieceType::Rook => String::from("飛車"),
            PieceType::Bishop => String::from("角行"),
            PieceType::Pawn => String::from("步兵"),
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Piece {
    pub r#type: PieceType,
    pub side: bool,
}

