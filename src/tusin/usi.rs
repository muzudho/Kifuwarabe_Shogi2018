use std::fmt;
use memory::uchu::*;

/// 駒種類。先後なしの駒と空白。
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

/// 指し手。最大で５桁の文字列。
///
/// # Members.
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
pub fn parse_movement(
    line: &String,
    starts: &mut usize,
    len: usize
) -> UsiMovement {

    let mut result = UsiMovement{
        source_file : -1,
        source_rank : -1,
        drop : PieceType::Space,
        destination_file : -1,
        destination_rank : -1,
        promotion : false,
    };

    // 4文字か5文字あるはず。
    if (len-*starts)<4{
        // 指し手読取終了時にここを通るぜ☆（＾～＾）
        // 残り４文字もない。
        return result;
    }

    // 1文字目と2文字目
    match &line[*starts..(*starts+1)]{
        // 1文字目が駒だったら打。2文字目は必ず「*」なはずなので読み飛ばす。
        "R" => { *starts+= 2; result.drop= PieceType::R },
        "B" => { *starts+= 2; result.drop= PieceType::B },
        "G" => { *starts+= 2; result.drop= PieceType::G },
        "S" => { *starts+= 2; result.drop= PieceType::S },
        "N" => { *starts+= 2; result.drop= PieceType::N },
        "L" => { *starts+= 2; result.drop= PieceType::L },
        "P" => { *starts+= 2; result.drop= PieceType::P },
        _ => {
            // 残りは「筋の数字」、「段のアルファベット」のはず。
            result.source_file = match &line[*starts..(*starts+1)]{
                "1" => 1,
                "2" => 2,
                "3" => 3,
                "4" => 4,
                "5" => 5,
                "6" => 6,
                "7" => 7,
                "8" => 8,
                "9" => 9,
                _ => {g_writeln(&format!("(1) '{}' だった。", &line[*starts..(*starts+1)])); return result;},
            };
            *starts+=1;

            result.source_rank = match &line[*starts..(*starts+1)]{
                "a" => 1,
                "b" => 2,
                "c" => 3,
                "d" => 4,
                "e" => 5,
                "f" => 6,
                "g" => 7,
                "h" => 8,
                "i" => 9,
                _ => {g_writeln(&format!("(2) '{}' だった。", &line[*starts..(*starts+1)])); return result;},
            };
            *starts+=1;
        },
    }

    // 3文字目
    result.destination_file = match &line[*starts..(*starts+1)]{
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => {g_writeln(&format!("(3) '{}' だった。", &line[*starts..(*starts+1)])); return result;},
    };
    *starts+=1;
    
    // 4文字目
    result.destination_rank = match &line[*starts..(*starts+1)]{
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        "i" => 9,
        _ => {g_writeln(&format!("(4) '{}' だった。", &line[*starts..(*starts+1)])); return result;},
    };
    *starts+=1;
    
    // 5文字に「+」があれば成り。
    if 0<(len-*starts) && &line[*starts..(*starts+1)]=="+" {
        result.promotion = true;
        *starts+=1;
    }

    // 続きにスペース「 」が１つあれば読み飛ばす
    if 0<(len-*starts) && &line[*starts..(*starts+1)]==" " {
        *starts+=1;
    }

    // 残りは「筋の数字」、「段のアルファベット」のはず。成り
    return result;
}






/// 先後付きの駒と空白
#[derive(Copy, Clone)]
pub enum Piece{
    // ▼玉(King)。
    K0,
    // ▼飛車(Rook)。
    R0,
    // ▼角(Bishop)。
    B0,
    // ▼金(Gold)。
    G0,
    // ▼銀(Silver)。
    S0,
    // ▼桂(kNight)。
    N0,
    // ▼香(Lance)。
    L0,
    // ▼歩(Pawn)。
    P0,
    // ▼竜(Promoted Rook)。
    PR0,
    // ▼馬(Promoted Bishop)。
    PB0,
    // ▼全(Promoted Silver)。
    PS0,
    // ▼圭(Promoted kNight)。
    PN0,
    // ▼杏(Promoted Lance)。
    PL0,
    // ▼と(Promoted Pawn)。
    PP0,
    // △玉(King)。
    K1,
    // △飛車(Rook)。
    R1,
    // △角(Bishop)。
    B1,
    // △金(Gold)。
    G1,
    // △銀(Silver)。
    S1,
    // △桂(kNight)。
    N1,
    // △香(Lance)。
    L1,
    // △歩(Pawn)。
    P1,
    // △竜(Promoted Rook)。
    PR1,
    // △馬(Promoted Bishop)。
    PB1,
    // △全(Promoted Silver)。
    PS1,
    // △圭(Promoted kNight)。
    PN1,
    // △杏(Promoted Lance)。
    PL1,
    // △と(Promoted Pawn)。
    PP1,
    // 空マス
    Space,
    // 要素数より1小さい数。該当なしや、エラー値用としても兼用する
    Num
}
impl fmt::Display for Piece{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use tusin::usi::Piece::*;
        match *self{
            K0 => { write!(f,"▼玉")},
            R0 => { write!(f,"▼飛")},
            B0 => { write!(f,"▼角")},
            G0 => { write!(f,"▼金")},
            S0 => { write!(f,"▼銀")},
            N0 => { write!(f,"▼桂")},
            L0 => { write!(f,"▼香")},
            P0 => { write!(f,"▼歩")},
            PR0 => { write!(f,"▼竜")},
            PB0 => { write!(f,"▼馬")},
            PS0 => { write!(f,"▼全")},
            PN0 => { write!(f,"▼圭")},
            PL0 => { write!(f,"▼杏")},
            PP0 => { write!(f,"▼と")},
            K1 => { write!(f,"△玉")},
            R1 => { write!(f,"△飛")},
            B1 => { write!(f,"△角")},
            G1 => { write!(f,"△金")},
            S1 => { write!(f,"△銀")},
            N1 => { write!(f,"△桂")},
            L1 => { write!(f,"△香")},
            P1 => { write!(f,"△歩")},
            PR1 => { write!(f,"△竜")},
            PB1 => { write!(f,"△馬")},
            PS1 => { write!(f,"△全")},
            PN1 => { write!(f,"△圭")},
            PL1 => { write!(f,"△杏")},
            PP1 => { write!(f,"△と")},
            Space => { write!(f,"　　")},
            Num => { write!(f,"××")},
        }
    }
}

