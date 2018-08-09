/**
 * 局面
 *
 * 後手（上手）から見た盤にすると、
 * 筋と段の方向は　数学のデカルト座標の第一象限のＸ軸、Ｙ軸方向と一致する☆（＾～＾）
 *
 * プログラム上に違いは無いが、ソースコードを読むときは　後手（上手）から
 * 盤を想像すること☆（＾～＾）！
 */

use std::*;
use std::collections::*;








/********
 * 先後 *
 ********/
pub const SN_LN : usize = 3;
/**
 * 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
 * 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
 */
#[derive(Clone)]
pub enum Sengo{
    Sen,
    Go,
    // 空升の先後を調べようとした場合等
    Owari,
}
pub const SN_SEN : usize = 0;
pub const SN_GO : usize = 1;
/**
 * 後手（上手）を盤の下側に持ってきて表示するのを基本とするぜ☆（＾～＾）
 */
impl fmt::Display for Sengo{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use memory::ky::Sengo::*;
        match *self{
            Sen => { write!(f,"▼")},
            Go => { write!(f,"△")},
            Owari => { write!(f,"×")},
        }
    }
}
/**
 * 先後の一致比較
 */
pub fn match_sn(a:&Sengo, b:&Sengo)->bool{
    sn_to_num(a) == sn_to_num(b)
}

pub const SN_ARRAY_LN : usize = 2;
pub const SN_ARRAY : [Sengo;SN_ARRAY_LN] = [
    Sengo::Sen,
    Sengo::Go,
];


/************
 * 自分相手 *
 ************/
// 先後とは別物
#[allow(dead_code)]
pub const JIAI_LN : usize = 3;
/**
 * 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
 * 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
 */
#[derive(Clone)]
pub enum Jiai{
    Ji,
    Ai,
    #[allow(dead_code)]
    Owari,
}
#[allow(dead_code)]
pub const JIAI_JI : usize = 0;
#[allow(dead_code)]
pub const JIAI_AI : usize = 1;
impl fmt::Display for Jiai {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        use memory::ky::Jiai::*;
        match *self{
            Ji => { write!(f,"自")},
            Ai => { write!(f,"相")},
            Owari => { write!(f,"×")},
        }
    }
}
/**
 * 一致比較
 */
pub fn match_jiai(a:&Jiai, b:&Jiai)->bool{
    jiai_to_num(a) == jiai_to_num(b)
}

#[allow(dead_code)]
pub const JIAI_ARRAY_LN : usize = 2;
#[allow(dead_code)]
pub const JIAI_ARRAY : [Jiai;JIAI_ARRAY_LN] = [
    Jiai::Ji,
    Jiai::Ai,
];





/******
 * 陣 *
 ******/

/**
 * 先手陣
 */
pub struct SenteJin{
}
impl SenteJin{
    pub fn to_elm()->Vec<umasu>{
        vec![
            91,81,71,61,51,41,31,21,11,
            92,82,72,62,52,42,32,22,12,
            93,83,73,63,53,43,33,23,13,
        ]
    }
}

/**
 * 後手陣
 */
pub struct GoteJin{
}
impl GoteJin{
    pub fn to_elm()->Vec<umasu>{
        vec![
            91,81,71,61,51,41,31,21,11,
            92,82,72,62,52,42,32,22,12,
            93,83,73,63,53,43,33,23,13,
        ]
    }
}









/******************
 * 盤、升、筋、段 *
 ******************/
/*
 * 盤の符号は、後手番から見る
 *
 *
 * 19  29  39  49  59  69  79  89  99
 * 18  28  38  48  58  68  78  88  98
 * 17  27  37  47  57  67  77  87  97
 * 16  26  36  46  56  66  76  86  96
 * 15  25  35  45  55  65  75  85  95
 * 14  24  34  44  54  64  74  84  94
 * 13  23  33  43  53  63  73  83  93
 * 12  22  32  42  52  62  72  82  92
 * 11  21  31  41  51  61  71  81  91
 *
 */
/**
 * 盤を回転するのに使うぜ☆（＾～＾）
 */
pub const BAN_MIN :usize = 11;
/**
 * 盤を回転するのに使うぜ☆（＾～＾）
 */
pub const BAN_MAX :usize = 99;
/**
 * 盤のヨコ幅、タテ幅。
 * 筋と段は x,y とは逆方向なので、幅も左端、下端を指す。
 */
//pub const BAN_W :i8 = 9;
//pub const BAN_H :i8 = 9;
pub const BAN_SIZE :usize = 100;
// 1辺の長さ
//pub const BAN_LINE :usize = 10;
/**
 * 筋、段は 1 から始まる、という明示。
 * 増減はよく使うので u8 ではなく i8 にした。
 */
