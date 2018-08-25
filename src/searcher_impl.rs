use kifuwarabe_alpha_beta_search::*;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use memory::uchu::*;
use std::collections::HashSet;
use std::time::Instant;
use syazo::sasite_seisei::*;
use syazo::sasite_sentaku::*;
use time_manager::*;

// use teigi::shogi_syugo::*;
use UCHU_WRAP;


/// 任意の構造体を作成する。
pub struct Searcher {
    pub stopwatch: Instant,
    pub info_stopwatch: Instant,
    /// 最大思考時間(秒)。
    pub thought_max_milliseconds: i32,
    // 反復深化探索(iteration deeping)で現在探索途中の深さ。
    pub id_cur_depth: i16,
    // 反復深化探索(iteration deeping)で一番有力な評価値。
    pub id_evaluation: i16,
    // 駒割の差分更新。
    pub incremental_komawari: i16,
    // 初期局面のコピー。
    pub ini_position: Position,
    // 現局面のコピー。
    pub cur_position: Position,
}
impl Searcher {
    pub fn new() -> Searcher {
        Searcher {
            stopwatch: Instant::now(),
            info_stopwatch: Instant::now(),
            thought_max_milliseconds: 0,
            id_cur_depth: 0,
            id_evaluation: 0,
            incremental_komawari: 0,
            ini_position: Position::new(),
            cur_position: Position::new(),
        }
    }
}

pub fn visit_leaf_callback(searcher: &mut Searcher, _position1: &mut Position, display_information: &DisplayInformation) -> (i16) {

    // 評価値は駒割り。
    let komawari = searcher.incremental_komawari;
    g_writeln(&format!("info string DEBUG komawari {}", komawari));

    // 読み筋表示。
    {
        let end = searcher.info_stopwatch.elapsed();
        if 3 < end.as_secs() {
            // 3秒以上考えていたら、情報表示。
            g_writeln(&format!("info depth {} seldepth 0 time 0 nodes {} score cp {} nps 0 pv", searcher.id_cur_depth, display_information.nodes, searcher.id_evaluation));
            searcher.info_stopwatch = Instant::now();
        }
    }

    // 現局面の駒割りを評価値とする。
    komawari
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
pub fn pick_movements_callback(searcher: &mut Searcher, max_depth: i16, cur_depth: i16, position1: &mut Position) -> (HashSet<u64>, bool) {

    let mut hashset_movement : HashSet<u64> = HashSet::new();
    // 反復深化探索の打ち切り。
    let end = searcher.stopwatch.elapsed(); // 計測時間。
    if is_thought_timeout(searcher, end) {
        // 指定時間以上考えていたら探索打切り。
        return (hashset_movement, true);
    }


    // 駒の動き方
    insert_potential_move(&mut hashset_movement, &position1);
    // g_writeln("テスト ポテンシャルムーブ.");
    // use consoles::visuals::dumps::*;
    // hyoji_ss_hashset( &hashset_movement );

    if max_depth == cur_depth {
        // 王手されている場合、王手回避の手に絞り込む。
        filtering_ss_except_oute(&mut hashset_movement, &position1);

        // FIXME 負けてても、千日手は除くぜ☆（＾～＾）ただし、千日手を取り除くと手がなくなる場合は取り除かないぜ☆（＾～＾）
        filtering_ss_except_sennitite(searcher, &mut hashset_movement, position1);

        // 自殺手は省くぜ☆（＾～＾）
        filtering_ss_except_jisatusyu(searcher, &mut hashset_movement, position1);
    };

    (hashset_movement, false)
}

/// １手指す。
///
/// # Arguments.
///
/// * `movement_hash` - 指し手のハッシュ値。
pub fn makemove(searcher: &mut Searcher, movement_hash: u64, position1: &mut Position) {

    let cap_kms;
    {
        let movement = Movement::from_hash(movement_hash);
        {
            cap_kms = GAME_RECORD_WRAP.try_write().unwrap().make_movement2(&movement, position1);
        }
    }

    // 駒割の差分更新。
    searcher.incremental_komawari += get_koma_score(&cap_kms);

    // 現局面表示
    {
        let uchu_r = UCHU_WRAP.try_read().unwrap();
        g_writeln(&uchu_r.kaku_ky(&position1, false));
    }
}

pub fn unmakemove(searcher: &mut Searcher, position1: &mut Position) -> (bool, KmSyurui) {

    let successful;
    let cap_kms;
    {
        let (successful2, cap_kms2) = GAME_RECORD_WRAP.try_write().unwrap().unmake_movement2(position1);
        successful = successful2;
        cap_kms = cap_kms2;
    }

    if successful {
        // 駒割り
        searcher.incremental_komawari -= get_koma_score(&cap_kms);
    }

    // 現局面表示
    {
        let uchu_r = UCHU_WRAP.try_read().unwrap();
        g_writeln(&uchu_r.kaku_ky(&position1, false));
    }

    (successful, cap_kms)
}
pub fn unmakemove_not_return(searcher: &mut Searcher, position1: &mut Position) {
    unmakemove(searcher, position1);
}

/// 指し手の比較。
///
/// # Arguments.
///
/// * `t` - 任意のオブジェクト。
/// * `best_movement_hash` - ベストな指し手のハッシュ値。
/// * `alpha` - より良い手があれば増える。
/// * `beta` - ベータ。
/// * `movement_hash` - 今回比較する指し手のハッシュ値。
/// * `evaluation` - 今回比較する指し手の評価値。
///
/// # Returns.
///
/// 1. 探索を打ち切るなら真。（ベータカット）
/// 2. 探索をすみやかに安全に終了するなら真。
pub fn compare_best_callback(searcher: &mut Searcher, best_movement_hash: &mut u64, alpha: &mut i16, beta: i16, movement_hash: u64, child_evaluation: i16, _position1: &mut Position) -> (bool, bool) {

    // 比較して、一番良い手を選ぶ。（アップデート アルファ）
    if *alpha < child_evaluation {
        *alpha = child_evaluation;
        *best_movement_hash = movement_hash; // この手。
    }

    if beta < *alpha {
        // TODO 探索を打ち切って欲しい。評価値は alpha で。
        return (true, false);
    }

    // 反復深化探索の打ち切り。
    let end = searcher.stopwatch.elapsed(); // 計測時間。
    if is_thought_timeout(searcher, end) {
        // 指定時間以上考えていたら、すべての探索打切り。
        return (false, true);
    }

    (false, false)
}
