use CUR_POSITION_EX_WRAP;
use kifuwarabe_alpha_beta_search::*;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use SEARCHER_VAR_WRAP;
use std::collections::HashSet;
use std::time::{Instant};
use syazo::sasite_seisei::*;
use syazo::sasite_sentaku::*;

use memory::uchu::*;
use UCHU_WRAP;
use teigi::shogi_syugo::*;


pub struct SearcherVariable {
    pub stopwatch: Instant,
}
impl SearcherVariable {
    pub fn new() -> SearcherVariable {
        SearcherVariable {
            stopwatch: Instant::now()
        }
    }
}

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

/// 指し手生成。
pub fn default_pick_movements_callback(max_depth: i16, cur_depth: i16) -> (HashSet<u64>, bool) {

    let mut hashset_movement : HashSet<u64> = HashSet::new();
    // 反復深化探索の打ち切り。
    let end; // 計測時間。
    {
        end = SEARCHER_VAR_WRAP.try_read().unwrap().stopwatch.elapsed();
    }
    if 30 < end.as_secs() {
        // TODO: 30秒以上考えていたら探索打切り。
        return (hashset_movement, true);
    }


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

    (hashset_movement, false)
}

pub fn default_makemove_callback(cap: &KmSyurui) {
    // 駒割り
    let mut position_ex = CUR_POSITION_EX_WRAP.try_write().unwrap();
    position_ex.komawari += get_koma_score(&cap);

/*
    // 現局面表示
    {
        let uchu_r = UCHU_WRAP.try_read().unwrap();
        g_writeln(&uchu_r.kaku_ky(&KyNums::Current, false));
    }
*/
}

pub fn default_unmakemove_callback(cap: &KmSyurui) {
    // 駒割り
    let mut position_ex = CUR_POSITION_EX_WRAP.try_write().unwrap();
    position_ex.komawari -= get_koma_score(&cap);

/*
    // 現局面表示
    {
        let uchu_r = UCHU_WRAP.try_read().unwrap();
        g_writeln(&uchu_r.kaku_ky(&KyNums::Current, false));
    }
*/
}

/// 指し手の比較。
///
/// # Arguments.
///
/// * `_best_movement` - ベストな指し手。
/// * `_alpha` - alpha。より良い手があれば増える。
/// * `_beta` - beta。
/// * `_movement` - 今回比較する指し手。
/// * `_child_evaluation` - 今回比較する指し手の評価値。
///
/// # Returns.
///
/// 探索を打ち切るなら真。
pub fn default_compare_best_callback(best_movement: &mut Movement, alpha: &mut i16, beta: i16, movement: Movement, child_evaluation: i16) -> (bool, bool) {

    // 比較して、一番良い手を選ぶ。（アップデート アルファ）
    if *alpha < child_evaluation {
        *alpha = child_evaluation;
        *best_movement = movement; // この手。
    }

    if beta < *alpha {
        // TODO 探索を打ち切って欲しい。評価値は alpha で。
        return (true, false);
    }

    // 反復深化探索の打ち切り。
    let end; // 計測時間。
    {
        end = SEARCHER_VAR_WRAP.try_read().unwrap().stopwatch.elapsed();
    }
    if 30 < end.as_secs() {
        // TODO: 30秒以上考えていたら、すべての探索打切り。
        return (false, true);
    }

    (false, false)
}
