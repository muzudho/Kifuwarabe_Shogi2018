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
use std::time::{Instant};
use UCHU_WRAP;





/// 現局面での最善手を返すぜ☆（*＾～＾*）
pub fn think() -> Movement{

    // 任意の構造体を作成する。
    let mut searcher = Searcher::new();

    // 時間計測。
    searcher.stopwatch = Instant::now();
    searcher.info_stopwatch = Instant::now();

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

    // 任意の構造体を受け取る、コールバック カタログを作成する。
    let mut callback_catalog = CallbackCatalog {
        visit_leaf_callback: visit_leaf_callback,
        makemove_callback: makemove_callback,
        unmakemove_callback: unmakemove_callback,
        pick_movements_callback: pick_movements_callback,
        compare_best_callback: compare_best_callback,
    };


    // 探索を開始する。
    // どの深さまで潜るか。
    let mut max_depth = 3;
    {
        let eng = ENGINE_SETTINGS_WRAP.try_write().unwrap();
        if eng.contains(&"depth".to_string()) {
            max_depth = eng.get(&"depth".to_string()).parse::<i16>().unwrap();
        }
    }
    g_writeln(&format!("info string max_depth:{}.", max_depth));

    // 反復深化探索 iteration deeping.
    let mut best_movement_hash = TORYO_HASH;
    for id_depth in 1..max_depth+1 {
        // 指し手を選ぶ。
        let (id_best_movement_hash, _best_evaluation) = search(&mut searcher, &mut callback_catalog, id_depth, id_depth,
        -<i16>::max_value(), // min_value (負値) を - にすると正数があふれてしまうので、正の最大数に - を付ける。
        <i16>::max_value());

        // 反復深化探索の打ち切り。
        let end = searcher.stopwatch.elapsed(); // 計測時間。
        if 30 < end.as_secs() {
            // TODO: 30秒以上考えていたら、すべての探索打切り。
            break;
        }

        // 更新
        best_movement_hash = id_best_movement_hash;
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
    let end = searcher.stopwatch.elapsed();
    g_writeln(&format!("info string {}.{:03}sec.", end.as_secs(), end.subsec_nanos() / 1_000_000));

    // 返却
    Movement::from_hash(best_movement_hash)
}
