#![allow(dead_code)]
/**
 * いろんな値、定義☆（＾～＾）
 */
use std::collections::HashSet;
use std::fmt;
use teigi::conv::*;

/********
 * 手目 *
 ********/
/**
 * 手目数。何手目まで指せるか。
 * 棋譜を残す配列のサイズでもある。
 * 大会ルールが 256手として、終端子として投了を１個入れておけるようにする。
 */
pub const TEME_LN :usize = 257;
/**
 * 同一局面何回で千日手
 */
pub const SENNTITE_NUM :i8 = 4;





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
        use teigi::shogi_syugo::KmSyurui::*;
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

// 非成 駒種類数
pub const KMS_NPRO_ARRAY_LN : usize = 8;
// 非成 駒種類
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

// 成 駒種類数
pub const KMS_PRO_ARRAY_LN : usize = 6;
// 成 駒種類
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

/**
 * 駒種類集合
 */
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


/*********
 * 8方向 *
 *********/
// 盤の方向は、後手から見た視点
pub enum Dir8{
    // 東
    E,
    // 北東
    NE,
    // 北
    N,
    // 北西
    NW,
    // 西
    W,
    // 南西
    SW,
    // 南
    S,
    // 南東
    SE,
    // 要素数より1小さい数。エラー値用に使っても可
    Owari
}

/****************
 * 駒の動く方向 *
 ****************/
/**
 * 後手から見た盤を想像すること。筋、段を第一象限と同じ向きに合わせる。
 * 駒が戻る方向10方向。東から反時計回り。boolは長い利きなら真
 */
#[derive(Clone)]
pub enum KmDir{
    // 東
    E(bool),
    // 北東
    NE(bool),
    // 北北東（桂馬が戻る動き）
    NNE,
    // 北
    N(bool),
    // 北北西（桂馬が戻る動き）
    NNW,
    // 北西
    NW(bool),
    // 西
    W(bool),
    // 南西
    SW(bool),
    // 南南西（桂馬の動き）
    SSW,
    // 南
    S(bool),
    // 南南東（桂馬の動き）
    SSE,
    // 南東
    SE(bool),
    // 要素数より1小さい数。エラー値用に使っても可
    Owari
}
impl fmt::Display for KmDir{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use teigi::shogi_syugo::KmDir::*;
        match *self{
            E(b)        => if b { write!(f,"長東")  }else{ write!(f,"東")  },
            NE(b)       => if b { write!(f,"長北東")}else{ write!(f,"北東")},
            NNE         => {write!(f,"北北東")},
            N(b)        => if b { write!(f,"長北")  }else{ write!(f,"北")  },
            NNW         => { write!(f,"北北西")},
            NW(b)       => if b { write!(f,"長北西")}else{ write!(f,"北西")},
            W(b)        => if b { write!(f,"長西")  }else{ write!(f,"西")  },
            SW(b)       => if b { write!(f,"長南西")}else{ write!(f,"南西")},
            SSW         => { write!(f,"南南西")},
            S(b)        => if b { write!(f,"長南")  }else{ write!(f,"南")  },
            SSE         => { write!(f,"南南東")},
            SE(b)       => if b { write!(f,"長南東")}else{ write!(f,"南東")},
            Owari       => { write!(f,"×")},
        }
    }
}
/************
 * 駒の動き *
 ************/
// 駒が戻る動き
#[allow(dead_code)]
pub struct KmUgoki{
    // 駒種類ごとに、駒の動きを保持。動ける方向は、駒ごとに可変長配列
    pub back:[[KmDir;KM_UGOKI_LN];KMS_LN]
}
/**
 * 駒が戻る動き。投了図から現局面へ逆向きに指す思想。
 * [駒種類][9]
 *
 * （１）この表は、後手から盤面を見たものを想像する。
 * （２）後手から見て、普通に駒の動きが　登録されている。
 *       先手から見たとき、back （後ろ向きの動き）となる。
 */
pub const KM_UGOKI : KmUgoki = KmUgoki{
    back:[
        // 東,北東,北,北西,西,南西,南南西,南,南南東,南東,終わり
        /*ら  */ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),KmDir::SW(false),KmDir::S(false),KmDir::SE(false),KmDir::Owari],
        /*き  */ [KmDir::E(true ),                            KmDir::N(true ),                            KmDir::W(true ),                 KmDir::S(true ),                 KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぞ  */ [                KmDir::NE(true ),                                      KmDir::NW(true ),                KmDir::SW(true ),                KmDir::SE(true ),KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*い  */ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ね  */ [                KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),                KmDir::SW(false),                KmDir::SE(false),KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*う  */ [                                 KmDir::NNE,                KmDir::NNW                 ,                                                                  KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*し  */ [                                            KmDir::N(true )                            ,                                                                  KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ひ  */ [                                            KmDir::N(false)                            ,                                                                  KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぱき*/ [KmDir::E(true ),KmDir::NE(false),           KmDir::N(true ),           KmDir::NW(false),KmDir::W(true ),KmDir::SW(false),KmDir::S(true ),KmDir::SE(false),KmDir::Owari],
        /*ぱぞ*/ [KmDir::E(false),KmDir::NE(true ),           KmDir::N(false),           KmDir::NW(true ),KmDir::W(false),KmDir::SW(true ),KmDir::S(false),KmDir::SE(true ),KmDir::Owari],
        /*ぱね*/ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぱう*/ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぱし*/ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぱひ*/ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*空升*/ [                                                                                                                                                          KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*終り*/ [                                                                                                                                                          KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
    ]
};

/********
 * 局面 *
 ********/
pub enum KyNums {
    // 現局面
    Current,
    // 初期局面
    Start,
}

/**************
 * 予想の結果 *
 **************/
pub enum DoingResult {
    // 起こった
    Done,
    // 起こらなかった
    None,
    // それ以外
    Owari,
}
