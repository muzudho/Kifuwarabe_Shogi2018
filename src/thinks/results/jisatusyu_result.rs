#![allow(dead_code)]
/**
 * 結果：自殺手。移動先が敵の利き
 */

use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use searcher_impl::*;
use UCHU_WRAP;

/// 動かした先が、敵の利きに飛び込んでいれば、自殺手
/// TODO 利きを再計算したい
pub fn is_jisatusyu(searcher: &Searcher, ss:&Movement)->bool{
    // 移動元升、動かした駒の先後、駒種類、
    let km_src = searcher.cur_position.get_km_by_ms( ss.source );
    let (sn_teban,_kms) = km_to_sn_kms( km_src );
    // 相手番の先後
    let sn_aite = hanten_sn( sn_teban );

    // 升の利き数だが、指した後で再計算が要るはず
    let kikisu = UCHU_WRAP.try_read().unwrap().kiki_su_by_sn[sn_aite as usize].get_su_by_ms( ss.destination );
    0<kikisu
    // g_writeln(&format!(
    //     "info string is_jisatusyu={} km_src={} sn_teban={} kms={} sn_aite={} ss.destination={} kikisu={}"
    //     ,result ,km_src ,sn_teban ,kms ,sn_aite ,ss.destination ,kikisu
    // ));
}