pub const SUJI_0 :i8 = 0;
pub const SUJI_1 :i8 = 1;
#[allow(dead_code)]
pub const SUJI_9 :i8 = 9;
pub const SUJI_10 :i8 = 10;
pub const DAN_0 :i8 = 0;
pub const DAN_1 :i8 = 1;
pub const DAN_2 :i8 = 2;
pub const DAN_3 :i8 = 3;
pub const DAN_4 :i8 = 4;
#[allow(dead_code)]
pub const DAN_5 :i8 = 5;
pub const DAN_6 :i8 = 6;
pub const DAN_7 :i8 = 7;
pub const DAN_8 :i8 = 8;//うさぎの打てる段の上限
#[allow(dead_code)]
pub const DAN_9 :i8 = 9;
pub const DAN_10 :i8 = 10;
/**
 * 升番号 0～99。
 * 10の位を筋、1の位を段とする。0筋、0段は未使用（番兵として使用）
 * 該当なしの場合 0 を使う
 */
 #[allow(non_camel_case_types)]
pub type umasu = usize;
/**
 * 升の検索等で、該当なしの場合
 */
pub const MASU_0 : umasu = 0;

/**
 * 指し手。打の場合のsrc
 */
pub const SS_SRC_DA : umasu = 0;




/******
 * 駒 *
 ******/
/// 先後付き駒

/// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const MG_MAX : usize = 18;
pub const KM_LN : usize = 30;
/// 先後付きの駒と空白
#[derive(Copy, Clone)]
pub enum Koma{
    // ▼らいおん
    R0,
    // ▼きりん
    K0,
    // ▼ぞう
    Z0,
    // ▼いぬ
    I0,
    // ▼ねこ
    N0,
    // ▼うさぎ
    U0,
    // ▼いのしし
    S0,
    // ▼ひよこ
    H0,
    // ▼ぱわーあっぷきりん
    PK0,
    // ▼ぱわーあっぷぞう
    PZ0,
    // ▼ぱわーあっぷねこ
    PN0,
    // ▼ぱわーあっぷうさぎ
    PU0,
    // ▼ぱわーあっぷいのしし
    PS0,
    // ▼ぱわーあっぷひよこ
    PH0,
    // △ライオン
    R1,
    // △キリン
    K1,
    // △ゾウ
    Z1,
    // △イヌ
    I1,
    // △ネコ
    N1,
    // △ウサギ
    U1,
    // △イノシシ
    S1,
    // △ヒヨコ
    H1,
    // △パワーアップキリン
    PK1,
    // △パワーアップゾウ
    PZ1,
    // △パワーアップネコ
    PN1,
    // △パワーアップウサギ
    PU1,
    // △パワーアップイノシシ
    PS1,
    // △パワーアップヒヨコ
    PH1,
    // 空マス
    Kara,
    // 要素数より1小さい数。該当なしや、エラー値用としても兼用する
    Owari
}
impl fmt::Display for Koma{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use memory::ky::Koma::*;
        match *self{
            R0 => { write!(f,"▼ら")},
            K0 => { write!(f,"▼き")},
            Z0 => { write!(f,"▼ぞ")},
            I0 => { write!(f,"▼い")},
            N0 => { write!(f,"▼ね")},
            U0 => { write!(f,"▼う")},
            S0 => { write!(f,"▼し")},
            H0 => { write!(f,"▼ひ")},
            PK0 => { write!(f,"▼PK")},
            PZ0 => { write!(f,"▼PZ")},
            PN0 => { write!(f,"▼PN")},
            PU0 => { write!(f,"▼PU")},
            PS0 => { write!(f,"▼PS")},
            PH0 => { write!(f,"▼PH")},
            R1 => { write!(f,"△ラ")},
            K1 => { write!(f,"△キ")},
            Z1 => { write!(f,"△ゾ")},
            I1 => { write!(f,"△イ")},
            N1 => { write!(f,"△ネ")},
            U1 => { write!(f,"△ウ")},
            S1 => { write!(f,"△シ")},
            H1 => { write!(f,"△ヒ")},
            PK1 => { write!(f,"△pk")},
            PZ1 => { write!(f,"△pz")},
            PN1 => { write!(f,"△pn")},
            PU1 => { write!(f,"△pu")},
            PS1 => { write!(f,"△ps")},
            PH1 => { write!(f,"△ph")},
            Kara => { write!(f,"　　")},
            Owari => { write!(f,"××")},
        }
    }
}
/**
 * 駒の一致比較
 */
pub fn match_km(a:&Koma, b:&Koma)->bool{
    km_to_num(a) == km_to_num(b)
}

