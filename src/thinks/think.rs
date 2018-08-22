/// 深い考えだぜ☆（＾～＾）
extern crate rand;

use CUR_POSITION_WRAP;
use CUR_POSITION_EX_WRAP;
use ENGINE_SETTINGS_WRAP;
use kifuwarabe_alpha_beta_search::*;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use mediators::med_kikisu::*;
use memory::uchu::*;
use searcher_impl::*;
use SEARCHER_VAR_WRAP;
use std::time::{Duration, Instant};
use UCHU_WRAP;




/// 現局面での最善手を返すぜ☆（*＾～＾*）
pub fn think() -> Movement{

    // 時間計測。
    {
        SEARCHER_VAR_WRAP.try_write().unwrap().stopwatch = Instant::now();
    }

    // 現局面まで、状態に進める。（差分更新できない部分）
    // グローバル変数を使う。
    {
        let mut uchu = UCHU_WRAP.try_write().unwrap();

        // ゼロ・リセット
        // 駒別に用意した盤を使った、利き数。
        for km in KM_ARRAY.iter() {
            uchu.kiki_su_by_km[km_to_num(km)].clear();
        }

        // 先後別に用意した盤を使った、利き数。
        for sn in SN_ARRAY.iter() {
            uchu.kiki_su_by_sn[sn_to_num(sn)].clear();
        }

        // 相手の利き升調べ（自殺手防止のため）
        let (local_kiki_su_by_sn, local_kiki_su_by_km) = refresh_kikisu(
            // 現局面を読取専用で取得し、ロック。
            &CUR_POSITION_WRAP.try_read().unwrap()
            );
        // g_writeln( &format!("info string test is_s={}", kasetu::atamakin::is_s() ) );

        // 駒別
        uchu.set_kiki_su_by_km( local_kiki_su_by_km);

        // 先後別
        uchu.set_kiki_su_by_sn( local_kiki_su_by_sn);
    }


    let mut alphaBetaSearcher = AlphaBetaSearcher::new();
    alphaBetaSearcher.leaf_callback = default_leaf_callback;
    alphaBetaSearcher.makemove_callback = default_makemove_callback;
    alphaBetaSearcher.unmakemove_callback = default_unmakemove_callback;
    alphaBetaSearcher.pick_movements_callback = default_pick_movements_callback;
    alphaBetaSearcher.compare_best_callback = default_compare_best_callback;


    // 探索を開始する。
    // どの深さまで潜るか。
    let mut max_depth = 3;
    {
        let eng = ENGINE_SETTINGS_WRAP.try_write().unwrap();
        if eng.contains(&"max_depth".to_string()) {
            max_depth = eng.get(&"max_depth".to_string()).parse::<i16>().unwrap();
        }
    }
    g_writeln(&format!("info string max_depth:{}.", max_depth));

    // TODO: 反復深化探索
    let mut best_movement = Movement::new();
    for id_depth in 1..max_depth+1 {
        // 指し手を選ぶ。
        let (id_best_movement, _best_evaluation) = alphaBetaSearcher.search(id_depth, id_depth,
        -<i16>::max_value(), // min_value (負値) を - にすると正数があふれてしまうので、正の最大数に - を付ける。
        <i16>::max_value());

        // 反復深化探索の打ち切り。
        let end; // 計測時間。
        {
            end = SEARCHER_VAR_WRAP.try_read().unwrap().stopwatch.elapsed();
        }
        if 30 < end.as_secs() {
            // TODO: 30秒以上考えていたら、すべての探索打切り。
            break;
        }

        // 更新
        best_movement = id_best_movement;
    }


    // これはテスト
    {
        let position_ex = CUR_POSITION_EX_WRAP.try_read().unwrap();
        g_writeln(&format!("info string TEST komawari calculation:{}. (Expected: 0)", position_ex.komawari ));
    }


        /*
        // 現局面を見て、ビジョンを作り直せだぜ☆（＾～＾）
        &UCHU_WRAP.try_write().unwrap().remake_visions();
        */

        /*
        // 楽観筋
        for sn in SN_ARRAY.iter() {
            let ai_sn = hanten_sn( sn );
            // 相手の　らいおん　の位置を覚える
            let ai_ms_r = CUR_POSITION_WRAP.try_read().unwrap().ms_r[sn_to_num(&ai_sn)];
            insert_rakkansuji(&sn, &mut UCHU_WRAP.try_write().unwrap().vision_tree_by_sn[sn_to_num(sn)], ai_ms_r);
        }
        // TODO 楽観筋はまだ使ってない☆（＾～＾）
        */

        // 楽観王手の一覧はできているはず。

    // 計測時間。
    let end;
    {
        end = SEARCHER_VAR_WRAP.try_write().unwrap().stopwatch.elapsed();
    }
    g_writeln(&format!("info string {}.{:03}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000));

    // 返却
    best_movement
}
