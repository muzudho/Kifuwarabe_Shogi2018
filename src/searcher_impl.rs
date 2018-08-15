use CUR_POSITION_EX_WRAP;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use std::collections::HashSet;
use syazo::sasite_seisei::*;
use syazo::sasite_sentaku::*;

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

pub fn default_pick_movements_callback(max_depth: i16, cur_depth: i16) -> HashSet<u64> {
    let mut hashset_movement : HashSet<u64> = HashSet::new();
    // 駒の動き方
    insert_potential_move(&mut hashset_movement);
    // g_writeln("テスト ポテンシャルムーブ.");
    // use consoles::visuals::dumps::*;
    // hyoji_ss_hashset( &hashset_movement );

    if max_depth == cur_depth {
        // 王手されている場合、王手回避の手に絞り込む。
        filtering_ss_except_oute(&mut hashset_movement);

        // FIXME 負けてても、千日手は除くぜ☆（＾～＾）ただし、千日手を取り除くと手がなくなる場合は取り除かないぜ☆（＾～＾）
        filtering_ss_except_sennitite(&mut hashset_movement);

        // 自殺手は省くぜ☆（＾～＾）
        filtering_ss_except_jisatusyu( &mut hashset_movement);
    };

    hashset_movement
}

pub fn default_compare_best_callback(best_movement: &mut Movement, best_evaluation: &mut i16, movement: Movement, child_evaluation: i16) {
    // 比較して、一番良い手を選ぶ。
    if *best_evaluation < child_evaluation {
        *best_evaluation = child_evaluation;
        *best_movement = movement; // この手。
    }
}
