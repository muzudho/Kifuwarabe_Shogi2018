/**
 * 盤上いろいろ☆（＾～＾）
 */

use CUR_POSITION_WRAP;
use kifuwarabe_position::*;
use UCHU_WRAP;

pub fn is_ji_km_by_ms(ms:umasu) -> bool {
    let km = CUR_POSITION_WRAP.try_read().unwrap().get_km_by_ms( ms );
    let (sn,_kms) = km_to_sn_kms( &km );
    match_sn( &sn, &UCHU_WRAP.try_read().unwrap().get_teban(&Jiai::Ji) )
}

// TODO
pub fn is_ai_kiki_by_ms(_ms:umasu) -> bool {
    false
}
