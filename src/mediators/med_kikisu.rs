/**
 * 利き数
 */

use consoles::asserts::*;
use kifuwarabe_position::*;
use memory::number_board::*;
use movement_thinks::*;
use std::collections::HashSet;

/**
 * 盤上の利き升調べ
 *
 * 用途：自殺手防止他
 *
 * TODO: 差分更新にしたい。
 */
pub fn refresh_kikisu(gen_ky: &Position) -> (
    [NumberBoard; Sengo::Num as usize],
    [NumberBoard; Koma::Num as usize]
){
    // 利き数（先後別）
    let mut local_kiki_su_by_sn : [NumberBoard; Sengo::Num as usize] = [
        NumberBoard::new(), NumberBoard::new(),
    ];

    // 利きの数（先後付き駒別）
    // 利き数（駒別なので３０個ある）
    let mut local_kiki_su_by_km : [NumberBoard; Koma::Num as usize] = [
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
    ];

    // カウント    
    for km_dst in &KM_ARRAY {
        for x in SUJI_1..SUJI_10 {// 9..0 みたいに降順に書いても動かない？
            for y in DAN_1..DAN_10 {
                let ms_dst = suji_dan_to_ms( x, y );
                assert_banjo_ms(ms_dst,"think 利き調べ");

                // 移動元の升
                let mut mv_src_hashset : HashSet<umasu>     = HashSet::new();

                insert_narazu_src_by_ms_km  (&gen_ky, ms_dst, &km_dst, &mut mv_src_hashset);

                insert_narumae_src_by_ms_km (&gen_ky, ms_dst, &km_dst, &mut mv_src_hashset);

                // 打は考えない。盤上の利き数なので
                let kikisu = mv_src_hashset.len();

                let sn = km_to_sn( &km_dst);

                // 駒別
                local_kiki_su_by_km[*km_dst as usize].add_su_by_ms( ms_dst, kikisu as i8 );

                // 先後別
                local_kiki_su_by_sn[sn as usize].add_su_by_ms( ms_dst, kikisu as i8 );
            }
        }
    }

    return (local_kiki_su_by_sn, local_kiki_su_by_km);
}