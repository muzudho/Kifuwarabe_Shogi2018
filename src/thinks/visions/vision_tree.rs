/**
 * 思考部だぜ☆（＾～＾）
 */
use syazo::sasite_element::*;
use std::collections::HashSet;
use teigi::conv::*;
use teigi::shogi_syugo::*;
use tusin::usi::*;
use memory::ky::*;
use memory::uchu::*;

use UCHU_WRAP;

/**
 * 狙いは、この木にぶら下げていくぜ☆（*＾～＾*）
 * 思考で、内容はどんどん変わっていくぜ☆（＾～＾）
 */
pub struct VisionTree {
    // 相手玉の位置
    pub ms_ai_r : umasu,
    // 相手玉を取る楽観筋
    pub ss_tume_hashset : HashSet<u64>,
}
impl VisionTree{
    pub fn new()->VisionTree{
        VisionTree{
            ms_ai_r         : 0,
            ss_tume_hashset : HashSet::new(),
        }
    }
    pub fn clear(&mut self){
        self.ss_tume_hashset.clear();
    }
    pub fn set_ai_r(&mut self, ms:umasu){
        self.ms_ai_r = ms;
    }
}

/**
 * 楽観筋
 */
pub fn insert_rakkansuji(
    gen_ky: &Kyokumen,
    sn: &Sengo,
    vtree: &mut VisionTree,
    ai_ms_r: umasu
){

    g_writeln( &format!("insert_rakkansuji(1..3)") );

    vtree.set_ai_r( ai_ms_r );

    g_writeln( &format!("insert_rakkansuji(4)") );

    // 盤上に相手の　らいおん１枚　しかないと想定して、アタックする手
    let mut mv_src_hashset : HashSet<umasu> = HashSet::new();
    //let mut da_kms_hashset : HashSet<usize> = HashSet::new();

    g_writeln( &format!("insert_rakkansuji(5)") );

    for kms_dst in KMS_ARRAY.iter() {

        g_writeln( &format!("insert_rakkansuji(6)") );

        let km_dst = sn_kms_to_km( &sn, &kms_dst );

        g_writeln( &format!("insert_rakkansuji(7)") );

        for x in SUJI_1..SUJI_10 {// 9..0 みたいに降順に書いても動かない？

            g_writeln( &format!("insert_rakkansuji(8)") );

            for y in DAN_1..DAN_10 {

                g_writeln( &format!("insert_rakkansuji(9)") );

                let ms_dst = suji_dan_to_ms( x, y );

                g_writeln( &format!("insert_rakkansuji(10)") );

                mv_src_hashset.clear();

                g_writeln( &format!("insert_rakkansuji(11)") );

                //da_kms_hashset.clear();
                // 現局面を読取専用で取得し、ロック。
                let gen_ky = &UCHU_WRAP.read().unwrap().ky;
                insert_narazu_src_by_ms_km  (&gen_ky, ms_dst, &km_dst, &mut mv_src_hashset );

                g_writeln( &format!("insert_rakkansuji(12)") );

                insert_narumae_src_by_ms_km (&gen_ky, ms_dst, &km_dst, &mut mv_src_hashset );
                // TODO 王手になるところに打ちたい
                //insert_da_kms_by_ms_km      ( &ms_dst, &km_dst, &mut da_kms_hashset );

                g_writeln( &format!("insert_rakkansuji(13)") );

                // 盤上
                for ms_src in mv_src_hashset.iter() {

                    g_writeln( &format!("insert_rakkansuji(14)") );

                    // 成り
                    let pro = &UCHU_WRAP.read().unwrap().ky.is_natta( *ms_src, ms_dst );

                    g_writeln( &format!("insert_rakkansuji(15)") );

                    let hash_ss = Sasite{
                        src:*ms_src,
                        dst:ms_dst,
                        pro:*pro,
                        drop:KmSyurui::Kara,
                    }.to_hash();

                    g_writeln( &format!("insert_rakkansuji(16)") );

                    vtree.ss_tume_hashset.insert( hash_ss );

                    g_writeln( &format!("insert_rakkansuji(17)") );
                }

                g_writeln( &format!("insert_rakkansuji(18)") );

                /*
                // 打
                for kms_da in da_kms_hashset.iter() {
                    let km_da = sn_kms_to_km( &sn, &kms_da );
                    
                    let hash_ss = Sasite{
                        src:SS_SRC_DA,
                        dst:ms_dst,
                        pro:false,
                        drop:km_da,
                    }.to_hash();
                    &UCHU_WRAP.write().unwrap().vision_tree_by_sn[sn].ss_tume_hashset.insert( hash_ss );
                }
                */
            }
            g_writeln( &format!("insert_rakkansuji(19)") );
        }
        g_writeln( &format!("insert_rakkansuji(20)") );
    }
    g_writeln( &format!("insert_rakkansuji(21)") );
}