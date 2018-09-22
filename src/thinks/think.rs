/// 深い考えだぜ☆（＾～＾）
extern crate rand;

use kifuwarabe_alpha_beta_search::*;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use mediators::med_kikisu::*;
use searcher_impl::*;
use shell_impl::*;
use std::time::Instant;
use time_manager::*;
use LOGGER;
use UCHU_WRAP;

/// どの深さまで潜るか。
pub fn get_max_depth(shell_var: &mut ShellVar) -> i16 {
    // 深さ 3 ぐらいなら 0.015秒ぐらい。
    if shell_var.engine_settings.contains(&"depth".to_string()) {
        let depth_str = shell_var.engine_settings.get(&"depth".to_string());

        match depth_str.parse::<i16>() {
            Ok(depth_num) => depth_num,
            Err(err) => panic!("info string depth_fail. message: {}.", err.to_string()),
        }
    } else {
        // 指定がなければ 5。
        5
    }
}

/// 現局面での最善手を返すぜ☆（*＾～＾*）
///
/// # Arguments.
///
/// * `milliseconds` - 残り思考時間(ミリ秒)
pub fn think(shell_var: &mut ShellVar, milliseconds: i32, max_depth: i16) -> Movement {
    // 思考時間設定。
    shell_var.searcher.thought_max_milliseconds = get_thought_max_milliseconds(milliseconds);

    // 時間計測。
    shell_var.searcher.stopwatch = Instant::now();
    shell_var.searcher.info_stopwatch = Instant::now();

    // 現局面まで、状態に進める。（差分更新できない部分）
    // グローバル変数を使う。
    {
        let mut uchu = UCHU_WRAP.try_write().unwrap();

        // ゼロ・リセット
        // 駒別に用意した盤を使った、利き数。
        for km in &KM_ARRAY {
            uchu.kiki_su_by_km[*km as usize].clear();
        }

        // 先後別に用意した盤を使った、利き数。
        for sn in &SN_ARRAY {
            uchu.kiki_su_by_sn[*sn as usize].clear();
        }

        // 相手の利き升調べ（自殺手防止のため）
        let (local_kiki_su_by_sn, local_kiki_su_by_km) =
            refresh_kikisu(&shell_var.searcher.cur_position);
        // g_writeln( &format!("info string test is_s={}", kasetu::atamakin::is_s() ) );

        // 駒別
        uchu.set_kiki_su_by_km(local_kiki_su_by_km);

        // 先後別
        uchu.set_kiki_su_by_sn(local_kiki_su_by_sn);
    }

    // 任意の構造体を受け取る、コールバック カタログを作成する。
    let mut callback_catalog = CallbackCatalog::<Searcher> {
        visit_leaf_callback: userdefined_visit_leaf_callback,
        makemove_callback: userdefined_makemove,
        unmakemove_callback: userdefined_unmakemove_not_return,
        pick_movements_callback: userdefined_pick_movements_callback,
        compare_best_callback: userdefined_compare_best_callback,
    };

    // ノード数を累計していく。
    let mut display_information = DisplayInformation::new();

    // 探索を開始する。
    if !shell_var.searcher.info_off {
        LOGGER.try_write().unwrap().writeln(&format!(
            "info string thought seconds: {}/{}, max_depth:{}.",
            shell_var.searcher.thought_max_milliseconds, milliseconds, max_depth
        ));
    }

    // 反復深化探索 iteration deeping.
    let mut best_movement_hash = RESIGN_HASH;
    for id_depth in 1..max_depth + 1 {
        shell_var.searcher.id_cur_depth = id_depth;

        // 指し手を選ぶ。
        // min_value (負値) を - にすると正数があふれてしまうので、正の最大数に - を付ける。
        let (id_best_movement_hash, best_evaluation) = search(
            &mut shell_var.searcher,
            &mut callback_catalog,
            id_depth,
            id_depth,
            -<i16>::max_value(),
            <i16>::max_value(),
            &mut display_information,
        );

        shell_var.searcher.id_evaluation = best_evaluation;

        // 反復深化探索の打ち切り。
        let end = shell_var.searcher.stopwatch.elapsed(); // 計測時間。
        if is_thought_timeout(&shell_var.searcher, end) {
            // 指定時間以上考えていたら、すべての探索打切り。
            break;
        }

        // 更新
        best_movement_hash = id_best_movement_hash;
    }

    if !shell_var.searcher.info_off {
        // 手を決めたときにも情報表示。
        LOGGER.try_write().unwrap().writeln(&format!(
            "info score cp {}",
            shell_var.searcher.id_evaluation
        ));
        // VERBOSE
        LOGGER.try_write().unwrap().writeln(&format!(
            "info string score: {}, nodes: {}, bestmove: {},  incremental_komawari: {}",
            shell_var.searcher.id_evaluation,
            display_information.nodes,
            Movement::from_hash(best_movement_hash),
            shell_var.searcher.incremental_komawari
        ));

        // 計測時間。
        let end = shell_var.searcher.stopwatch.elapsed();
        LOGGER.try_write().unwrap().writeln(&format!(
            "info string {}.{:03}sec.",
            end.as_secs(),
            end.subsec_millis()
        )); // end.subsec_nanos() / 1_000_000
    }

    // 返却
    Movement::from_hash(best_movement_hash)
}
