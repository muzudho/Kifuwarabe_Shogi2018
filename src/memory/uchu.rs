/**
 * 宇宙
 *
 * グローバル変数の作り方が分からないので仕方なく多くの変数を詰め込んでいるぜ☆（＾～＾）
 * 引数で渡しまくれだぜ☆（＾～＾）
 */
extern crate rand;
use rand::Rng;

use CUR_POSITION_WRAP;
use config::*;
use GAME_RECORD_WRAP;
use INI_POSITION_WRAP;
use kifuwarabe_position::*;
use memory::number_board::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use thinks::visions::vision_tree::*;
use teigi::shogi_syugo::*;
use tusin::us_conv::*;


/**
 * グローバル定数
 *
 * 使い方（lazy_static!マクロ）
 * ============================
 * 定数の値を実行時に決めることができる。
 *
 * Cargo.toml に１行追記
 * > [dependencies]
 * > lazy_static = "1.0.0"
 *
 * main.rs の冒頭あたりに次の２行を記述
 * > #[macro_use]
 * > extern crate lazy_static;
 *
 * 「How can I use mutable lazy_static?」
 * https://users.rust-lang.org/t/how-can-i-use-mutable-lazy-static/3751/3
 */
lazy_static! {
    /**
      * ログ・ファイル
      */
    pub static ref LOGFILE: Mutex<File> = {
        // File::createの返り値は`io::Result<File>` なので .unwrap() で中身を取り出す
        Mutex::new(File::create(Path::new(LOG_FILE_PATH)).unwrap())
    };
}

#[allow(dead_code)]
pub fn g_write(s:&str){
    println!("{}",s);
    if LOG_ENABLE {
        // write_allメソッドを使うには use std::io::Write; が必要
        match LOGFILE.lock().unwrap().write_all(s.as_bytes()) {
            // 大会向けに、ログ書き込み失敗は出力しないことにする
            Err(_why) => {},//panic!("couldn't write log. : {}",Error::description(&why)),
            Ok(_) => {},
        }
    }
}
#[allow(dead_code)]
pub fn g_writeln(s:&str){
    println!("{}",s);
    if LOG_ENABLE {
        match LOGFILE.lock().unwrap().write_all(format!("{}\n",s).as_bytes()) {
            Err(_why) => {},
            Ok(_) => {},
        }
    }
}



/**
 * グローバル変数の作り方が分からないので、
 * ここに全部入れてあるぜ☆（＾～＾）
 */
pub struct Uchu{
    // 対話モード
    pub dialogue_mode : bool,
    // 利きの数（先後別）
    pub kiki_su_by_sn : [NumberBoard; SN_LN],
    // 利きの数（先後付き駒別）
    pub kiki_su_by_km : [NumberBoard; KM_LN],
    // ビジョン・ツリー
    pub vision_tree_by_sn : [VisionTree; SN_LN],
}
impl Uchu{
    pub fn set_kiki_su_by_sn(&mut self, kiki_su_by_sn:[NumberBoard; SN_LN]){
        self.kiki_su_by_sn = kiki_su_by_sn
    }
    pub fn set_kiki_su_by_km(&mut self, kiki_su_by_km:[NumberBoard; KM_LN]){
        self.kiki_su_by_km = kiki_su_by_km
    }
    pub fn new()->Uchu{
        Uchu{
            dialogue_mode : false,
            // 利き数（先後別）
            kiki_su_by_sn : [
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
            ],
            // 利き数（駒別なので３０個ある）
            kiki_su_by_km : [
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
                NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(), NumberBoard::new(),
            ],
            vision_tree_by_sn : [
                VisionTree::new(), VisionTree::new(), VisionTree::new(),
            ],
        }
    }
    /**
     * 宇宙誕生
     */
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット

        let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();

