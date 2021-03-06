#![allow(dead_code)]
/**
 * ランダム移動カード
 */

use consoles::asserts::*;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use movement_thinks::*;
use searcher_impl::*;
use std::collections::HashSet;
use thinks::randommove;
use thinks::results::jisatusyu_result::*;

/**
 * ランダム移動
 *
 * km_dst : 移動した先の駒
 */
pub fn get_ido_ss_by_km_random(searcher: &Searcher, km_dst:Koma) -> Movement {
    
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    for _i_retry in 0..1_000_000 {
        // 移動したい先の升
        let ms_dst = randommove::rnd_ms();
        assert_banjo_ms(ms_dst, "get_ido_ss_by_km_random");

        ss_hashset.clear();
        insert_ss_by_ms_km_on_banjo (&searcher, ms_dst, km_dst, &mut ss_hashset);
        insert_ss_by_ms_km_on_da    (&searcher, ms_dst, km_dst, &mut ss_hashset);
        let ss = choice_1ss_by_hashset(&ss_hashset);

        if ss.exists(){ return ss;}
    }
    // 投了
    Movement::default()
}

/**
 * 指し手１つをランダム選出
 */
pub fn get_ss_by_random(searcher: &Searcher)->Movement{
    
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    'random: for _i_retry in 0..1_000_000 {
        // 移動したい先の升
        let ms_dst = randommove::rnd_ms();
        assert_banjo_ms(ms_dst, "Ｇet_ss_by_random");

        // 手番の、移動した先の駒
        let sn1;
        {
            sn1 = searcher.game_record.get_teban(Jiai::Ji);
        }
        let km_dst = sn_kms_to_km( sn1, *randommove::rnd_kms() );

        ss_hashset.clear();
        insert_ss_by_ms_km_on_banjo (&searcher, ms_dst, km_dst, &mut ss_hashset);
        insert_ss_by_ms_km_on_da    (&searcher, ms_dst, km_dst, &mut ss_hashset);
        let ss = choice_1ss_by_hashset( &ss_hashset );

        // 移動後は、玉が利きに飛び込まないか？
        if is_jisatusyu(&searcher, &ss) {
            continue 'random;
        }

        if ss.exists(){ return ss;}
    }
    // 投了
    Movement::default()
}
