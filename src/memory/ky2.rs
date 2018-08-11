use GAME_RECORD_WRAP;
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
        for km in KM_ARRAY.iter() {
            num_syugo1.insert( km_to_num(km) );
        }
        let km_syugo = KmSyugo {
            num_syugo : num_syugo1,
        };
        km_syugo
    }
    /// 自分相手
    #[allow(dead_code)]
    pub fn new_jiai(&self, jiai:&Jiai) -> KmSyugo {
        let sn0;
        {
            sn0 = GAME_RECORD_WRAP.try_read().unwrap().get_teban(&jiai);
        }
        let mut num_syugo1 : HashSet<usize> = HashSet::new();
        for km in KM_ARRAY.iter() {
            let (sn1,_kms) = km_to_sn_kms( km );
            if match_sn( &sn0, &sn1 ) {
                num_syugo1.insert( km_to_num(km) );
            }
        }
        let km_syugo = KmSyugo {
            num_syugo : num_syugo1,
        };
        km_syugo
    }
    #[allow(dead_code)]
    pub fn remove( &mut self, km:&Koma ) {
        self.num_syugo.remove( &km_to_num(km) );
    }
}