#[allow(dead_code)]
pub const KM_ARRAY_HALF_LN : usize = 14;
pub const KM_ARRAY_LN : usize = 28;
pub const KM_ARRAY : [Koma;KM_ARRAY_LN] = [
    Koma::R0,// らいおん
    Koma::K0,// きりん
    Koma::Z0,// ぞう
    Koma::I0,// いぬ
    Koma::N0,// ねこ
    Koma::U0,// うさぎ
    Koma::S0,// いのしし
    Koma::H0,// ひよこ
    Koma::PK0,// ぱわーあっぷきりん
    Koma::PZ0,// ぱわーあっぷぞう
    Koma::PN0,// ぱわーあっぷねこ
    Koma::PU0,// ぱわーあっぷうさぎ
    Koma::PS0,// ぱわーあっぷいのしし
    Koma::PH0,// ぱわーあっぷひよこ
    Koma::R1,// らいおん
    Koma::K1,// きりん
    Koma::Z1,// ぞう
    Koma::I1,// いぬ
    Koma::N1,// ねこ
    Koma::U1,// うさぎ
    Koma::S1,// いのしし
    Koma::H1,// ひよこ
    Koma::PK1,// ぱわーあっぷきりん
    Koma::PZ1,// ぱわーあっぷぞう
    Koma::PN1,// ぱわーあっぷねこ
    Koma::PU1,// ぱわーあっぷうさぎ
    Koma::PS1,// ぱわーあっぷいのしし
    Koma::PH1,// ぱわーあっぷひよこ
];
#[allow(dead_code)]
pub const SN_KM_ARRAY : [[Koma;KM_ARRAY_HALF_LN];SN_LN] = [
    [
        Koma::R0,// らいおん
        Koma::K0,// きりん
        Koma::Z0,// ぞう
        Koma::I0,// いぬ
        Koma::N0,// ねこ
        Koma::U0,// うさぎ
        Koma::S0,// いのしし
        Koma::H0,// ひよこ
        Koma::PK0,// ぱわーあっぷきりん
        Koma::PZ0,// ぱわーあっぷぞう
        Koma::PN0,// ぱわーあっぷねこ
        Koma::PU0,// ぱわーあっぷうさぎ
        Koma::PS0,// ぱわーあっぷいのしし
        Koma::PH0,// ぱわーあっぷひよこ
    ],
    [
        Koma::R1,// らいおん
        Koma::K1,// きりん
        Koma::Z1,// ぞう
        Koma::I1,// いぬ
        Koma::N1,// ねこ
        Koma::U1,// うさぎ
        Koma::S1,// いのしし
        Koma::H1,// ひよこ
        Koma::PK1,// ぱわーあっぷきりん
        Koma::PZ1,// ぱわーあっぷぞう
        Koma::PN1,// ぱわーあっぷねこ
        Koma::PU1,// ぱわーあっぷうさぎ
        Koma::PS1,// ぱわーあっぷいのしし
        Koma::PH1,// ぱわーあっぷひよこ
    ],
    [
        Koma::Owari,// らいおん
        Koma::Owari,// きりん
        Koma::Owari,// ぞう
        Koma::Owari,// いぬ
        Koma::Owari,// ねこ
        Koma::Owari,// うさぎ
        Koma::Owari,// いのしし
        Koma::Owari,// ひよこ
        Koma::Owari,// ぱわーあっぷきりん
        Koma::Owari,// ぱわーあっぷぞう
        Koma::Owari,// ぱわーあっぷねこ
        Koma::Owari,// ぱわーあっぷうさぎ
        Koma::Owari,// ぱわーあっぷいのしし
        Koma::Owari,// ぱわーあっぷひよこ
    ],
];










/**********
 * 駒種類 *
 **********/
pub const KMS_LN : usize = 16;
// 駒の動ける方向数、終端子込み
pub const KM_UGOKI_LN : usize = 9;
// 先後なしの駒と空白
#[derive(Copy, Clone)]
pub enum KmSyurui{
    // らいおん
    R,
    // きりん
    K,
    // ぞう
    Z,
    // いぬ
    I,
    // ねこ
    N,
    // うさぎ
    U,
    // いのしし
    S,
    // ひよこ
    H,
    // ぱわーあっぷきりん
    PK,
    // ぱわーあっぷぞう
    PZ,
    // ぱわーあっぷねこ
    PN,
    // ぱわーあっぷうさぎ
    PU,
    // ぱわーあっぷいのしし
    PS,
    // ぱわーあっぷひよこ
    PH,
    // 空マス
    Kara,
    // 要素数より1小さい数。エラー値用に使っても可
    Owari
}
impl fmt::Display for KmSyurui{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use memory::ky::KmSyurui::*;
        match *self{
            R => { write!(f,"ら")},
            K => { write!(f,"き")},
            Z => { write!(f,"ぞ")},
            I => { write!(f,"い")},
            N => { write!(f,"ね")},
            U => { write!(f,"う")},
            S => { write!(f,"い")},
            H => { write!(f,"ひ")},
            PK => { write!(f,"PK")},
            PZ => { write!(f,"PZ")},
            PN => { write!(f,"PN")},
            PU => { write!(f,"PU")},
            PS => { write!(f,"PS")},
            PH => { write!(f,"PH")},
            Kara => { write!(f,"　")},
            Owari => { write!(f,"×")},
        }
    }
}
/**
 * 駒種類の一致比較
 */
pub fn match_kms(a:&KmSyurui, b:&KmSyurui)->bool{
    kms_to_num(a) == kms_to_num(b)
}

// 駒種類数
pub const KMS_ARRAY_LN : usize = 14;
// 駒種類
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

/// 非成 駒種類数
#[allow(dead_code)]
pub const KMS_NPRO_ARRAY_LN : usize = 8;
/// 非成 駒種類
#[allow(dead_code)]
pub const KMS_NPRO_ARRAY : [KmSyurui;KMS_NPRO_ARRAY_LN] = [
    KmSyurui::R,// らいおん
    KmSyurui::K,// きりん
    KmSyurui::Z,// ぞう
    KmSyurui::I,// いぬ
    KmSyurui::N,// ねこ
    KmSyurui::U,// うさぎ
    KmSyurui::S,// いのしし
    KmSyurui::H,// ひよこ
];

