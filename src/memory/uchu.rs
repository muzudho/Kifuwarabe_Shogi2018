/**
 * 宇宙
 *
 * グローバル変数の作り方が分からないので仕方なく多くの変数を詰め込んでいるぜ☆（＾～＾）
 * 引数で渡しまくれだぜ☆（＾～＾）
 */
extern crate rand;

// use config::*;
use kifuwarabe_position::*;
use kifuwarabe_movement_picker::*;
use LOGGER;
use memory::number_board::*;
// use std::fs::File;
// use std::io::Write;
// use std::path::Path;
// use std::sync::Mutex;
use thinks::visions::vision_tree::*;


/**
 * グローバル変数の作り方が分からないので、
 * ここに全部入れてあるぜ☆（＾～＾）
 */
pub struct Uchu{
    // 空打ちでタイトル画面を出すなら真。
    pub title_dirty : bool,
    // 利きの数（先後別）
    pub kiki_su_by_sn : [NumberBoard; Sengo::Num as usize],
    // 利きの数（先後付き駒別）
    pub kiki_su_by_km : [NumberBoard; Koma::Num as usize],
    // ビジョン・ツリー
    pub vision_tree_by_sn : [VisionTree; Sengo::Num as usize],
}
impl Uchu{
    pub fn set_kiki_su_by_sn(&mut self, kiki_su_by_sn:[NumberBoard; Sengo::Num as usize]){
        self.kiki_su_by_sn = kiki_su_by_sn
    }
    pub fn set_kiki_su_by_km(&mut self, kiki_su_by_km:[NumberBoard; Koma::Num as usize]){
        self.kiki_su_by_km = kiki_su_by_km
    }
    pub fn new()->Uchu{
        Uchu{
            title_dirty : true,
            // 利き数（先後別）
            kiki_su_by_sn : [
                NumberBoard::new(), NumberBoard::new(),
            ],
            // 利き数（駒別なので３０個ある）
            kiki_su_by_km : [
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
            ],
            vision_tree_by_sn : [
                VisionTree::new(), VisionTree::new(),
            ],
        }
    }

    /* ******
     * 棋譜 *
     ********/


