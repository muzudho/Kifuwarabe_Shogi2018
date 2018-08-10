use kifuwarabe_position::*;
use std::fmt;




/**********
 * 論理値 *
 **********/
 /**
  * false => 0
  * true => 1
  *
  * bool は i32 だが、_to_num 系は usize を返すように合わせるぜ☆（*＾～＾*）
  */
#[allow(dead_code)]
pub fn bool_to_num(b:bool) -> usize {
    b as usize
}
/**
 * 0 なら偽、それ以外は真☆（＾～＾）
 */
#[allow(dead_code)]
pub fn num_to_bool(n:usize) -> bool {
    match n {
        0 => false,
        _ => true
    }
}
/**
 * ハッシュ値を作る
 */
#[allow(dead_code)]
pub fn push_bool_to_hash(hash:u64, b:bool) -> u64 {
    // bool は i32 だが、hash は u64 なので u64 に合わせるぜ☆（*＾～＾*）
    (hash<<7) + b as u64
}
/**
 * ハッシュ値から作る
 */
#[allow(dead_code)]
pub fn pop_bool_from_hash(hash:u64) -> (u64, bool) {
    let b_num = num_to_bool( (hash & 0b1) as usize );
    (hash>>7, b_num)
}



/********
 * 手目 *
 ********/
/**
 * 手目数。何手目まで指せるか。
 * 棋譜を残す配列のサイズでもある。
 * 大会ルールが 256手として、終端子として投了を１個入れておけるようにする。
 */
pub const TEME_LN :usize = 257;
/**
 * 同一局面何回で千日手
 */
pub const SENNTITE_NUM :i8 = 4;







/// 棋譜
pub struct GameRecord{
    /// 手目
    pub teme : usize,
    /// 棋譜
    //#[derive(Copy, Clone)]
    pub moves : [Movement; TEME_LN],
}
impl GameRecord {
    pub fn new()->GameRecord{
        GameRecord{
            teme : 0,
            moves : [
                // 1行16要素で並べるぜ☆（＾～＾）
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),  Movement::new(),
                Movement::new()//257要素
            ],
        }
    }
}





/// # Movement (ムーブメント;指し手)
///
/// * `source` - 移動元升。打った場合は 0。
/// * `destination` - 移動先升。これが 0 なら投了とするぜ☆（＾～＾）
/// * `promotion` - 移動後に成るなら真。
/// * `drop` - 打の場合、打った駒種類。
#[derive(Copy,Clone)]
pub struct Movement{
    pub source : umasu,
    pub destination : umasu,
    pub promotion : bool,
    pub drop : KmSyurui,
}
impl Movement{
    pub fn new()->Movement{
        Movement{
            source: 0,
            destination: 0,
            promotion: false,
            drop: KmSyurui::Kara,
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self){
        self.source = 0;
        self.destination = 0;
        self.promotion = false;
        self.drop = KmSyurui::Kara;
    }

    /**
     * 考えた結果、指し手が考え付いていれば真。
     */
    pub fn exists(&self) -> bool{
        self.destination != MASU_0
    }
}
impl Movement{
    pub fn to_hash(&self)->u64{
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_kms_to_hash(hash, &self.drop);
        hash = push_bool_to_hash(hash, self.promotion);
        hash = push_ms_to_hash(hash, self.destination);
        push_ms_to_hash(hash, self.source)
    }
    pub fn from_hash(hash:u64)->Movement{
        // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
        let (hash,src) = pop_ms_from_hash(hash);
        let (hash,dst) = pop_ms_from_hash(hash);
        let (hash,pro) = pop_bool_from_hash(hash);
        let (_hash,drop) = pop_kms_from_hash(hash);
        Movement{
            source: src,
            destination: dst,
            promotion: pro,
            drop: drop,
        }
    }
}
impl fmt::Display for Movement{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {

        // 手が何もない、ぐらいの意味だが、
        // その手を指す場合、投了表示
        if !self.exists() { return write!(f,"resign"); }

        // 投了を弾いたあと、診断☆（＾～＾）
        //assert_banjo_ms(self.destination,"Ｓasite Ｄisplay");
        let (dx,dy) = ms_to_suji_dan(self.destination);

        if self.source==SS_SRC_DA {
            use kifuwarabe_position::KmSyurui;
            write!(f, "{}*{}{}{}",
                match self.drop {
                    KmSyurui::K => { "R" },
                    KmSyurui::Z => { "B" },
                    KmSyurui::I => { "G" },
                    KmSyurui::N => { "S" },
                    KmSyurui::U => { "N" },
                    KmSyurui::S => { "L" },
                    KmSyurui::H => { "P" },
                    _  => { "?" },
                },
                dx,
                num_to_lower_case(dy),
                if self.promotion {"+"}else{""}
            )
        } else {
            let (sx,sy) = if self.source==MASU_0 {
                // エラー・データも表示したい
                 (0,0)
            } else {
                //assert_banjo_ms(self.source,"Ｓasite Ｄisplay＜その２＞");
                ms_to_suji_dan(self.source)
            };
            write!(f, "{}{}{}{}{}",
                sx,
                num_to_lower_case(sy),
                dx,
                num_to_lower_case(dy),
                if self.promotion {"+"}else{""}
            )
        }
    }
}
impl fmt::Debug for Movement{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Movement(source:{}, destination:{}, promotion:{}, drop:{})", self.source, self.destination, self.promotion, self.drop)
    }
}






/**
 * 指し手の通りに、盤上の駒配置を動かすぜ☆（＾～＾）
 * 手目のカウントが増えたりはしないぜ☆（＾～＾）
 *
 * return : 取った駒
 */
pub fn make_movement(sn:&Sengo, ss:&Movement, position: &mut Kyokumen) -> Koma {
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
pub fn unmake_movement(sn:&Sengo, ss:&Movement, cap:&Koma, position: &mut Kyokumen){
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