/// 成 駒種類数
#[allow(dead_code)]
pub const KMS_PRO_ARRAY_LN : usize = 6;
/// 成 駒種類
#[allow(dead_code)]
pub const KMS_PRO_ARRAY : [KmSyurui;KMS_PRO_ARRAY_LN] = [
    KmSyurui::PK,// ぱわーあっぷきりん
    KmSyurui::PZ,// ぱわーあっぷぞう
    KmSyurui::PN,// ぱわーあっぷねこ
    KmSyurui::PU,// ぱわーあっぷうさぎ
    KmSyurui::PS,// ぱわーあっぷいのしし
    KmSyurui::PH,// ぱわーあっぷひよこ
];

// 持駒種類数
pub const MGS_ARRAY_LN : usize = 7;
// 持駒種類
pub const MGS_ARRAY : [KmSyurui;MGS_ARRAY_LN] = [
    KmSyurui::K,
    KmSyurui::Z,
    KmSyurui::I,
    KmSyurui::N,
    KmSyurui::U,
    KmSyurui::S,
    KmSyurui::H,
];

/// 駒種類集合
pub struct KmsSyugo {
    num_syugo : HashSet<usize>,
}
impl KmsSyugo {
    /**
     * 全ての元を含む
     */
    pub fn new_all() -> KmsSyugo {
        let mut num_syugo1 : HashSet<usize> = HashSet::new();
        for kms in KMS_ARRAY.iter() {
            num_syugo1.insert( kms_to_num(kms) );
        }
        let kms_syugo = KmsSyugo {
            num_syugo : num_syugo1,
        };
        kms_syugo
    }
    pub fn remove( &mut self, kms:&KmSyurui ) {
        self.num_syugo.remove( &kms_to_num(kms) );
    }
}
















/********
 * 先後 *
 ********/
pub fn sn_to_num(sn:&Sengo)->usize{
    use memory::ky::Sengo::*;
    match *sn {
        Sen   => 0,
        Go    => 1,
        Owari => 2,
    }
}
pub fn hanten_sn(sn:&Sengo)->Sengo{
    use memory::ky::Sengo::*;
    match *sn {
        Sen   => Go,
        Go    => Sen,
        Owari => Owari,
    }    
}

/************
 * 自分相手 *
 ************/
pub fn jiai_to_num(jiai:&Jiai)->usize{
    use memory::ky::Jiai::*;
    match *jiai {
        Ji    => 0,
        Ai    => 1,
        Owari => 2,
    }
}
#[allow(dead_code)]
pub fn hanten_jiai(jiai:&Jiai)->Jiai{
    use memory::ky::Jiai::*;
    match *jiai {
        Ji    => Ai,
        Ai    => Ji,
        Owari => Owari,
    }    
}

/******************
 * 盤、升、筋、段 *
 ******************/

 /**
  * umasu は 後手から見た、将棋盤座標
  *
  * 19 29 39 ...
  * 18 28 38
  * 17 27 37
  * ...
  */

pub fn ms_to_suji_dan(ms:umasu)->(i8,i8){
    //assert_banjo_ms(ms,"(203)Ｍs_to_suji_dan");
    (
        (ms/10) as i8,
        (ms%10) as i8
    )
}
pub fn suji_dan_to_ms(suji:i8, dan:i8)->umasu{
    debug_assert!(
            (SUJI_0<suji && suji<SUJI_10)
         && ( DAN_0< dan &&  dan< DAN_10)
         , "(204)suji_dan_to_ms suji={},dan={}",suji, dan);

    (suji*10 + dan) as umasu
}
/**
 * ハッシュ値を作る
 */
pub fn push_ms_to_hash(hash:u64, ms:umasu) -> u64 {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    (hash<<7) + ms as u64
}
/**
 * ハッシュ値から作る
 */
pub fn pop_ms_from_hash(hash:u64) -> (u64, umasu) {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    let ms_num =(hash & 0b1111111)as umasu;
    (hash>>7, ms_num)
}

/**
 * 指し手のために、段をアルファベットにすることを想定
 */
pub fn num_to_lower_case(num:i8)->&'static str{
    match num{
        1 =>{"a"},
        2 =>{"b"},
        3 =>{"c"},
        4 =>{"d"},
        5 =>{"e"},
        6 =>{"f"},
        7 =>{"g"},
        8 =>{"h"},
        9 =>{"i"},
        _ =>{panic!("[{}] to lower error.",num)},
    }
}
/****************************************************
 * 先手であれば、後手のように番号を振った座標に変換 *
 ****************************************************/
pub fn kaiten180_ms_by_ms_sn(ms:umasu, sn:&Sengo) -> umasu {
    use memory::ky::Sengo::*;
    match *sn {
        Sen   => {
            BAN_MAX - ms + BAN_MIN
        },
        _ => {
            ms
        },
    }    
}


/**************
 * 先後付き駒 *
 **************/

