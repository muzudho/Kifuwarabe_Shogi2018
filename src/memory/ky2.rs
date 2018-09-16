use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use std::collections::*;

/**
 * 駒集合
 */
#[allow(dead_code)]
pub struct KmSyugo {
    num_syugo : HashSet<usize>,
}
impl KmSyugo {
    /**
     * 全ての元を含む
     */
    #[allow(dead_code)]
    pub fn new_all() -> KmSyugo {
        let mut num_syugo1 : HashSet<usize> = HashSet::new();
        for km in &KM_ARRAY {
            num_syugo1.insert(*km as usize);
        }
        KmSyugo {
            num_syugo : num_syugo1,
        }
    }
    /// 自分相手
    #[allow(dead_code)]
    pub fn new_jiai(&self, jiai:Jiai, game_record: &GameRecord) -> KmSyugo {
        let sn0 = game_record.get_teban(jiai);
        let mut num_syugo1 : HashSet<usize> = HashSet::new();
        for km in &KM_ARRAY {
            let (sn1,_kms) = km_to_sn_kms( *km );
            if sn0 == sn1 {
                num_syugo1.insert(*km as usize);
            }
        }
        KmSyugo {
            num_syugo : num_syugo1,
        }
    }
    #[allow(dead_code)]
    pub fn remove( &mut self, km:Koma ) {
        self.num_syugo.remove( &(km as usize) );
    }
}
