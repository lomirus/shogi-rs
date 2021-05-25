#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    /// 香車
    Lance,
    /// 桂馬
    Knight,
    /// 銀將
    Silver,
    /// 金將
    Gold,
    /// 王將
    King,
    /// 飛車
    Rook,
    /// 角行
    Bishop,
    /// 步兵
    Pawn,
}

impl PieceType {
    /// Returns the kanji name of the piece.
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

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    /// Piece type.
    pub r#type: PieceType,
    /// Piece side.
    pub side: bool,
}