/// 先後付き駒の数値化。
pub fn km_to_num(km:&Koma) -> usize{
    use memory::ky::Koma::*;
    match *km {
        R0 =>0,
        K0 =>1,
        Z0 =>2,
        I0 =>3,
        N0 =>4,
        U0 =>5,
        S0 =>6,
        H0 =>7,
        PK0 =>8,
        PZ0 =>9,
        PN0 =>10,
        PU0 =>11,
        PS0 =>12,
        PH0 =>13,
        R1 =>14,
        K1 =>15,
        Z1 =>16,
        I1 =>17,
        N1 =>18,
        U1 =>19,
        S1 =>20,
        H1 =>21,
        PK1 =>22,
        PZ1 =>23,
        PN1 =>24,
        PU1 =>25,
        PS1 =>26,
        PH1 =>27,
        Kara =>28,
        Owari =>29,
    }
}
pub fn num_to_km(km_num:usize) -> Koma{
    use memory::ky::Koma::*;
    match km_num {
        0 =>R0,
        1 =>K0,
        2 =>Z0,
        3 =>I0,
        4 =>N0,
        5 =>U0,
        6 =>S0,
        7 =>H0,
        8 =>PK0,
        9 =>PZ0,
        10 =>PN0,
        11 =>PU0,
        12 =>PS0,
        13 =>PH0,
        14 =>R1,
        15 =>K1,
        16 =>Z1,
        17 =>I1,
        18 =>N1,
        19 =>U1,
        20 =>S1,
        21 =>H1,
        22 =>PK1,
        23 =>PZ1,
        24 =>PN1,
        25 =>PU1,
        26 =>PS1,
        27 =>PH1,
        28 =>Kara,
        _ =>Owari,
    }
}
/// ハッシュ値を作る
pub fn push_km_to_hash(hash:u64, km:&Koma) -> u64 {
    // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
    (hash<<5) + km_to_num(km) as u64
}
/// ハッシュ値から作る
pub fn pop_km_from_hash(hash:u64) -> (u64,Koma) {
    // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
    let km_num = num_to_km( (hash & 0b11111) as usize);
    (hash>>5, km_num)
}
/// 駒→成駒　（成れない駒は、そのまま）
pub fn km_to_prokm(km:&Koma) -> Koma{
    use memory::ky::Koma::*;
    match *km {
        R0 =>R0,
        K0 =>PK0,
        Z0 =>PZ0,
        I0 =>I0,
        N0 =>PN0,
        U0 =>PU0,
        S0 =>PS0,
        H0 =>PH0,
        PK0 =>PK0,
        PZ0 =>PZ0,
        PN0 =>PN0,
        PU0 =>PU0,
        PS0 =>PS0,
        PH0 =>PH0,
        R1 =>R1,
        K1 =>PK1,
        Z1 =>PZ1,
        I1 =>I1,
        N1 =>PN1,
        U1 =>PU1,
        S1 =>PS1,
        H1 =>PH1,
        PK1 =>PK1,
        PZ1 =>PZ1,
        PN1 =>PN1,
        PU1 =>PU1,
        PS1 =>PS1,
        PH1 =>PH1,
        Kara =>Kara,
        Owari =>Owari,
    }
}
/// 成駒→駒
pub fn prokm_to_km(km:&Koma) -> Koma{
    use memory::ky::Koma::*;
    match *km {
        R0 =>R0,
        K0 =>K0,
        Z0 =>Z0,
        I0 =>I0,
        N0 =>N0,
        U0 =>U0,
        S0 =>S0,
        H0 =>H0,
        PK0 =>K0,
        PZ0 =>Z0,
        PN0 =>N0,
        PU0 =>U0,
        PS0 =>S0,
        PH0 =>H0,
        R1 =>R1,
        K1 =>K1,
        Z1 =>Z1,
        I1 =>I1,
        N1 =>N1,
        U1 =>U1,
        S1 =>S1,
        H1 =>H1,
        PK1 =>K1,
        PZ1 =>Z1,
        PN1 =>N1,
        PU1 =>U1,
        PS1 =>S1,
        PH1 =>H1,
        Kara =>Kara,
        Owari =>Owari,
    }
}
/// 駒→長い利きの有無
pub fn km_is_nagaikiki(km:&Koma) -> bool{
    kms_is_nagaikiki( &km_to_kms( km ) )
}
/// 先後付き駒→駒種類
pub fn km_to_sn_kms(km:&Koma)->(Sengo,KmSyurui){
    use memory::ky::Koma;
    use memory::ky::Koma::*;
    use memory::ky::Sengo::*;
    use memory::ky::KmSyurui;
    use memory::ky::KmSyurui::*;
    match *km{
        R0  => { (Sen,R ) },
        K0  => { (Sen,K ) },
        Z0  => { (Sen,Z ) },
        I0  => { (Sen,I ) },
        N0  => { (Sen,N ) },
        U0  => { (Sen,U ) },
        S0  => { (Sen,S ) },
        H0  => { (Sen,H ) },
        PK0 => { (Sen,PK) },
        PZ0 => { (Sen,PZ) },
        PN0 => { (Sen,PN) },
        PU0 => { (Sen,PU) },
        PS0 => { (Sen,PS) },
        PH0 => { (Sen,PH) },
        R1  => { (Go ,R ) },
        K1  => { (Go ,K ) },
        Z1  => { (Go ,Z ) },
        I1  => { (Go ,I ) },
        N1  => { (Go ,N ) },
        U1  => { (Go ,U ) },
        S1  => { (Go ,S ) },
        H1  => { (Go ,H ) },
        PK1 => { (Go ,PK) },
        PZ1 => { (Go ,PZ) },
        PN1 => { (Go ,PN) },
        PU1 => { (Go ,PU) },
        PS1 => { (Go ,PS) },
        PH1 => { (Go ,PH) },
        Koma::Kara  => { (Sengo::Owari,KmSyurui::Kara ) },
        Koma::Owari => { (Sengo::Owari,KmSyurui::Owari) },
    }
}
/// 先後付き駒　を　先後　へ変換。
#[allow(dead_code)]
pub fn km_to_sn(km:&Koma)->Sengo{
    use memory::ky::Koma::*;
    use memory::ky::Sengo::*;
    match *km{
        R0 => { Sen},
        K0 => { Sen},
        Z0 => { Sen},
        I0 => { Sen},
        N0 => { Sen},
        U0 => { Sen},
        S0 => { Sen},
        H0 => { Sen},
        PK0 => { Sen},
        PZ0 => { Sen},
        PN0 => { Sen},
        PU0 => { Sen},
        PS0 => { Sen},
        PH0 => { Sen},
        R1 => { Go},
        K1 => { Go},
        Z1 => { Go},
        I1 => { Go},
        N1 => { Go},
        U1 => { Go},
        S1 => { Go},
        H1 => { Go},
        PK1 => { Go},
        PZ1 => { Go},
        PN1 => { Go},
        PU1 => { Go},
        PS1 => { Go},
        PH1 => { Go},
        Kara => { Sengo::Owari},
        Koma::Owari => { Sengo::Owari},
    }
}
/// 先後付き駒→駒種類
pub fn km_to_kms(km:&Koma)->KmSyurui{
    use memory::ky::Koma;
    use memory::ky::Koma::*;
    use memory::ky::KmSyurui;
    use memory::ky::KmSyurui::*;
    match *km{
        R0 => { R},
        K0 => { K},
        Z0 => { Z},
        I0 => { I},
        N0 => { N},
        U0 => { U},
        S0 => { S},
        H0 => { H},
        PK0 => { PK},
        PZ0 => { PZ},
        PN0 => { PN},
        PU0 => { PU},
        PS0 => { PS},
        PH0 => { PH},
        R1 => { R},
        K1 => { K},
        Z1 => { Z},
        I1 => { I},
        N1 => { N},
        U1 => { U},
        S1 => { S},
        H1 => { H},
        PK1 => { PK},
        PZ1 => { PZ},
        PN1 => { PN},
        PU1 => { PU},
        PS1 => { PS},
        PH1 => { PH},
        Koma::Kara => { KmSyurui::Kara},
        Koma::Owari => { KmSyurui::Owari},
    }
}
/// 先後付き駒　を　持ち駒種類　へ変換。
/// 持ち駒にするので、先後は反転するぜ☆（＾～＾）
pub fn km_to_mg(km_cap:Koma)->Koma{
    use memory::ky::Koma::*;
    match km_cap{
        R0 => { Owari},
        K0 => { K1},
        Z0 => { Z1},
        I0 => { I1},
        N0 => { N1},
        U0 => { U1},
        S0 => { S1},
        H0 => { H1},
        PK0 => { K1},
        PZ0 => { Z1},
        PN0 => { N1},
        PU0 => { U1},
        PS0 => { S1},
        PH0 => { H1},
        R1 => { Owari},
        K1 => { K0},
        Z1 => { Z0},
        I1 => { I0},
        N1 => { N0},
        U1 => { U0},
        S1 => { S0},
        H1 => { H0},
        PK1 => { K0},
        PZ1 => { Z0},
        PN1 => { N0},
        PU1 => { U0},
        PS1 => { S0},
        PH1 => { H0},
        Kara => { Owari},
        Owari => { Owari},
    }
}

