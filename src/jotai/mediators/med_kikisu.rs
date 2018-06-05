/**
 * 利き数
 */

use consoles::asserts::*;
use teigi::conv::*;
use teigi::shogi_syugo::*;
use syazo::sasite_element::*;
use std::collections::HashSet;
use jotai::uchu::*;

/**
 * 盤上の利き升調べ
 *
 * 用途：自殺手防止他
 */
pub fn read_kikisu(uchu:&mut Uchu){

    // ゼロ・リセット
    for km in KM_ARRAY.iter() {
        &uchu.kiki_su_by_km[km_to_num(km)].clear();
    }

    for sn in SN_ARRAY.iter() {
        &uchu.kiki_su_by_sn[sn_to_num(sn)].clear();
    }

    // カウント    
    for km_dst in KM_ARRAY.iter()
    {
        for x in SUJI_1..SUJI_10 {// 9..0 みたいに降順に書いても動かない？
            for y in DAN_1..DAN_10 {
                let ms_dst = suji_dan_to_ms( x, y );
                assert_banjo_ms(ms_dst,"think 利き調べ");

                // 移動元の升
                let mut mv_src_hashset : HashSet<umasu>     = HashSet::new();
                insert_narazu_src_by_ms_km  ( ms_dst, &km_dst, &uchu, &mut mv_src_hashset );
                insert_narumae_src_by_ms_km ( ms_dst, &km_dst, &uchu, &mut mv_src_hashset );
                // 打は考えない。盤上の利き数なので
                let kikisu = mv_src_hashset.len();
                let sn = km_to_sn( &km_dst);

                // 駒別
                uchu.kiki_su_by_km[km_to_num(&km_dst)].add_su_by_ms( ms_dst, kikisu as i8 );

                // 先後別
                uchu.kiki_su_by_sn[sn_to_num(&sn)].add_su_by_ms( ms_dst, kikisu as i8 );
            }
        }
    }

}