pub const FILE0 :i8 = 0;
pub const FILE9 :i8 = 9;
pub const FILE10 :i8 = 10;
pub const RANK0 :i8 = 0;
pub const RANK1 :i8 = 1;
pub const RANK10 :i8 = 10;


pub fn file_rank_to_cell(file:i8, rank:i8)->usize{
    debug_assert!(
            (FILE0<file && file<FILE10)
         && (RANK0<rank && rank<RANK10)
         , "(204)file_rank_to_cell file={},rank={}",file, rank);

    (file*10 + rank) as usize
}

/// TODO position コマンド 盤上部分のみ 読取
pub fn parse_banjo(line:&String, starts:&mut usize, len:usize) -> [Piece;100] {

    use tusin::usi::Piece::Space;
    // 初期局面の盤面
    let mut ban = [
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
    ];

    // 盤部
    use tusin::usi::Piece;
    let mut file = FILE9;//９筋から右方向へ読取
    let mut rank = RANK1;
    'ban: while 0<(len-*starts) {
        match &line[*starts..(*starts+1)]{
            "/" => { *starts+=1; file=FILE9; rank+=1; },
            "1" => { *starts+=1;
                ban[file_rank_to_cell(file,rank)] = Space; file-=1;
            },
            "2" => { *starts+=1;
                ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                ban[file_rank_to_cell(file,rank)] = Space; file-=1;
            },
            "3" => { *starts+=1;
                ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                ban[file_rank_to_cell(file,rank)] = Space; file-=1;
            },
            "4" => { *starts+=1;
                for _i_kara in 0..4{
                    ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                }
            },
            "5" => { *starts+=1;
                for _i_kara in 0..5{
                    ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                }
            },
            "6" => { *starts+=1;
                for _i_kara in 0..6{
                    ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                }
            },
            "7" => { *starts+=1;
                for _i_kara in 0..7{
                    ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                }
            },
            "8" => { *starts+=1;
                for _i_kara in 0..8{
                    ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                }
            },
            "9" => { *starts+=1;
                for _i_kara in 0..9{
                    ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                }
            },
            "K" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::K0; file-=1; },
            "R" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::R0; file-=1; },
            "B" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::B0; file-=1; },
            "G" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::G0; file-=1; },
            "S" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::S0; file-=1; },
            "N" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::N0; file-=1; },
            "L" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::L0; file-=1; },
            "P" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::P0; file-=1; },
            "k" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::K1; file-=1; },
            "r" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::R1; file-=1; },
            "b" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::B1; file-=1; },
            "g" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::G1; file-=1; },
            "s" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::S1; file-=1; },
            "n" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::N1; file-=1; },
            "l" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::L1; file-=1; },
            "p" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::P1; file-=1; },
            "+" => {
                *starts+=1;
                match &line[*starts..(*starts+1)]{
                    "R" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PR0; file-=1; },
                    "B" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PB0; file-=1; },
                    "S" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PS0; file-=1; },
                    "N" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PN0; file-=1; },
                    "L" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PL0; file-=1; },
                    "P" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PP0; file-=1; },
                    "r" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PR1; file-=1; },
                    "b" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PB1; file-=1; },
                    "s" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PS1; file-=1; },
                    "n" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PN1; file-=1; },
                    "l" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PL1; file-=1; },
                    "p" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PP1; file-=1; },
                    _ => { g_writeln(&format!("盤部(0) '{}' だった。", &line[*starts..(*starts+1)])); break 'ban;},
                }                    
            },
            _ => {break 'ban;}, // 盤部正常終了
        }
    }

    // 盤面を返却
    ban
}
