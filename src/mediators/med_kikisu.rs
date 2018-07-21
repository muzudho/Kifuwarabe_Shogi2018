/**
 * 利き数
 */

use consoles::asserts::*;
use memory::number_board::*;
use memory::ky::*;
use memory::uchu::*;
use syazo::sasite_element::*;
use std::collections::HashSet;
use teigi::conv::*;
use teigi::shogi_syugo::*;

use UCHU_WRAP;

/**
 * 盤上の利き升調べ
 *
 * 用途：自殺手防止他
 *
 * TODO: 差分更新にしたい。
 */
pub fn refresh_kikisu(gen_ky: &Kyokumen) -> (
    [NumberBoard; SN_LN],
    [NumberBoard; KM_LN]
){
    g_writeln( &format!("r efresh_kikisu(0.1)") );

    // 利き数（先後別）
    let mut local_kiki_su_by_sn : [NumberBoard; SN_LN] = [
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
    ];

    // 利きの数（先後付き駒別）
    // 利き数（駒別なので３０個ある）
    let mut local_kiki_su_by_km : [NumberBoard; KM_LN] = [
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
        NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
    ];

    // カウント    
    for km_dst in KM_ARRAY.iter()
    {
        for x in SUJI_1..SUJI_10 {// 9..0 みたいに降順に書いても動かない？
            for y in DAN_1..DAN_10 {
                let ms_dst = suji_dan_to_ms( x, y );
                assert_banjo_ms(ms_dst,"think 利き調べ");

                // 移動元の升
                let mut mv_src_hashset : HashSet<umasu>     = HashSet::new();

                g_writeln( &format!("r efresh_kikisu(1)") );
                insert_narazu_src_by_ms_km  (&gen_ky, ms_dst, &km_dst, &mut mv_src_hashset);

                g_writeln( &format!("r efresh_kikisu(2)") );
                insert_narumae_src_by_ms_km (&gen_ky, ms_dst, &km_dst, &mut mv_src_hashset);
                g_writeln( &format!("r efresh_kikisu(3)") );

                // 打は考えない。盤上の利き数なので
                let kikisu = mv_src_hashset.len();

                g_writeln( &format!("r efresh_kikisu(4)") );

                let sn = km_to_sn( &km_dst);

                g_writeln( &format!("r efresh_kikisu(5)") );

                // 駒別
                local_kiki_su_by_km[km_to_num(&km_dst)].add_su_by_ms( ms_dst, kikisu as i8 );

                g_writeln( &format!("r efresh_kikisu(6)") );

                // 先後別
                local_kiki_su_by_sn[sn_to_num(&sn)].add_su_by_ms( ms_dst, kikisu as i8 );
                g_writeln( &format!("r efresh_kikisu(7)") );
            }
            g_writeln( &format!("r efresh_kikisu(8)") );
        }
        g_writeln( &format!("r efresh_kikisu(9)") );
    }
    g_writeln( &format!("r efresh_kikisu(10)") );

    return (local_kiki_su_by_sn, local_kiki_su_by_km);
}