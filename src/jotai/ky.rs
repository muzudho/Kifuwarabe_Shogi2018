/**
 * 局面
 *
 * 後手（上手）から見た盤にすると、
 * 筋と段の方向は　数学のデカルト座標の第一象限のＸ軸、Ｙ軸方向と一致する☆（＾～＾）
 *
 * プログラム上に違いは無いが、ソースコードを読むときは　後手（上手）から
 * 盤を想像すること☆（＾～＾）！
 */

use teigi::conv::*;
use teigi::shogi_syugo::*;
use teigi::shogi_syugo::Koma::*;
use tusin::usi::*;
use jotai::uchu::*;

// 局面
pub struct Kyokumen{
    /**
     * 10の位を筋、1の位を段とする。
     * 0筋、0段は未使用
     */
    ban : [Koma; BAN_SIZE],
    /**
     * 持ち駒数。持ち駒に使える、成らずの駒の部分だけ使用。
     * 増減させたいので、u8 ではなく i8。
     */
    pub mg : [i8; KM_LN],
    /**
     * らいおんの位置
     * [先後]
     */
    pub ms_r : [umasu; SN_LN]
}
impl Kyokumen{
    pub fn new()->Kyokumen{
         Kyokumen{
                // 盤上
                ban:[
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                    Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
                ],
                // 持ち駒数
                mg:[
                    // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
                        0,   0,   0,   0,   0,   0,   0,   0,     0,     0,     0,     0,     0,     0,
                    // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
                        0,   0,   0,   0,   0,   0,   0,   0,     0,     0,     0,     0,     0,     0,
                    // 空マス, 終わり,
                            0,      0,
                ],
                ms_r:[0,0,0],
            }
    }
    pub fn clear(&mut self){
        use teigi::shogi_syugo::Koma::Kara;
        self.ban = [
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
            Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,Kara,
        ];
        self.mg = [
            // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
                0,   0,   0,   0,   0,   0,   0,   0,     0,     0,     0,     0,     0,     0,
            // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
                0,   0,   0,   0,   0,   0,   0,   0,     0,     0,     0,     0,     0,     0,
            // 空マス, 終わり,
                    0,      0,
        ];
    }
    /**
     * 歩が置いてあるか確認
     */
    pub fn exists_fu_by_sn_suji( &self, sn:&Sengo, suji:i8 ) -> bool {
        for dan in DAN_1..DAN_10 {
            let ms = suji_dan_to_ms( suji, dan );
            let km = self.get_km_by_ms( ms );
            let (sn_km,kms) = km_to_sn_kms( &km );
            if match_sn( &sn_km, sn ) && match_kms( &kms, &KmSyurui::H ) {
                return true;
            }
        }
        false
    }
    /**
     * 升で指定して駒を取る
     */
    pub fn get_km_by_ms(&self, ms:umasu)->Koma{
        self.ban[ms]
    }
    /**
     * 升で指定して駒を置く
     */
    pub fn set_km_by_ms(&mut self, ms:umasu, km:Koma){
        self.ban[ms] = km;
        use teigi::shogi_syugo::Sengo::*;
        match km{
            Koma::R0 => { self.ms_r[Sen as usize]=ms },
            Koma::R1 => { self.ms_r[Go  as usize]=ms },
            _ => {},
        }
    }
    /**
     * 持ち駒の枚数を加算
     */
    pub fn add_mg(&mut self, mg:Koma, maisu:i8){
        self.mg[km_to_num(&mg)] += maisu;
    }
    pub fn get_mg(&self, mg:&Koma) -> i8 {
        self.mg[km_to_num(mg)]
    }

    /**
     * 指し手の通りに、盤上の駒配置を動かすぜ☆（＾～＾）
     * 手目のカウントが増えたりはしないぜ☆（＾～＾）
     *
     * return : 取った駒
     */
    pub fn do_sasite(&mut self, sn:&Sengo, ss:&Sasite) -> Koma {

        // 動かす駒
        let km;
        // 取った駒
        let cap;

        // 打かどうか
        if ss.src==SS_SRC_DA {
            km = sn_kms_to_km( &sn, &ss.drop );
            // 自分の持ち駒を減らす
            self.add_mg(km,-1);
        } else {
            // 打で無ければ、元の升の駒を消す。
            if ss.pro {
                // 成りなら
                km = km_to_prokm( &self.get_km_by_ms(ss.src) );
            } else {
                km = self.get_km_by_ms(ss.src);
            }
            self.set_km_by_ms(ss.src, Koma::Kara);
        }

        // 移動先升に駒があるかどうか
        if let Koma::Kara=self.get_km_by_ms(ss.dst) {
            cap = Koma::Kara;
        } else {
            // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
            cap = self.get_km_by_ms(ss.dst);
            let mg = km_to_mg(cap);
            self.add_mg(mg,1);
        }

        // 移動先升に駒を置く
        self.set_km_by_ms(ss.dst, km);

        cap
    }

