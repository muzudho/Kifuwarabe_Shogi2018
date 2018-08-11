/// 深い考えだぜ☆（＾～＾）
extern crate rand;

use CUR_POSITION_WRAP;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use mediators::med_kikisu::*;
use searchs::searcher::*;
use UCHU_WRAP;

/// 現局面での最善手を返すぜ☆（*＾～＾*）
pub fn think()->Movement{

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
        // g_writeln( &format!("info test is_s={}", kasetu::atamakin::is_s() ) );

        // 駒別
        uchu.set_kiki_su_by_km( local_kiki_su_by_km);

        // 先後別
        uchu.set_kiki_su_by_sn( local_kiki_su_by_sn);
    }


    // 探索を開始する。
    let mut searcher = Searcher::new();

    // 指し手を返す。
    searcher.search()
}
