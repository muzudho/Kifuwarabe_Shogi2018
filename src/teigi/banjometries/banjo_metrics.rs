/**
 * 盤上いろいろ☆（＾～＾）
 */

use GAME_RECORD_WRAP;
use kifuwarabe_position::*;

pub fn is_ji_km_by_ms(ms:umasu, position1: &Position) -> bool {
    let km = position1.get_km_by_ms( ms );
    let (sn,_kms) = km_to_sn_kms( &km );
    match_sn( &sn, &GAME_RECORD_WRAP.try_read().unwrap().get_teban(&Jiai::Ji) )
}

// TODO
pub fn is_ai_kiki_by_ms(_ms:umasu) -> bool {
    false
}