    /**
     * 表示
     */
    pub fn kaku_number_board(&self, sn:Sengo, km:Koma)->String{

        let nb = match sn {
            Sengo::Num    => { &self.kiki_su_by_km[km as usize] },
            _             => { &self.kiki_su_by_sn[sn as usize] },
        };

        // 数盤表示
        format!("    +----+----+----+----+----+----+----+----+----+
i9  |{0:4}|{1:4}|{2:4}|{3:4}|{4:4}|{5:4}|{6:4}|{7:4}|{8:4}|
    +----+----+----+----+----+----+----+----+----+
h8  |{9:4}|{10:4}|{11:4}|{12:4}|{13:4}|{14:4}|{15:4}|{16:4}|{17:4}|
    +----+----+----+----+----+----+----+----+----+
g7  |{18:4}|{19:4}|{20:4}|{21:4}|{22:4}|{23:4}|{24:4}|{25:4}|{26:4}|
    +----+----+----+----+----+----+----+----+----+
f6  |{27:4}|{28:4}|{29:4}|{30:4}|{31:4}|{32:4}|{33:4}|{34:4}|{35:4}|
    +----+----+----+----+----+----+----+----+----+
e5  |{36:4}|{37:4}|{38:4}|{39:4}|{40:4}|{41:4}|{42:4}|{43:4}|{44:4}|
    +----+----+----+----+----+----+----+----+----+
d4  |{45:4}|{46:4}|{47:4}|{48:4}|{49:4}|{50:4}|{51:4}|{52:4}|{53:4}|
    +----+----+----+----+----+----+----+----+----+
c3  |{54:4}|{55:4}|{56:4}|{57:4}|{58:4}|{59:4}|{60:4}|{61:4}|{62:4}|
    +----+----+----+----+----+----+----+----+----+
b2  |{63:4}|{64:4}|{65:4}|{66:4}|{67:4}|{68:4}|{69:4}|{70:4}|{71:4}|
    +----+----+----+----+----+----+----+----+----+
a1  |{72:4}|{73:4}|{74:4}|{75:4}|{76:4}|{77:4}|{78:4}|{79:4}|{80:4}|
    +----+----+----+----+----+----+----+----+----+
       1    2    3    4    5    6    7    8    9\
",
            nb.get_su_by_ms(19),nb.get_su_by_ms(29),nb.get_su_by_ms(39),nb.get_su_by_ms(49),nb.get_su_by_ms(59),nb.get_su_by_ms(69),nb.get_su_by_ms(79),nb.get_su_by_ms(89),nb.get_su_by_ms(99),
            nb.get_su_by_ms(18),nb.get_su_by_ms(28),nb.get_su_by_ms(38),nb.get_su_by_ms(48),nb.get_su_by_ms(58),nb.get_su_by_ms(68),nb.get_su_by_ms(78),nb.get_su_by_ms(88),nb.get_su_by_ms(98),
            nb.get_su_by_ms(17),nb.get_su_by_ms(27),nb.get_su_by_ms(37),nb.get_su_by_ms(47),nb.get_su_by_ms(57),nb.get_su_by_ms(67),nb.get_su_by_ms(77),nb.get_su_by_ms(87),nb.get_su_by_ms(97),
            nb.get_su_by_ms(16),nb.get_su_by_ms(26),nb.get_su_by_ms(36),nb.get_su_by_ms(46),nb.get_su_by_ms(56),nb.get_su_by_ms(66),nb.get_su_by_ms(76),nb.get_su_by_ms(86),nb.get_su_by_ms(96),
            nb.get_su_by_ms(15),nb.get_su_by_ms(25),nb.get_su_by_ms(35),nb.get_su_by_ms(45),nb.get_su_by_ms(55),nb.get_su_by_ms(65),nb.get_su_by_ms(75),nb.get_su_by_ms(85),nb.get_su_by_ms(95),
            nb.get_su_by_ms(14),nb.get_su_by_ms(24),nb.get_su_by_ms(34),nb.get_su_by_ms(44),nb.get_su_by_ms(54),nb.get_su_by_ms(64),nb.get_su_by_ms(74),nb.get_su_by_ms(84),nb.get_su_by_ms(94),
            nb.get_su_by_ms(13),nb.get_su_by_ms(23),nb.get_su_by_ms(33),nb.get_su_by_ms(43),nb.get_su_by_ms(53),nb.get_su_by_ms(63),nb.get_su_by_ms(73),nb.get_su_by_ms(83),nb.get_su_by_ms(93),
            nb.get_su_by_ms(12),nb.get_su_by_ms(22),nb.get_su_by_ms(32),nb.get_su_by_ms(42),nb.get_su_by_ms(52),nb.get_su_by_ms(62),nb.get_su_by_ms(72),nb.get_su_by_ms(82),nb.get_su_by_ms(92),
            nb.get_su_by_ms(11),nb.get_su_by_ms(21),nb.get_su_by_ms(31),nb.get_su_by_ms(41),nb.get_su_by_ms(51),nb.get_su_by_ms(61),nb.get_su_by_ms(71),nb.get_su_by_ms(81),nb.get_su_by_ms(91),
        )
    }

    // 駒の動きを出力
    pub fn hyoji_kmugoki(&self){
        for kms in &KMS_ARRAY {
            LOGGER.try_write().unwrap().write(&format!( "{} ", kms ));
            self.hyoji_kmugoki_dir( *kms );
            LOGGER.try_write().unwrap().writeln("");//改行
        }
    }
    pub fn hyoji_kmugoki_dir(&self, kms:KmSyurui ){
        for kmdir in &KM_UGOKI.back[kms as usize] {
            match kmdir {
                KmDir::Num => break,
                _ => LOGGER.try_write().unwrap().write(&format!( "{},", kmdir))
            }
        }
    }


    #[allow(dead_code)]
    pub fn remake_visions(&mut self) {
        for sn in &SN_ARRAY {
            // 全部忘れる☆（＾～＾）
            self.vision_tree_by_sn[*sn as usize].clear();
        }
    }
}
