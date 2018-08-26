#![allow(dead_code)]
/**
 * 変換
 */

use consoles::asserts::*;
use kifuwarabe_position::*;
use movement_picker::*;
use teigi::geometries::geo_teigi::*;
use teigi::shogi_syugo::*;


/*********
 * 4角度 *
 *********/

/*********
 * 8方向 *
 *********/
#[allow(dead_code)]
pub fn dir8_to_num(dir:&Dir8) -> usize {
    use teigi::shogi_syugo::Dir8::*;
    match *dir {
        E   => 0,
        NE  => 1,
        N   => 2,
        NW  => 3,
        W   => 4,
        SW  => 5,
        S   => 6,
        SE  => 7,
        Owari => 8,
    }
}
#[allow(dead_code)]
pub fn num_to_dir8(n:usize) -> Dir8 {
    use teigi::shogi_syugo::Dir8::*;
    match n {
        0   => E,
        1   => NE,
        2   => N,
        3   => NW,
        4   => W,
        5   => SW,
        6   => S,
        7   => SE,
        _   => Owari,
    }
}
/**
 * ハッシュ値を作る
 */
#[allow(dead_code)]
pub fn push_dir8_to_hash(hash:u64, dir:&Dir8) -> u64 {
    // エラー値含めて 9bit あるので 2^5
    (hash<<5) + dir8_to_num(dir) as u64
}
/**
 * ハッシュ値から作る
 */
#[allow(dead_code)]
pub fn pop_dir8_from_hash(hash:u64) -> (u64, Dir8) {
    // エラー値含めて 9bit あるので 2^5
    let dir = num_to_dir8( (hash & 0b11111) as usize );
    (hash>>5, dir)
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
pub fn ms_to_p(ms:umasu)->Point{
    assert_banjo_ms(ms,"(203b)ms_to_p");
    Point {
        x : (ms/10) as i8,
        y : (ms%10) as i8,
    }
}
pub fn p_in_ban(p:&Point)->bool{
       (SUJI_0<p.x && p.x<SUJI_10)
    && ( DAN_0<p.y && p.y< DAN_10)    
}
pub fn p_to_ms(p:&Point)->umasu{
    debug_assert!(p_in_ban(&p), "(204b)p_to_ms x={},y={}",p.x, p.y);

    (p.x*10 + p.y) as umasu    
}







/************
 * 駒の動き *
 ************/

 /**
  * 上下反転
  */
pub fn hanten_kmdir_joge(kmdir:&KmDir)->KmDir{
    use movement_picker::KmDir::*;
    match *kmdir{
        // 東
        E(b)  => E(b),
        // 北東
        NE(b) => SE(b),
        // 北北東（桂馬が戻る動き）
        NNE   => SSE,
        // 北
        N(b)  => S(b),
        // 北北西（桂馬が戻る動き）
        NNW   => SSW,
        // 北西
        NW(b) => SW(b),
        // 西
        W(b)  => W(b),
        // 南西
        SW(b) => NW(b),
        // 南南西（桂馬の動き）
        SSW   => NNW,
        // 南
        S(b)  => N(b),
        // 南南東（桂馬の動き）
        SSE   => NNE,
        // 南東
        SE(b) => NE(b),
        // 要素数より1小さい数。エラー値用に使っても可
        Owari => Owari,
    }
}
/*
pub fn kmdir_id(kmdir:&KmDir) -> usize{
    use teigi::shogi_syugo::KmDir::*;
    match *kmdir {
        E  (b)=>if b { 0}else{ 1},
        NE (b)=>if b { 2}else{ 3},
        N  (b)=>if b { 4}else{ 5},
        NW (b)=>if b { 6}else{ 7},
        W  (b)=>if b { 8}else{ 9},
        SW (b)=>if b {10}else{11},
        SSW   =>12,
        S  (b)=>if b {13}else{14},
        SSE   =>15,
        SE (b)=>if b {16}else{17},
        Owari =>18,
    }
}
*/
