use crate::piece::{Piece, PieceType};

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ChessBoard([[Option<Piece>; 9]; 9]);

impl ChessBoard {
    pub fn print(&self) {
        println!("{:?}", self);
    }
}

pub fn new() -> ChessBoard {
    ChessBoard([
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
        [
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: false,
            }),
        ],
        [
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
        ],
        [
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
        ],
        [
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
            None::<Piece>,
        ],
        [
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
            Some(Piece {
                r#type: PieceType::Pawn,
                side: true,
            }),
        ],
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
    ])
}
