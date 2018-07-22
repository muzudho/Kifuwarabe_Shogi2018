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

/*
/**
 * position コマンド 盤上部分のみ 読取
 */
 pub fn parse_banjo(line:&String, starts:&mut usize, len:usize){

    // 盤部
    let mut suji = SUJI_9;//９筋から右方向へ読取
    let mut dan = DAN_1;
    'ban: while 0<(len-*starts) {
        match &line[*starts..(*starts+1)]{
            "/" => { *starts+=1; suji=SUJI_9; dan+=1; },
            "1" => { *starts+=1;
                UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
            },
            "2" => { *starts+=1;
                UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
            },
            "3" => { *starts+=1;
                UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
            },
            "4" => { *starts+=1;
                for _i_kara in 0..4{
                    UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                }
            },
            "5" => { *starts+=1;
                for _i_kara in 0..5{
                    UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                }
            },
            "6" => { *starts+=1;
                for _i_kara in 0..6{
                    UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                }
            },
            "7" => { *starts+=1;
                for _i_kara in 0..7{
                    UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                }
            },
            "8" => { *starts+=1;
                for _i_kara in 0..8{
                    UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                }
            },
            "9" => { *starts+=1;
                for _i_kara in 0..9{
                    UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                }
            },
            "K" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::R0); suji-=1; },
            "R" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::K0); suji-=1; },
            "B" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Z0); suji-=1; },
            "G" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::I0); suji-=1; },
            "S" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::N0); suji-=1; },
            "N" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::U0); suji-=1; },
            "L" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::S0); suji-=1; },
            "P" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::H0); suji-=1; },
            "k" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::R1); suji-=1; },
            "r" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::K1); suji-=1; },
            "b" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::Z1); suji-=1; },
            "g" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::I1); suji-=1; },
            "s" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::N1); suji-=1; },
            "n" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::U1); suji-=1; },
            "l" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::S1); suji-=1; },
            "p" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::H1); suji-=1; },
            "+" => {
                *starts+=1;
                match &line[*starts..(*starts+1)]{
                    "R" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::PK0); suji-=1; },
                    "B" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::PZ0); suji-=1; },
                    "S" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::PN0); suji-=1; },
                    "N" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::PU0); suji-=1; },
                    "L" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::PS0); suji-=1; },
                    "P" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::PH0); suji-=1; },
                    "r" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::PK1); suji-=1; },
                    "b" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::PZ1); suji-=1; },
                    "s" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::PN1); suji-=1; },
                    "n" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::PU1); suji-=1; },
                    "l" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::PS1); suji-=1; },
                    "p" => { *starts+=1; UCHU_WRAP.write().unwrap().set_ky0_ban_km(suji,dan,Koma::PH1); suji-=1; },
                    _ => { g_writeln(&format!("盤部(0) '{}' だった。", &line[*starts..(*starts+1)])); break 'ban;},
                }                    
            },
            _ => {break 'ban;}, // 盤部正常終了
        }
    }

    // 初期局面ハッシュを作り直す
    let ky_hash = UCHU_WRAP.write().unwrap().create_ky0_hash();
    UCHU_WRAP.write().unwrap().set_ky0_hash( ky_hash );
}
*/