/**********
 * 駒種類 *
 **********/

/// 駒種類の数値化
pub fn kms_to_num(kms:&KmSyurui) -> usize{
    use memory::ky::KmSyurui::*;
    match *kms {
        R=>0,
        K=>1,
        Z=>2,
        I=>3,
        N=>4,
        U=>5,
        S=>6,
        H=>7,
        PK=>8,
        PZ=>9,
        PN=>10,
        PU=>11,
        PS=>12,
        PH=>13,
        Kara=>14,
        Owari=>15,
    }
}
/**
 * 数値の駒種類化
 */
pub fn num_to_kms(n:usize) -> KmSyurui {
    use memory::ky::KmSyurui::*;
    match n {
        0=>R,
        1=>K,
        2=>Z,
        3=>I,
        4=>N,
        5=>U,
        6=>S,
        7=>H,
        8=>PK,
        9=>PZ,
        10=>PN,
        11=>PU,
        12=>PS,
        13=>PH,
        14=>Kara,
        _=>Owari,
    }
}
/**
 * ハッシュ値を作る
 */
pub fn push_kms_to_hash(hash:u64, kms:&KmSyurui) -> u64 {
    // 使ってるのは16駒種類番号ぐらいなんで、16(=2^4) あれば十分
    (hash<<4) + kms_to_num(kms) as u64
}
/**
 * ハッシュ値から作る
 */
