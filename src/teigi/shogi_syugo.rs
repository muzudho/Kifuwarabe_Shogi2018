#![allow(dead_code)]
/**
 * いろんな値、定義☆（＾～＾）
 */
use kifuwarabe_position::*;
use std::fmt;








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
