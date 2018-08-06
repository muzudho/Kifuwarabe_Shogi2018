/**
 * 局面
 *
 * 後手（上手）から見た盤にすると、
 * 筋と段の方向は　数学のデカルト座標の第一象限のＸ軸、Ｙ軸方向と一致する☆（＾～＾）
 *
 * プログラム上に違いは無いが、ソースコードを読むときは　後手（上手）から
 * 盤を想像すること☆（＾～＾）！
 */

use memory::ky::Koma::*;
use std::*;
use std::collections::*;
use teigi::conv::*;
use teigi::shogi_syugo::*;
use UCHU_WRAP;





/******************
 * 盤、升、筋、段 *
 ******************/
/*
 * 盤の符号は、後手番から見る
 *
 *
 * 19  29  39  49  59  69  79  89  99
 * 18  28  38  48  58  68  78  88  98
 * 17  27  37  47  57  67  77  87  97
 * 16  26  36  46  56  66  76  86  96
 * 15  25  35  45  55  65  75  85  95
 * 14  24  34  44  54  64  74  84  94
 * 13  23  33  43  53  63  73  83  93
 * 12  22  32  42  52  62  72  82  92
 * 11  21  31  41  51  61  71  81  91
 *
 */
/**
 * 盤を回転するのに使うぜ☆（＾～＾）
 */
pub const BAN_MIN :usize = 11;
/**
 * 盤を回転するのに使うぜ☆（＾～＾）
 */
pub const BAN_MAX :usize = 99;
/**
 * 盤のヨコ幅、タテ幅。
 * 筋と段は x,y とは逆方向なので、幅も左端、下端を指す。
 */
//pub const BAN_W :i8 = 9;
//pub const BAN_H :i8 = 9;
pub const BAN_SIZE :usize = 100;
// 1辺の長さ
//pub const BAN_LINE :usize = 10;
/**
 * 筋、段は 1 から始まる、という明示。
 * 増減はよく使うので u8 ではなく i8 にした。
 */
pub const SUJI_0 :i8 = 0;
pub const SUJI_1 :i8 = 1;
#[allow(dead_code)]
pub const SUJI_9 :i8 = 9;
pub const SUJI_10 :i8 = 10;
pub const DAN_0 :i8 = 0;
pub const DAN_1 :i8 = 1;
pub const DAN_2 :i8 = 2;
pub const DAN_3 :i8 = 3;
pub const DAN_4 :i8 = 4;
#[allow(dead_code)]
pub const DAN_5 :i8 = 5;
pub const DAN_6 :i8 = 6;
pub const DAN_7 :i8 = 7;
pub const DAN_8 :i8 = 8;//うさぎの打てる段の上限
#[allow(dead_code)]
pub const DAN_9 :i8 = 9;
pub const DAN_10 :i8 = 10;
/**
 * 升番号 0～99。
 * 10の位を筋、1の位を段とする。0筋、0段は未使用（番兵として使用）
 * 該当なしの場合 0 を使う
 */
 #[allow(non_camel_case_types)]
pub type umasu = usize;
/**
 * 升の検索等で、該当なしの場合
 */
pub const MASU_0 : umasu = 0;

/**
 * 指し手。打の場合のsrc
 */
pub const SS_SRC_DA : umasu = 0;




/******
 * 駒 *
 ******/
/// 先後付き駒

/// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const MG_MAX : usize = 18;
pub const KM_LN : usize = 30;
/// 先後付きの駒と空白
#[derive(Copy, Clone)]
pub enum Koma{
    // ▼らいおん
    R0,
    // ▼きりん
    K0,
    // ▼ぞう
    Z0,
    // ▼いぬ
    I0,
    // ▼ねこ
    N0,
    // ▼うさぎ
    U0,
    // ▼いのしし
    S0,
    // ▼ひよこ
    H0,
    // ▼ぱわーあっぷきりん
    PK0,
    // ▼ぱわーあっぷぞう
    PZ0,
    // ▼ぱわーあっぷねこ
    PN0,
    // ▼ぱわーあっぷうさぎ
    PU0,
    // ▼ぱわーあっぷいのしし
    PS0,
    // ▼ぱわーあっぷひよこ
    PH0,
    // △ライオン
    R1,
    // △キリン
    K1,
    // △ゾウ
    Z1,
    // △イヌ
    I1,
    // △ネコ
    N1,
    // △ウサギ
    U1,
    // △イノシシ
    S1,
    // △ヒヨコ
    H1,
    // △パワーアップキリン
    PK1,
    // △パワーアップゾウ
    PZ1,
    // △パワーアップネコ
    PN1,
    // △パワーアップウサギ
    PU1,
    // △パワーアップイノシシ
    PS1,
    // △パワーアップヒヨコ
    PH1,
    // 空マス
    Kara,
    // 要素数より1小さい数。該当なしや、エラー値用としても兼用する
    Owari
}
impl fmt::Display for Koma{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use memory::ky::Koma::*;
        match *self{
            R0 => { write!(f,"▼ら")},
            K0 => { write!(f,"▼き")},
            Z0 => { write!(f,"▼ぞ")},
            I0 => { write!(f,"▼い")},
            N0 => { write!(f,"▼ね")},
            U0 => { write!(f,"▼う")},
            S0 => { write!(f,"▼し")},
            H0 => { write!(f,"▼ひ")},
            PK0 => { write!(f,"▼PK")},
            PZ0 => { write!(f,"▼PZ")},
            PN0 => { write!(f,"▼PN")},
            PU0 => { write!(f,"▼PU")},
            PS0 => { write!(f,"▼PS")},
            PH0 => { write!(f,"▼PH")},
            R1 => { write!(f,"△ラ")},
            K1 => { write!(f,"△キ")},
            Z1 => { write!(f,"△ゾ")},
            I1 => { write!(f,"△イ")},
            N1 => { write!(f,"△ネ")},
            U1 => { write!(f,"△ウ")},
            S1 => { write!(f,"△シ")},
            H1 => { write!(f,"△ヒ")},
            PK1 => { write!(f,"△pk")},
            PZ1 => { write!(f,"△pz")},
            PN1 => { write!(f,"△pn")},
            PU1 => { write!(f,"△pu")},
            PS1 => { write!(f,"△ps")},
            PH1 => { write!(f,"△ph")},
            Kara => { write!(f,"　　")},
            Owari => { write!(f,"××")},
        }
    }
}
/**
 * 駒の一致比較
 */
pub fn match_km(a:&Koma, b:&Koma)->bool{
    km_to_num(a) == km_to_num(b)
}