pub fn pop_kms_from_hash(hash:u64) -> (u64,KmSyurui) {
    // 使ってるのは16駒種類番号ぐらいなんで、16(=2^4) あれば十分
    let kms_num = num_to_kms( (hash & 0b1111) as usize);
    (hash>>4, kms_num)
}
// 駒種類→｛　成駒,（不成駒、それ以外）　｝
pub fn kms_is_pro(kms:&KmSyurui) -> bool{
    use memory::ky::KmSyurui::*;
    match *kms {
        R=>false,
        K=>false,
        Z=>false,
        I=>false,
        N=>false,
        U=>false,
        S=>false,
        H=>false,
        PK=>true,
        PZ=>true,
        PN=>true,
        PU=>true,
        PS=>true,
        PH=>true,
        Kara=>false,
        Owari=>false,
    }
}
// 成り駒種類→成る前の駒種類。成り駒でなければ、空に戻る。
pub fn prokms_to_kms(kms:&KmSyurui) -> KmSyurui {
    use memory::ky::KmSyurui::*;
    match *kms {
        R=>Kara,
        K=>Kara,
        Z=>Kara,
        I=>Kara,
        N=>Kara,
        U=>Kara,
        S=>Kara,
        H=>Kara,
        PK=>K,
        PZ=>Z,
        PN=>N,
        PU=>U,
        PS=>S,
        PH=>H,
        Kara=>Kara,
        Owari=>Owari,
    }
}
/**
 * 駒種類→｛　長い利きの駒か否か　｝
 * 合い駒で防ぎえる可能性があれば真
 */
pub fn kms_is_nagaikiki(kms:&KmSyurui) -> bool {
    use memory::ky::KmSyurui::*;
    match *kms {
        R=>false,
        K=>true,
        Z=>true,
        I=>false,
        N=>false,
        U=>false,
        S=>true,
        H=>false,
        PK=>true,
        PZ=>true,
        PN=>false,
        PU=>false,
        PS=>false,
        PH=>false,
        Kara=>false,
        Owari=>false,
    }
}
/**
 * 成れる駒
 */
pub fn kms_can_pro(kms:&KmSyurui) -> bool {
    use memory::ky::KmSyurui::*;
    match *kms {
        R=>false,
        K=>true,
        Z=>true,
        I=>false,
        N=>true,
        U=>true,
        S=>true,
        H=>true,
        PK=>false,
        PZ=>false,
        PN=>false,
        PU=>false,
        PS=>false,
        PH=>false,
        Kara=>false,
        Owari=>false,
    }
}
/**
 * 打てる駒
 */
pub fn kms_can_da(kms:&KmSyurui) -> bool {
    use memory::ky::KmSyurui::*;
    match *kms {
        R=>false,
        K=>true,
        Z=>true,
        I=>true,
        N=>true,
        U=>true,
        S=>true,
        H=>true,
        PK=>false,
        PZ=>false,
        PN=>false,
        PU=>false,
        PS=>false,
        PH=>false,
        Kara=>false,
        Owari=>false,
    }
}
// 先後＆駒種類→先後付き駒
pub fn sn_kms_to_km(sn:&Sengo, kms:&KmSyurui)->Koma{
    use memory::ky::KmSyurui;
    use memory::ky::Koma;
    match *sn{
        Sengo::Sen=>
            match *kms{
                KmSyurui::R => Koma::R0,
                KmSyurui::K => Koma::K0,
                KmSyurui::Z => Koma::Z0,
                KmSyurui::I => Koma::I0,
                KmSyurui::N => Koma::N0,
                KmSyurui::U => Koma::U0,
                KmSyurui::S => Koma::S0,
                KmSyurui::H => Koma::H0,
                KmSyurui::PK => Koma::PK0,
                KmSyurui::PZ => Koma::PZ0,
                KmSyurui::PN => Koma::PN0,
                KmSyurui::PU => Koma::PU0,
                KmSyurui::PS => Koma::PS0,
                KmSyurui::PH => Koma::PH0,
                _=>Koma::Owari
            }
        ,
        Sengo::Go=>
            match *kms{
                KmSyurui::R => Koma::R1,
                KmSyurui::K => Koma::K1,
                KmSyurui::Z => Koma::Z1,
                KmSyurui::I => Koma::I1,
                KmSyurui::N => Koma::N1,
                KmSyurui::U => Koma::U1,
                KmSyurui::S => Koma::S1,
                KmSyurui::H => Koma::H1,
                KmSyurui::PK => Koma::PK1,
                KmSyurui::PZ => Koma::PZ1,
                KmSyurui::PN => Koma::PN1,
                KmSyurui::PU => Koma::PU1,
                KmSyurui::PS => Koma::PS1,
                KmSyurui::PH => Koma::PH1,
                _=>Koma::Owari
            }
        ,
        Sengo::Owari => Koma::Owari
        ,
    }
}



















/**
 * 局面ハッシュ種
 * ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
 */
pub struct KyHashSeed {
    // 盤上の駒
    pub km : [[u64;KM_LN];BAN_SIZE],
    // 持ち駒
    pub mg : [[u64;MG_MAX];KM_LN],
    // 先後
    pub sn : [u64;SN_LN],
}

