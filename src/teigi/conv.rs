#![allow(dead_code)]
/**
 * 変換
 */

use consoles::asserts::*;
use kifuwarabe_position::*;
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