        // 盤上の駒
        for i_ms in MASU_0..BAN_SIZE {
            for i_km in 0..KM_LN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                game_record.ky_hash_seed.km[i_ms][i_km] = rand::thread_rng().gen_range(0,18446744073709551615);
            }
        }
        // 持ち駒
        for i_km in 0..KM_LN {
            for i_mg in 0..MG_MAX {
                game_record.ky_hash_seed.mg[i_km][i_mg] = rand::thread_rng().gen_range(0,18446744073709551615);
            }
        }
        // 先後
        for i_sn in 0..SN_LN {
            game_record.ky_hash_seed.sn[i_sn] = rand::thread_rng().gen_range(0,18446744073709551615);
        }
    }
    /**
     * 初期局面、現局面ともにクリアーします。
     * 手目も 0 に戻します。
     */
    pub fn clear_ky01(&mut self){
        INI_POSITION_WRAP.try_write().unwrap().clear();
        CUR_POSITION_WRAP.try_write().unwrap().clear();
        GAME_RECORD_WRAP.try_write().unwrap().set_teme(0);
    }
    /**
     * 初期局面を、現局面にコピーします
    pub fn copy_ky0_to_ky1(&mut self){
        // グローバル変数を使う。
        {
            let ini_pos =  INI_POSITION_WRAP.try_read().unwrap();
            // 盤上
            for i_ms in 0..BAN_SIZE{
                CUR_POSITION_WRAP.try_write().unwrap().set_km_by_ms(i_ms, ini_pos.get_km_by_ms(i_ms));
            }

            // 持ち駒
            for i_mg in 0..KM_LN{
                CUR_POSITION_WRAP.try_write().unwrap().mg[i_mg] = ini_pos.mg[i_mg];
            }
        }
    }
     */

    /* ******
     * 棋譜 *
     ********/

    /// # Examples.
    ///
    /// let s = uchu.kaku_kifu();
    /// g_writeln( &s );
    pub fn kaku_kifu(&self)->String{
        let mut s = String::new();
        let teme: usize;
        {
            teme = GAME_RECORD_WRAP.try_read().unwrap().teme;
        }
        for i_teme in 0..teme {
            let ss;
            {
                let game_record = GAME_RECORD_WRAP.try_read().unwrap();
                ss = game_record.moves[i_teme];
            }
            s.push_str(&format!("[{}] {}", i_teme, movement_to_usi(&ss)));
        }
        s
    }
    pub fn kaku_ky_hash(&self)->String{
        let game_record = GAME_RECORD_WRAP.try_read().unwrap();
        let mut s = String::new();
        let teme: usize;
        {
            s.push_str(&format!("[ini] {:20}\n", &game_record.ky0_hash ));
            teme = game_record.teme;
        }
        for i_teme in 0..teme {
            let hash = &game_record.ky_hash[i_teme];
            // 64bitは10進数20桁。改行する
            s.push_str(&format!("[{:3}] {:20}\n", i_teme, hash));
        }
        s
    }

    /**
    * 自陣
    */
    #[allow(dead_code)]
    pub fn get_ji_jin(&self)->Vec<umasu>{
        let game_record = GAME_RECORD_WRAP.try_read().unwrap();
        if let Sengo::Sen = game_record.get_teban(&Jiai::Ji) {
            SenteJin::to_elm()
        } else {
            GoteJin::to_elm()
        }
    }
    /**
    * 相手陣
    */
    #[allow(dead_code)]
    pub fn get_aite_jin(&self)->Vec<umasu>{
        let game_record = GAME_RECORD_WRAP.try_read().unwrap();
        if let Sengo::Sen = game_record.get_teban(&Jiai::Ji) {
            GoteJin::to_elm()
        } else {
            SenteJin::to_elm()
        }
    }

    /**
     * 表示
     *
     * 後手から見た盤を表示するぜ☆（＾～＾）
     * デカルト座標の第一象限と x,y 方向が一致するメリットがあるぜ☆（＾～＾）
     */
    pub fn kaku_ky(&self, position: &Position, visible_record: bool) -> String {

        let mut teme = 0;
        let mut teban = Sengo::Sen;
        let mut same = 0;
        if visible_record {
            let game_record = GAME_RECORD_WRAP.try_read().unwrap();
            teme = game_record.get_teme();
            teban = game_record.get_teban(&Jiai::Ji);
            same = game_record.count_same_ky();
        }

        let ky = position;
        // 局面表示
        format!("\
表示 {95}手目 {96} 同一局面{97}回目

           +----+----+----+----+----+----+----+----+----+
        i9 |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|
           +----+----+----+----+----+----+----+----+----+
ひx{87:2}   h8 |{9}|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|   ヒx{94:2}
           +----+----+----+----+----+----+----+----+----+
しx{86:2}   g7 |{18}|{19}|{20}|{21}|{22}|{23}|{24}|{25}|{26}|   シx{93:2}
           +----+----+----+----+----+----+----+----+----+
うx{85:2}   f6 |{27}|{28}|{29}|{30}|{31}|{32}|{33}|{34}|{35}|   ウx{92:2}
           +----+----+----+----+----+----+----+----+----+
ねx{84:2}   e5 |{36}|{37}|{38}|{39}|{40}|{41}|{42}|{43}|{44}|   ネx{91:2}
           +----+----+----+----+----+----+----+----+----+
いx{83:2}   d4 |{45}|{46}|{47}|{48}|{49}|{50}|{51}|{52}|{53}|   イx{90:2}
           +----+----+----+----+----+----+----+----+----+
ぞx{82:2}   c3 |{54}|{55}|{56}|{57}|{58}|{59}|{60}|{61}|{62}|   ゾx{89:2}
           +----+----+----+----+----+----+----+----+----+
きx{81:2}   b2 |{63}|{64}|{65}|{66}|{67}|{68}|{69}|{70}|{71}|   キx{88:2}
           +----+----+----+----+----+----+----+----+----+
▼      a1 |{72}|{73}|{74}|{75}|{76}|{77}|{78}|{79}|{80}|   △
           +----+----+----+----+----+----+----+----+----+
              1    2    3    4    5    6    7    8    9\
",
            ky.get_km_by_ms(19),ky.get_km_by_ms(29),ky.get_km_by_ms(39),ky.get_km_by_ms(49),ky.get_km_by_ms(59),ky.get_km_by_ms(69),ky.get_km_by_ms(79),ky.get_km_by_ms(89),ky.get_km_by_ms(99),
            ky.get_km_by_ms(18),ky.get_km_by_ms(28),ky.get_km_by_ms(38),ky.get_km_by_ms(48),ky.get_km_by_ms(58),ky.get_km_by_ms(68),ky.get_km_by_ms(78),ky.get_km_by_ms(88),ky.get_km_by_ms(98),
            ky.get_km_by_ms(17),ky.get_km_by_ms(27),ky.get_km_by_ms(37),ky.get_km_by_ms(47),ky.get_km_by_ms(57),ky.get_km_by_ms(67),ky.get_km_by_ms(77),ky.get_km_by_ms(87),ky.get_km_by_ms(97),
            ky.get_km_by_ms(16),ky.get_km_by_ms(26),ky.get_km_by_ms(36),ky.get_km_by_ms(46),ky.get_km_by_ms(56),ky.get_km_by_ms(66),ky.get_km_by_ms(76),ky.get_km_by_ms(86),ky.get_km_by_ms(96),
            ky.get_km_by_ms(15),ky.get_km_by_ms(25),ky.get_km_by_ms(35),ky.get_km_by_ms(45),ky.get_km_by_ms(55),ky.get_km_by_ms(65),ky.get_km_by_ms(75),ky.get_km_by_ms(85),ky.get_km_by_ms(95),
            ky.get_km_by_ms(14),ky.get_km_by_ms(24),ky.get_km_by_ms(34),ky.get_km_by_ms(44),ky.get_km_by_ms(54),ky.get_km_by_ms(64),ky.get_km_by_ms(74),ky.get_km_by_ms(84),ky.get_km_by_ms(94),
            ky.get_km_by_ms(13),ky.get_km_by_ms(23),ky.get_km_by_ms(33),ky.get_km_by_ms(43),ky.get_km_by_ms(53),ky.get_km_by_ms(63),ky.get_km_by_ms(73),ky.get_km_by_ms(83),ky.get_km_by_ms(93),
            ky.get_km_by_ms(12),ky.get_km_by_ms(22),ky.get_km_by_ms(32),ky.get_km_by_ms(42),ky.get_km_by_ms(52),ky.get_km_by_ms(62),ky.get_km_by_ms(72),ky.get_km_by_ms(82),ky.get_km_by_ms(92),
            ky.get_km_by_ms(11),ky.get_km_by_ms(21),ky.get_km_by_ms(31),ky.get_km_by_ms(41),ky.get_km_by_ms(51),ky.get_km_by_ms(61),ky.get_km_by_ms(71),ky.get_km_by_ms(81),ky.get_km_by_ms(91),
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            ky.mg[Koma::K0 as usize],ky.mg[Koma::Z0 as usize],ky.mg[Koma::I0 as usize],ky.mg[Koma::N0 as usize],ky.mg[Koma::U0 as usize],ky.mg[Koma::S0 as usize],ky.mg[Koma::H0 as usize],
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            ky.mg[Koma::K1 as usize],ky.mg[Koma::Z1 as usize],ky.mg[Koma::I1 as usize],ky.mg[Koma::N1 as usize],ky.mg[Koma::U1 as usize],ky.mg[Koma::S1 as usize],ky.mg[Koma::H1 as usize],
            teme, teban, same
        )
    }




    /**
     * 表示
     */
    pub fn kaku_number_board(&self, sn:&Sengo, km:&Koma)->String{

        let nb = match *sn {
            Sengo::Owari    => { &self.kiki_su_by_km[km_to_num(&km)] },
            _               => { &self.kiki_su_by_sn[sn_to_num(&sn)] },
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
        for kms in KMS_ARRAY.iter() {
            g_write(&format!( "{} ", kms ));
            self.hyoji_kmugoki_dir( &kms );
            g_writeln("");//改行
        }
    }
    pub fn hyoji_kmugoki_dir(&self, kms:&KmSyurui ){
        for kmdir in KM_UGOKI.back[kms_to_num(kms)].iter() {
            match *kmdir {
                KmDir::Owari => break,
                _ => g_write(&format!( "{},", kmdir))
            }
        }
    }


    #[allow(dead_code)]
    pub fn remake_visions(&mut self) {
        for sn in SN_ARRAY.iter() {
            // 全部忘れる☆（＾～＾）
            self.vision_tree_by_sn[sn_to_num(sn)].clear();
        }
    }
}