// 局面
pub struct Kyokumen{
    /**
     * 10の位を筋、1の位を段とする。
     * 0筋、0段は未使用
     */
    ban : [Koma; BAN_SIZE],
    /**
     * 持ち駒数。持ち駒に使える、成らずの駒の部分だけ使用。
     * 増減させたいので、u8 ではなく i8。
     */
    pub mg : [i8; KM_LN],
    /**
     * らいおんの位置
     * [先後]
     */
    pub ms_r : [umasu; SN_LN]
}
impl Kyokumen{
    pub fn new()->Kyokumen{
        use memory::ky::Koma::*;
         Kyokumen{
                // 盤上
                ban:[
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                ],
                // 持ち駒数
                mg:[
                    // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
                        0,   0,   0,   0,   0,   0,   0,   0,     0,     0,     0,     0,     0,     0,
                    // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
                        0,   0,   0,   0,   0,   0,   0,   0,     0,     0,     0,     0,     0,     0,
                    // 空マス, 終わり,
                            0,      0,
                ],
                ms_r:[0,0,0],
            }
    }
    pub fn clear(&mut self){
        use memory::ky::Koma::Kara;
        self.ban = [
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
        ];
        self.mg = [
            // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
                0,   0,   0,   0,   0,   0,   0,   0,     0,     0,     0,     0,     0,     0,
            // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
                0,   0,   0,   0,   0,   0,   0,   0,     0,     0,     0,     0,     0,     0,
            // 空マス, 終わり,
                    0,      0,
        ];
    }
    /**
     * 歩が置いてあるか確認
     */
    pub fn exists_fu_by_sn_suji( &self, sn:&Sengo, suji:i8 ) -> bool {
        for dan in DAN_1..DAN_10 {
            let ms = suji_dan_to_ms( suji, dan );
            let km = self.get_km_by_ms( ms );
            let (sn_km,kms) = km_to_sn_kms( &km );
            if match_sn( &sn_km, sn ) && match_kms( &kms, &KmSyurui::H ) {
                return true;
            }
        }
        false
    }
    /**
     * 升で指定して駒を取る
     */
    pub fn get_km_by_ms(&self, ms:umasu)->Koma{
        self.ban[ms]
    }
    /**
     * 升で指定して駒を置く
     */
    pub fn set_km_by_ms(&mut self, ms:umasu, km:Koma){
        self.ban[ms] = km;
        use memory::ky::Sengo::*;
        match km{
            Koma::R0 => { self.ms_r[Sen as usize]=ms },
            Koma::R1 => { self.ms_r[Go  as usize]=ms },
            _ => {},
        }
    }
    /**
     * 持ち駒の枚数を加算
     */
    pub fn add_mg(&mut self, mg:Koma, maisu:i8){
        self.mg[km_to_num(&mg)] += maisu;
    }
    pub fn get_mg(&self, mg:&Koma) -> i8 {
        self.mg[km_to_num(mg)]
    }


    /**
     * 指定の升に駒があれば真
     */
    pub fn exists_km( &self, ms:umasu)->bool{
        !match_km( &self.get_km_by_ms(ms), &Koma::Kara)
    }    

    /**
     * 指定の升に指定の駒があれば真
     */
    pub fn has_ms_km( &self, ms:umasu, km:&Koma)->bool{
        match_km( &self.get_km_by_ms(ms), km)
    }    

    /**
     * 指定の升にある駒の先後、または空升
     */
    pub fn get_sn_by_ms( &self, ms:umasu)->Sengo{
        km_to_sn( &self.get_km_by_ms(ms))
    }

    /**
     * 移動先と移動元を比較し、違う駒があれば、成ったと判定するぜ☆（＾～＾）
     */
    #[allow(dead_code)]
    pub fn is_natta( &self, ms_src:umasu, ms_dst:umasu )->bool{
        let km_src = &self.get_km_by_ms(ms_src);
        let kms_src = km_to_kms(&km_src);
        let km_dst = &self.get_km_by_ms(ms_dst);
        let kms_dst = km_to_kms(&km_dst);
        // 移動先の駒が成り駒で、 移動元の駒が不成駒なら、成る
        let pro_dst = kms_is_pro(&kms_dst);
        let pro_src = kms_is_pro(&kms_src);

        // 成り
        pro_dst && !pro_src
    }

    /**
     * 局面ハッシュを作り直す
     */
    pub fn create_hash(&self, ky_hash_seed: &KyHashSeed) -> u64 {

        let mut hash : u64 = 0;

        // 盤上の駒
        for i_ms in MASU_0..BAN_SIZE {
            let km = self.get_km_by_ms(i_ms as umasu);
            let num_km = km_to_num(&km);
            hash ^= ky_hash_seed.km[i_ms][num_km];
        }

        // 持ち駒ハッシュ
        for i_km in 0..KM_ARRAY_LN {
            let km = KM_ARRAY[i_km];
            let num_km = km_to_num(&km);

            let maisu = self.get_mg(&km);
            debug_assert!( -1<maisu && maisu <= MG_MAX as i8,
                "持ち駒 {} の枚数 {} <= {}", km, maisu, MG_MAX
            );

            hash ^= ky_hash_seed.mg[num_km][maisu as usize];
        }

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }
}

