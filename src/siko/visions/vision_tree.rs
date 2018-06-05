/**
 * 思考部だぜ☆（＾～＾）
 */
use teigi::conv::*;
use syazo::sasite_element::*;
use std::collections::HashSet;
use teigi::shogi_syugo::*;
use tusin::usi::*;
use jotai::uchu::*;

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
pub fn insert_rakkansuji( uchu:&mut Uchu ){
    for sn in SN_ARRAY.iter() {
        let ai_sn = hanten_sn( sn );

        // 相手の　らいおん　の位置を覚える
        &uchu.vision_tree_by_sn[sn_to_num(sn)].set_ai_r( uchu.ky.ms_r[sn_to_num(&ai_sn)] );
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
                    insert_narazu_src_by_ms_km  ( ms_dst, &km_dst, &uchu, &mut mv_src_hashset );
                    insert_narumae_src_by_ms_km ( ms_dst, &km_dst, &uchu, &mut mv_src_hashset );
                    // TODO 王手になるところに打ちたい
                    //insert_da_kms_by_ms_km      ( &ms_dst, &km_dst, &uchu, &mut da_kms_hashset );

                    // 盤上
                    for ms_src in mv_src_hashset.iter() {
                        // 成り
                        let pro = &uchu.ky.is_natta( *ms_src, ms_dst );
                        
                        let hash_ss = Sasite{
                            src:*ms_src,
                            dst:ms_dst,
                            pro:*pro,
                            drop:KmSyurui::Kara,
                        }.to_hash();
                        &uchu.vision_tree_by_sn[sn_to_num(sn)].ss_tume_hashset.insert( hash_ss );
                    }

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
                        &uchu.vision_tree_by_sn[sn].ss_tume_hashset.insert( hash_ss );
                    }
                    */
                }
            }
        }
    }//sn
}