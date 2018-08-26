#![allow(dead_code)]
/**
 * 思考部だぜ☆（＾～＾）
 */
use kifuwarabe_position::*;
use std::collections::HashSet;

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