#[allow(dead_code)]
pub const KM_ARRAY_HALF_LN : usize = 14;
pub const KM_ARRAY_LN : usize = 28;
pub const KM_ARRAY : [Koma;KM_ARRAY_LN] = [
    Koma::R0,// らいおん
    Koma::K0,// きりん
    Koma::Z0,// ぞう
    Koma::I0,// いぬ
    Koma::N0,// ねこ
    Koma::U0,// うさぎ
    Koma::S0,// いのしし
    Koma::H0,// ひよこ
    Koma::PK0,// ぱわーあっぷきりん
    Koma::PZ0,// ぱわーあっぷぞう
    Koma::PN0,// ぱわーあっぷねこ
    Koma::PU0,// ぱわーあっぷうさぎ
    Koma::PS0,// ぱわーあっぷいのしし
    Koma::PH0,// ぱわーあっぷひよこ
    Koma::R1,// らいおん
    Koma::K1,// きりん
    Koma::Z1,// ぞう
    Koma::I1,// いぬ
    Koma::N1,// ねこ
    Koma::U1,// うさぎ
    Koma::S1,// いのしし
    Koma::H1,// ひよこ
    Koma::PK1,// ぱわーあっぷきりん
    Koma::PZ1,// ぱわーあっぷぞう
    Koma::PN1,// ぱわーあっぷねこ
    Koma::PU1,// ぱわーあっぷうさぎ
    Koma::PS1,// ぱわーあっぷいのしし
    Koma::PH1,// ぱわーあっぷひよこ
];
#[allow(dead_code)]
pub const SN_KM_ARRAY : [[Koma;KM_ARRAY_HALF_LN];SN_LN] = [
    [
        Koma::R0,// らいおん
        Koma::K0,// きりん
        Koma::Z0,// ぞう
        Koma::I0,// いぬ
        Koma::N0,// ねこ
        Koma::U0,// うさぎ
        Koma::S0,// いのしし
        Koma::H0,// ひよこ
        Koma::PK0,// ぱわーあっぷきりん
        Koma::PZ0,// ぱわーあっぷぞう
        Koma::PN0,// ぱわーあっぷねこ
        Koma::PU0,// ぱわーあっぷうさぎ
        Koma::PS0,// ぱわーあっぷいのしし
        Koma::PH0,// ぱわーあっぷひよこ
    ],
    [
        Koma::R1,// らいおん
        Koma::K1,// きりん
        Koma::Z1,// ぞう
        Koma::I1,// いぬ
        Koma::N1,// ねこ
        Koma::U1,// うさぎ
        Koma::S1,// いのしし
        Koma::H1,// ひよこ
        Koma::PK1,// ぱわーあっぷきりん
        Koma::PZ1,// ぱわーあっぷぞう
        Koma::PN1,// ぱわーあっぷねこ
        Koma::PU1,// ぱわーあっぷうさぎ
        Koma::PS1,// ぱわーあっぷいのしし
        Koma::PH1,// ぱわーあっぷひよこ
    ],
    [
        Koma::Owari,// らいおん
        Koma::Owari,// きりん
        Koma::Owari,// ぞう
        Koma::Owari,// いぬ
        Koma::Owari,// ねこ
        Koma::Owari,// うさぎ
        Koma::Owari,// いのしし
        Koma::Owari,// ひよこ
        Koma::Owari,// ぱわーあっぷきりん
        Koma::Owari,// ぱわーあっぷぞう
        Koma::Owari,// ぱわーあっぷねこ
        Koma::Owari,// ぱわーあっぷうさぎ
        Koma::Owari,// ぱわーあっぷいのしし
        Koma::Owari,// ぱわーあっぷひよこ
    ],
];
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
 * 局面ハッシュ種
 * ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
 */
pub struct KyHashSeed {
    // 盤上の駒
    pub km : [[u64;KM_LN];BAN_SIZE],
    // 持ち駒
    pub mg : [[u64;MG_MAX];KM_LN],
    // 先後
    pub sn : [u64;SN_LN],
}

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
        use memory::ky::Koma::Kara;
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
    #[allow(dead_code)]
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
    pub fn create_hash(&self, ky_hash_seed: &KyHashSeed) -> u64 {

        let mut hash : u64 = 0;

        // 盤上の駒
        for i_ms in MASU_0..BAN_SIZE {
            let km = self.get_km_by_ms(i_ms as umasu);
            let num_km = km_to_num(&km);
            hash ^= ky_hash_seed.km[i_ms][num_km];
        }

        // 持ち駒ハッシュ
        for i_km in 0..KM_ARRAY_LN {
            let km = KM_ARRAY[i_km];
            let num_km = km_to_num(&km);

            let maisu = self.get_mg(&km);
            debug_assert!( -1<maisu && maisu <= MG_MAX as i8,
                "持ち駒 {} の枚数 {} <= {}", km, maisu, MG_MAX
            );

            hash ^= ky_hash_seed.mg[num_km][maisu as usize];
        }

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }
}

