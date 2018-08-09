use CUR_POSITION_WRAP;
use kifuwarabe_position::*;
use models::movement::*;
use std::collections::*;
use UCHU_WRAP;

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
        let sn0 = UCHU_WRAP.try_read().unwrap().get_teban(&jiai);
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

/**
 * 指し手の通りに、盤上の駒配置を動かすぜ☆（＾～＾）
 * 手目のカウントが増えたりはしないぜ☆（＾～＾）
 *
 * return : 取った駒
 */
pub fn do_sasite(sn:&Sengo, ss:&Movement) -> Koma {
    let mut position = CUR_POSITION_WRAP.try_write().unwrap();
    // 動かす駒
    let km;
    // 取った駒
    let cap;

    // 打かどうか
    if ss.source==SS_SRC_DA {
        km = sn_kms_to_km( &sn, &ss.drop );
        // 自分の持ち駒を減らす
        position.add_mg(km,-1);
    } else {
        // 打で無ければ、元の升の駒を消す。
        if ss.promotion {
            // 成りなら
            km = km_to_prokm( &position.get_km_by_ms(ss.source) );
        } else {
            km = position.get_km_by_ms(ss.source);
        }
        position.set_km_by_ms(ss.source, Koma::Kara);
    }

    // 移動先升に駒があるかどうか
    if let Koma::Kara=position.get_km_by_ms(ss.destination) {
        cap = Koma::Kara;
    } else {
        // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
        cap = position.get_km_by_ms(ss.destination);
        let mg = km_to_mg(cap);
        position.add_mg(mg,1);
    }

    // 移動先升に駒を置く
    position.set_km_by_ms(ss.destination, km);

    cap
}

/**
 * 指し手の　進む戻る　を逆さにして、盤上の駒配置を動かすぜ☆（＾～＾）
 * 手目のカウントが増えたりはしないぜ☆（＾～＾）
 */
pub fn undo_sasite(sn:&Sengo, ss:&Movement, cap:&Koma){
    let mut position = CUR_POSITION_WRAP.try_write().unwrap();
    // 移動先の駒
    let km;

    // 打かどうか
    if ss.source==SS_SRC_DA {
        km = sn_kms_to_km(sn, &ss.drop);
        // 自分の持ち駒を増やす
        position.add_mg(km,1);
    } else {
        // 打で無ければ
        if ss.promotion {
            // 成ったなら、成る前へ
            km = prokm_to_km( &position.get_km_by_ms(ss.destination) );
        } else {
            km = position.get_km_by_ms(ss.destination);
        }
    }

    // 移動先の駒を、取った駒（あるいは空）に戻す
    position.set_km_by_ms(ss.destination, *cap);
    match *cap {
        Koma::Kara =>{},
        _ => { 
            // 自分の持ち駒を減らす
            let mg = km_to_mg(*cap);
            position.add_mg(mg,-1);                
        }
    }

    // 移動元升に、動かした駒を置く
    position.set_km_by_ms(ss.source, km);
}
