/**
 * 盤上いろいろ☆（＾～＾）
 */

use kifuwarabe_position::*;
use searcher_impl::*;

pub fn is_ji_km_by_ms(searcher: &Searcher, ms:umasu) -> bool {
    let km = searcher.cur_position.get_km_by_ms( ms );
    let (sn,_kms) = km_to_sn_kms( km );
    sn == searcher.game_record.get_teban(Jiai::Ji)
}

// TODO
pub fn is_ai_kiki_by_ms(_ms:umasu) -> bool {
    false
}
