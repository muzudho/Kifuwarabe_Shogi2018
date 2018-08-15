use CUR_POSITION_EX_WRAP;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;

pub fn default_leaf_callback() -> (Movement, i16) {
    // 現局面の駒割りを評価値とする。

    // 評価値は駒割り。
    let komawari;
    {
        let position_ex = CUR_POSITION_EX_WRAP.try_read().unwrap();
        komawari = position_ex.komawari;
    }

    // まだ次の手ではないので、点数は逆さにしておくと、さかさまにし直すように動く。
    (Movement::new(), -komawari)
}

/// 駒割り。
fn get_koma_score(km: &KmSyurui) -> i16 {
    use kifuwarabe_position::KmSyurui;
    match *km {
        KmSyurui::R => {15000},
        KmSyurui::Z => {  800},
        KmSyurui::K => { 1100},
        KmSyurui::I => {  600},
        KmSyurui::N => {  500},
        KmSyurui::U => {  300},
        KmSyurui::S => {  200},
        KmSyurui::H => {  100},
        KmSyurui::PZ => {  800},
        KmSyurui::PK => { 1100},
        KmSyurui::PN => {  500},
        KmSyurui::PU => {  300},
        KmSyurui::PS => {  200},
        KmSyurui::PH => {  100},
        _ => { 0},
    }
}

pub fn default_makemove_callback(cap: &KmSyurui) {
    // 駒割り
    let mut position_ex = CUR_POSITION_EX_WRAP.try_write().unwrap();
    position_ex.komawari += get_koma_score(&cap);
}

pub fn default_unmakemove_callback(cap: &KmSyurui) {
    // 駒割り
    let mut position_ex = CUR_POSITION_EX_WRAP.try_write().unwrap();
    position_ex.komawari -= get_koma_score(&cap);
}
