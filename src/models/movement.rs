use consoles::asserts::*;
use memory::ky::*;
use std::fmt;
use teigi::conv::*;

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
        assert_banjo_ms(self.destination,"Ｓasite Ｄisplay");
        let (dx,dy) = ms_to_suji_dan(self.destination);

        if self.source==SS_SRC_DA {
            use memory::ky::KmSyurui;
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
                assert_banjo_ms(self.source,"Ｓasite Ｄisplay＜その２＞");
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