    /**
     * 指し手の　進む戻る　を逆さにして、盤上の駒配置を動かすぜ☆（＾～＾）
     * 手目のカウントが増えたりはしないぜ☆（＾～＾）
     */
    pub fn undo_sasite(&mut self, sn:&Sengo, ss:&Sasite, cap:&Koma){

        // 移動先の駒
        let km;

        // 打かどうか
        if ss.src==SS_SRC_DA {
            km = sn_kms_to_km( sn, &ss.drop );
            // 自分の持ち駒を増やす
            //let mg = km_to_mg(km);
            //self.add_mg(mg,1);
            self.add_mg(km,1);
        } else {
            // 打で無ければ
            if ss.pro {
                // 成ったなら、成る前へ
                km = prokm_to_km( &self.get_km_by_ms(ss.dst) );
            } else {
                km = self.get_km_by_ms(ss.dst);
            }
        }

        // 移動先の駒を、取った駒（あるいは空）に戻す
        self.set_km_by_ms(ss.dst, *cap);
        match *cap {
            Koma::Kara =>{},
            _ => { 
                // 自分の持ち駒を減らす
                let mg = km_to_mg(*cap);
                self.add_mg(mg,-1);                
            }
        }

        // 移動元升に、動かした駒を置く
        self.set_km_by_ms(ss.src, km);
    }

    /**
     * 指定の升に駒があれば真
     */
    pub fn exists_km( &self, ms:umasu)->bool{
        !match_km( &self.get_km_by_ms(ms), &Koma::Kara)
    }    

    /**
     * 指定の升に指定の駒があれば真
     */
    pub fn has_ms_km( &self, ms:umasu, km:&Koma)->bool{
        match_km( &self.get_km_by_ms(ms), km)
    }    

    /**
     * 指定の升にある駒の先後、または空升
     */
    pub fn get_sn_by_ms( &self, ms:umasu)->Sengo{
        km_to_sn( &self.get_km_by_ms(ms))
    }

    /**
     * 移動先と移動元を比較し、違う駒があれば、成ったと判定するぜ☆（＾～＾）
     */
    pub fn is_natta( &self, ms_src:umasu, ms_dst:umasu )->bool{
        let km_src = &self.get_km_by_ms(ms_src);
        let kms_src = km_to_kms(&km_src);
        let km_dst = &self.get_km_by_ms(ms_dst);
        let kms_dst = km_to_kms(&km_dst);
        // 移動先の駒が成り駒で、 移動元の駒が不成駒なら、成る
        let pro_dst = kms_is_pro(&kms_dst);
        let pro_src = kms_is_pro(&kms_src);

        // 成り
        pro_dst && !pro_src
    }

    /**
     * 局面ハッシュを作り直す
     */
    pub fn create_hash( &self, uchu:&Uchu ) -> u64 {
        let mut hash : u64 = 0;

        // 盤上の駒
        for i_ms in MASU_0..BAN_SIZE {
            let km = self.get_km_by_ms(i_ms as umasu);
            let num_km = km_to_num(&km);
            hash ^= uchu.ky_hash_seed.km[i_ms][num_km];
        }

        // 持ち駒ハッシュ
        for i_km in 0..KM_ARRAY_LN {
            let km = KM_ARRAY[i_km];
            let num_km = km_to_num(&km);

            let maisu = self.get_mg(&km);
            debug_assert!( -1<maisu && maisu <= MG_MAX as i8,
                "持ち駒 {} の枚数 {} <= {}", km, maisu, MG_MAX
            );

            hash ^= uchu.ky_hash_seed.mg[num_km][maisu as usize];
        }

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }
}

