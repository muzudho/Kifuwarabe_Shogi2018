/**
 * 思考部だぜ☆（＾～＾）
 */
use syazo::sasite_element::*;
use std::collections::HashSet;
use teigi::conv::*;
use teigi::shogi_syugo::*;
use tusin::usi::*;
use memory::ky::*;

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
    #[allow(dead_code)]
    pub fn set_ai_r(&mut self, ms:umasu){
        self.ms_ai_r = ms;
    }
}

/**
 * 楽観筋
 */
#[allow(dead_code)]
pub fn insert_rakkansuji(
    _gen_ky: &Kyokumen,
    sn: &Sengo,
    vtree: &mut VisionTree,
    ai_ms_r: umasu
){
    vtree.set_ai_r( ai_ms_r );

    // 盤上に相手の　らいおん１枚　しかないと想定して、アタックする手
    let mut mv_src_hashset : HashSet<umasu> = HashSet::new();
    //let mut da_kms_hashset : HashSet<usize> = HashSet::new();

    for kms_dst in KMS_ARRAY.iter() {
        let km_dst = sn_kms_to_km( &sn, &kms_dst );
        for x in SUJI_1..SUJI_10 {// 9..0 みたいに降順に書いても動かない？
            for y in DAN_1..DAN_10 {
                let ms_dst = suji_dan_to_ms( x, y );
                mv_src_hashset.clear();
                //da_kms_hashset.clear();
                // 現局面を読取専用で取得し、ロック。
                let gen_ky = &UCHU_WRAP.read().unwrap().ky;
                insert_narazu_src_by_ms_km  (&_gen_ky, ms_dst, &km_dst, &mut mv_src_hashset );
                insert_narumae_src_by_ms_km (&gen_ky, ms_dst, &km_dst, &mut mv_src_hashset );
                // TODO 王手になるところに打ちたい
                //insert_da_kms_by_ms_km      ( &ms_dst, &km_dst, &mut da_kms_hashset );
                // 盤上
                for ms_src in mv_src_hashset.iter() {
                    // 成り
                    let pro = &UCHU_WRAP.read().unwrap().ky.is_natta( *ms_src, ms_dst );
                    let hash_ss = Movement{
                        source:*ms_src,
                        destination:ms_dst,
                        promotion:*pro,
                        drop:KmSyurui::Kara,
                    }.to_hash();
                    vtree.ss_tume_hashset.insert( hash_ss );
                }

                /*
                // 打
                for kms_da in da_kms_hashset.iter() {
                    let km_da = sn_kms_to_km( &sn, &kms_da );
                    
                    let hash_ss = Movement{
                        source:SS_SRC_DA,
                        destination:ms_dst,
                        promotion:false,
                        drop:km_da,
                    }.to_hash();
                    &UCHU_WRAP.write().unwrap().vision_tree_by_sn[sn].ss_tume_hashset.insert( hash_ss );
                }
                */
            }
        }
    }
}