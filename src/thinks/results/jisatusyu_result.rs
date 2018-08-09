#![allow(dead_code)]
/**
 * 結果：自殺手。移動先が敵の利き
 */

use CUR_POSITION_WRAP;
use kifuwarabe_position::*;
use models::movement::*;
use UCHU_WRAP;

/// 動かした先が、敵の利きに飛び込んでいれば、自殺手
/// TODO 利きを再計算したい
pub fn is_jisatusyu(ss:&Movement)->bool{
    // 移動元升、動かした駒の先後、駒種類、
    let km_src = CUR_POSITION_WRAP.try_read().unwrap().get_km_by_ms( ss.source );
    let (sn_teban,_kms) = km_to_sn_kms( &km_src );
    // 相手番の先後
    let sn_aite = hanten_sn( &sn_teban );

    // 升の利き数だが、指した後で再計算が要るはず
    let kikisu = UCHU_WRAP.try_read().unwrap().kiki_su_by_sn[ sn_to_num( &sn_aite) ].get_su_by_ms( ss.destination );
    let result = 0<kikisu;
    // g_writeln(&format!(
    //     "info is_jisatusyu={} km_src={} sn_teban={} kms={} sn_aite={} ss.destination={} kikisu={}"
    //     ,result ,km_src ,sn_teban ,kms ,sn_aite ,ss.destination ,kikisu
    // ));

    result
}
