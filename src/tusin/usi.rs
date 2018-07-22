use std::fmt;

/// 駒種類の数。
//pub const PIECE_TYPE_LN : usize = 16;
/// # 駒種類
/// 先後なしの駒と空白。
#[derive(Copy, Clone)]
pub enum PieceType{
    // 玉(King)。
    K,
    // 飛車(Rook)。
    R,
    // 角(Bishop)。
    B,
    // 金(Gold)。
    G,
    // 銀(Silver)。
    S,
    // 桂(kNight)。
    N,
    // 香(Lance)。
    L,
    // 歩(Pawn)。
    P,
    // 竜(Promoted Rook)。
    PR,
    // 馬(Promoted Bishop)。
    PB,
    // 全(Promoted Silver)。
    PS,
    // 圭(Promoted kNight)。
    PN,
    // 杏(Promoted Lance)。
    PL,
    // と(Promoted Pawn)。
    PP,
    // 空マス。
    Space,
    // 要素数より1小さい数。エラー値用に使っても可。
    Num
}
impl fmt::Display for PieceType{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use tusin::usi::PieceType::*;
        match *self{
            K => { write!(f,"玉")},
            R => { write!(f,"飛")},
            B => { write!(f,"角")},
            G => { write!(f,"金")},
            S => { write!(f,"銀")},
            N => { write!(f,"桂")},
            L => { write!(f,"香")},
            P => { write!(f,"歩")},
            PB => { write!(f,"竜")},
            PR => { write!(f,"馬")},
            PS => { write!(f,"全")},
            PN => { write!(f,"圭")},
            PL => { write!(f,"杏")},
            PP => { write!(f,"と")},
            Space => { write!(f,"　")},
            Num => { write!(f,"×")},
        }
    }
}
/*
/// 駒種類の一致比較
pub fn match_piece_type(a:&PieceType, b:&PieceType)->bool{
    kms_to_num(a) == kms_to_num(b)
}

/// 駒種類数
pub const KMS_ARRAY_LN : usize = 14;
/// 駒種類
pub const KMS_ARRAY : [KmSyurui;KMS_ARRAY_LN] = [
    KmSyurui::R,// らいおん
    KmSyurui::K,// きりん
    KmSyurui::Z,// ぞう
    KmSyurui::I,// いぬ
    KmSyurui::N,// ねこ
    KmSyurui::U,// うさぎ
    KmSyurui::S,// いのしし
    KmSyurui::H,// ひよこ
    KmSyurui::PK,// ぱわーあっぷきりん
    KmSyurui::PZ,// ぱわーあっぷぞう
    KmSyurui::PN,// ぱわーあっぷねこ
    KmSyurui::PU,// ぱわーあっぷうさぎ
    KmSyurui::PS,// ぱわーあっぷいのしし
    KmSyurui::PH,// ぱわーあっぷひよこ
];
*/

/// 指し手。最大で５桁の文字列。
///
/// * `source_file` - 移動元の筋。
/// * `source_rank` - 移動元の段。
/// * `drop` - 打の場合、打った駒種類。
/// * `destination_file` - 移動先の筋。
/// * `destination_rank` - 移動先の段。
/// * `promotion` - 移動後に成るなら真。
#[derive(Copy,Clone)]
pub struct UsiMovement{
    pub source_file : i8,
    pub source_rank : i8,
    pub drop : PieceType,
    pub destination_file : i8,
    pub destination_rank : i8,
    pub promotion : bool,
}

/// 指し手文字列から、打った駒種類を抽出します。
///
/// # Examples.
/// 
/// * `7g7f`
/// * `B*5e`
/// * `3d3c+`
pub fn read_movement(
    line: &String,
    starts: &mut usize,
    len: usize
) -> UsiMovement {

    // 4文字か5文字あるはず。
    if (len-*starts)<4{
        // 指し手読取終了時にここを通るぜ☆（＾～＾）
        // 残り４文字もない。
        return UsiMovement{
            source_file : -1,
            source_rank : -1,
            drop : PieceType::Space,
            destination_file : -1,
            destination_rank : -1,
            promotion : false,
        };
    }

    let drop : PieceType;
    // 1文字目と2文字目
    match &line[*starts..(*starts+1)]{
        // 1文字目が駒だったら打。2文字目は必ず「*」なはずなので読み飛ばす。
        "R" => { *starts+= 2; drop= PieceType::R },
        "B" => { *starts+= 2; drop= PieceType::B },
        "G" => { *starts+= 2; drop= PieceType::G },
        "S" => { *starts+= 2; drop= PieceType::S },
        "N" => { *starts+= 2; drop= PieceType::N },
        "L" => { *starts+= 2; drop= PieceType::L },
        "P" => { *starts+= 2; drop= PieceType::P },
        _ => {
            // 残りは「筋の数字」、「段のアルファベット」のはず。
            drop= PieceType::Space;
        },
    }

    // 残りは「筋の数字」、「段のアルファベット」のはず。
    UsiMovement{
        source_file : -1,
        source_rank : -1,
        drop : drop,
        destination_file : -1,
        destination_rank : -1,
        promotion : false,
    }
}
