/**
 * USIプロトコル
 */
use consoles::asserts::*;
use teigi::conv::*;
use teigi::constants::*;
use std::fmt;
use teigi::shogi_syugo::*;
use jotai::uchu::*;

/**
 * 指し手
 * 棋譜にも使うので、取った駒の情報を記憶しておくんだぜ☆（＾～＾）
 * しかし、なんで英語が並んでるんだぜ☆（＾～＾）
 */
#[derive(Copy,Clone)]
pub struct Sasite{
    // 移動元升。打った場合は 0。
    pub src : umasu,
    // 移動先升。これが 0 なら投了とするぜ☆（＾～＾）
    pub dst : umasu,
    // 移動後に成るなら真
    pub pro : bool,
    // 打の場合、打った駒種類
    pub drop : KmSyurui,
}
impl Sasite{
    pub fn new()->Sasite{
        Sasite{
            src:0,
            dst:0,
            pro:false,
            drop:KmSyurui::Kara,
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self){
        self.src = 0;
        self.dst = 0;
        self.pro = false;
        self.drop= KmSyurui::Kara;
    }
    pub fn to_hash(&self)->u64{
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_kms_to_hash(hash, &self.drop);
        hash = push_bool_to_hash(hash, self.pro);
        hash = push_ms_to_hash(hash, self.dst);
        push_ms_to_hash(hash, self.src)
    }
    pub fn from_hash(hash:u64)->Sasite{
        // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
        let (hash,src) = pop_ms_from_hash(hash);
        let (hash,dst) = pop_ms_from_hash(hash);
        let (hash,pro) = pop_bool_from_hash(hash);
        let (_hash,drop) = pop_kms_from_hash(hash);
        Sasite{
            src:src,
            dst:dst,
            pro:pro,
            drop:drop,
        }
    }

    /**
     * 考えた結果、指し手が考え付いていれば真。
     */
    pub fn exists(&self)->bool{
        self.dst != MASU_0
    }
}
impl fmt::Display for Sasite{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {

        // 手が何もない、ぐらいの意味だが、
        // その手を指す場合、投了表示
        if !self.exists() { return write!(f,"resign"); }

        // 投了を弾いたあと、診断☆（＾～＾）
        assert_banjo_ms(self.dst,"Ｓasite Ｄisplay");
        let (dx,dy) = ms_to_suji_dan(self.dst);

        if self.src==SS_SRC_DA {
            use teigi::shogi_syugo::KmSyurui::*;
            write!(f, "{}*{}{}{}",
                match self.drop {
                    K => { "R" },
                    Z => { "B" },
                    I => { "G" },
                    N => { "S" },
                    U => { "N" },
                    S => { "L" },
                    H => { "P" },
                    _  => { "?" },
                },
                dx,
                num_to_lower_case(dy),
                if self.pro {"+"}else{""}
            )
        } else {
            let (sx,sy) = if self.src==MASU_0 {
                // エラー・データも表示したい
                 (0,0)
            } else {
                assert_banjo_ms(self.src,"Ｓasite Ｄisplay＜その２＞");
                ms_to_suji_dan(self.src)
            };
            write!(f, "{}{}{}{}{}",
                sx,
                num_to_lower_case(sy),
                dx,
                num_to_lower_case(dy),
                if self.pro {"+"}else{""}
            )
        }
    }
}
impl fmt::Debug for Sasite{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sasite({}{}{}{})", self.src, self.dst, self.pro, self.drop)
    }
}

/**
 * 指し手読取
 * 例: 7g7f
 *
 * 読み取った指し手は、棋譜に入れる。
 * 現在の手目のところに入れ、手目のカウントアップも行う。
 */
 pub fn read_sasite( line:&String, starts:&mut usize, len:usize, uchu:&mut Uchu )->bool{

    // 4文字か5文字あるはず。
    if (len-*starts)<4{
        // 指し手読取終了時にここを通るぜ☆（＾～＾）
        // 残り４文字もない。
        return false;
    }

    // 1文字目と2文字目
    match &line[*starts..(*starts+1)]{
        // 1文字目が駒だったら打。2文字目は必ず「*」なはずなので読み飛ばす。
        "R" => { *starts+=2; uchu.set_sasite_src(0); uchu.set_sasite_drop(KmSyurui::K); },
        "B" => { *starts+=2; uchu.set_sasite_src(0); uchu.set_sasite_drop(KmSyurui::Z); },
        "G" => { *starts+=2; uchu.set_sasite_src(0); uchu.set_sasite_drop(KmSyurui::I); },
        "S" => { *starts+=2; uchu.set_sasite_src(0); uchu.set_sasite_drop(KmSyurui::N); },
        "N" => { *starts+=2; uchu.set_sasite_src(0); uchu.set_sasite_drop(KmSyurui::U); },
        "L" => { *starts+=2; uchu.set_sasite_src(0); uchu.set_sasite_drop(KmSyurui::S); },
        "P" => { *starts+=2; uchu.set_sasite_src(0); uchu.set_sasite_drop(KmSyurui::H); },
        _ => {
            // 残りは「筋の数字」、「段のアルファベット」のはず。
            let suji;
            let dan;
            match &line[*starts..(*starts+1)]{
                "1" => { suji=1; *starts+=1; },
                "2" => { suji=2; *starts+=1; },
                "3" => { suji=3; *starts+=1; },
                "4" => { suji=4; *starts+=1; },
                "5" => { suji=5; *starts+=1; },
                "6" => { suji=6; *starts+=1; },
                "7" => { suji=7; *starts+=1; },
                "8" => { suji=8; *starts+=1; },
                "9" => { suji=9; *starts+=1; },
                _ => {g_writeln(&format!("(1) '{}' だった。", &line[*starts..(*starts+1)])); return false;},
            }

            match &line[*starts..(*starts+1)]{
                "a" => { dan=1; *starts+=1; },
                "b" => { dan=2; *starts+=1; },
                "c" => { dan=3; *starts+=1; },
                "d" => { dan=4; *starts+=1; },
                "e" => { dan=5; *starts+=1; },
                "f" => { dan=6; *starts+=1; },
                "g" => { dan=7; *starts+=1; },
                "h" => { dan=8; *starts+=1; },
                "i" => { dan=9; *starts+=1; },
                _ => {g_writeln(&format!("(2) '{}' だった。", &line[*starts..(*starts+1)])); return false;},
            }

            uchu.set_sasite_src(suji_dan_to_ms(suji, dan));
            uchu.set_sasite_drop(KmSyurui::Kara);
        },
    }

    // 残りは「筋の数字」、「段のアルファベット」のはず。
    let suji;
    let dan;

    // 3文字目
    match &line[*starts..(*starts+1)]{
        "1" => { suji=1; *starts+=1; },
        "2" => { suji=2; *starts+=1; },
        "3" => { suji=3; *starts+=1; },
        "4" => { suji=4; *starts+=1; },
        "5" => { suji=5; *starts+=1; },
        "6" => { suji=6; *starts+=1; },
        "7" => { suji=7; *starts+=1; },
        "8" => { suji=8; *starts+=1; },
        "9" => { suji=9; *starts+=1; },
        _ => {g_writeln(&format!("(3) '{}' だった。", &line[*starts..(*starts+1)])); return false;},
    }
    
    // 4文字目
    match &line[*starts..(*starts+1)]{
        "a" => { dan=1; *starts+=1; },
        "b" => { dan=2; *starts+=1; },
        "c" => { dan=3; *starts+=1; },
        "d" => { dan=4; *starts+=1; },
        "e" => { dan=5; *starts+=1; },
        "f" => { dan=6; *starts+=1; },
        "g" => { dan=7; *starts+=1; },
        "h" => { dan=8; *starts+=1; },
        "i" => { dan=9; *starts+=1; },
        _ => {g_writeln(&format!("(4) '{}' だった。", &line[*starts..(*starts+1)])); return false;},
    }

    uchu.set_sasite_dst(suji_dan_to_ms(suji, dan));
    
    // 5文字に「+」があれば成り。
    if 0<(len-*starts) && &line[*starts..(*starts+1)]=="+" {
        uchu.set_sasite_pro(true);
        *starts+=1;
    } else {
        uchu.set_sasite_pro(false);        
    }

    // 続きにスペース「 」が１つあれば読み飛ばす
    if 0<(len-*starts) && &line[*starts..(*starts+1)]==" " {
        *starts+=1;
    }

    uchu.teme+=1;
    true
 }

/**
 * position コマンド 盤上部分のみ 読取
 */
 pub fn read_banjo( line:&String, starts:&mut usize, len:usize, uchu:&mut Uchu ){
    // 盤部
    let mut suji = SUJI_9;//９筋から右方向へ読取
    let mut dan = DAN_1;
    'ban: while 0<(len-*starts) {
        match &line[*starts..(*starts+1)]{
            "/" => { *starts+=1; suji=SUJI_9; dan+=1; },
            "1" => { *starts+=1;
                uchu.set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
            },
            "2" => { *starts+=1;
                uchu.set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                uchu.set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
            },
            "3" => { *starts+=1;
                uchu.set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                uchu.set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                uchu.set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
            },
            "4" => { *starts+=1;
                for _i_kara in 0..4{
                    uchu.set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                }
            },
            "5" => { *starts+=1;
                for _i_kara in 0..5{
                    uchu.set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                }
            },
            "6" => { *starts+=1;
                for _i_kara in 0..6{
                    uchu.set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                }
            },
            "7" => { *starts+=1;
                for _i_kara in 0..7{
                    uchu.set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                }
            },
            "8" => { *starts+=1;
                for _i_kara in 0..8{
                    uchu.set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                }
            },
            "9" => { *starts+=1;
                for _i_kara in 0..9{
                    uchu.set_ky0_ban_km(suji,dan,Koma::Kara); suji-=1;
                }
            },
            "K" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::R0); suji-=1; },
            "R" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::K0); suji-=1; },
            "B" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::Z0); suji-=1; },
            "G" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::I0); suji-=1; },
            "S" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::N0); suji-=1; },
            "N" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::U0); suji-=1; },
            "L" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::S0); suji-=1; },
            "P" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::H0); suji-=1; },
            "k" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::R1); suji-=1; },
            "r" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::K1); suji-=1; },
            "b" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::Z1); suji-=1; },
            "g" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::I1); suji-=1; },
            "s" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::N1); suji-=1; },
            "n" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::U1); suji-=1; },
            "l" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::S1); suji-=1; },
            "p" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::H1); suji-=1; },
            "+" => {
                *starts+=1;
                match &line[*starts..(*starts+1)]{
                    "R" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::PK0); suji-=1; },
                    "B" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::PZ0); suji-=1; },
                    "S" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::PN0); suji-=1; },
                    "N" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::PU0); suji-=1; },
                    "L" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::PS0); suji-=1; },
                    "P" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::PH0); suji-=1; },
                    "r" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::PK1); suji-=1; },
                    "b" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::PZ1); suji-=1; },
                    "s" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::PN1); suji-=1; },
                    "n" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::PU1); suji-=1; },
                    "l" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::PS1); suji-=1; },
                    "p" => { *starts+=1; uchu.set_ky0_ban_km(suji,dan,Koma::PH1); suji-=1; },
                    _ => { g_writeln(&format!("盤部(0) '{}' だった。", &line[*starts..(*starts+1)])); break 'ban;},
                }                    
            },
            _ => {break 'ban;}, // 盤部正常終了
        }
    }

    // 初期局面ハッシュを作り直す
    let ky_hash = uchu.create_ky0_hash();
    uchu.set_ky0_hash( ky_hash );
}

 /**
  * position コマンド読取
  */
 pub fn read_position( line:&String, uchu:&mut Uchu ){

    let mut starts = 0;

    // 全体の長さ
    let len = line.chars().count();

    // 局面をクリアー。手目も 0 に戻します。
    uchu.clear_ky01();

    if 16<(len-starts) && &line[starts..(starts+17)]=="position startpos"{
        // 'position startpos' を読み飛ばし
        starts += 17;
        // 別途用意した平手初期局面文字列を読取
        let mut local_starts = 0;
        read_banjo( &STARTPOS.to_string(), &mut local_starts, STARTPOS_LN, uchu);

        if 0<(len-starts) && &line[starts..(starts+1)]==" "{
            // ' ' を読み飛ばした。
            starts += 1;
        }
    }else if 13<(len-starts) && &line[starts..(starts+14)]=="position sfen "{
        starts += 14; // 'position sfen ' を読み飛ばし
        read_banjo( line, &mut starts, len, uchu);

        if 0<(len-starts) && &line[starts..(starts+1)]==" "{
            starts += 1;
        }

        if 0<(len-starts) && &line[starts..(starts+1)]=="w"{
            starts += 1;
        }else if 0<(len-starts) && &line[starts..(starts+1)]=="b"{
            starts += 1;
        }

        if 0<(len-starts) && &line[starts..(starts+1)]==" "{
            starts += 1;
        }

        // 持ち駒の読取
        if 0<(len-starts) && &line[starts..(starts+1)]=="-"{
            starts += 1;
        } else {
            'mg:loop{
                if 0<(len-starts){
                    let mut maisu = 1;
                    match &line[starts..(starts+1)]{
                        "1"=>{
                            // 1枚のときは数字は付かないので、10～18 と確定☆
                            match &line[starts..(starts+1)]{
                                "0"=>{maisu=10; starts+=2;},
                                "1"=>{maisu=11; starts+=2;},
                                "2"=>{maisu=12; starts+=2;},
                                "3"=>{maisu=13; starts+=2;},
                                "4"=>{maisu=14; starts+=2;},
                                "5"=>{maisu=15; starts+=2;},
                                "6"=>{maisu=16; starts+=2;},
                                "7"=>{maisu=17; starts+=2;},
                                "8"=>{maisu=18; starts+=2;},
                                _ => { g_writeln(&format!("持駒部(0) '{}' だった。", &line[starts..(starts+2)])); return;},
                            }
                        },
                        "2"=>{maisu=2; starts+=1;},
                        "3"=>{maisu=3; starts+=1;},
                        "4"=>{maisu=4; starts+=1;},
                        "5"=>{maisu=5; starts+=1;},
                        "6"=>{maisu=6; starts+=1;},
                        "7"=>{maisu=7; starts+=1;},
                        "8"=>{maisu=8; starts+=1;},
                        "9"=>{maisu=9; starts+=1;},
                        _ => {},// 駒の名前か、エラーなら次へ
                    }

                    use teigi::shogi_syugo::Koma::*;
                    let km : Koma;
                    match &line[starts..(starts+1)]{
                        "R"=>{ km=K0; starts+=1; },
                        "B"=>{ km=Z0; starts+=1; },
                        "G"=>{ km=I0; starts+=1; },
                        "S"=>{ km=N0; starts+=1; },
                        "N"=>{ km=U0; starts+=1; },
                        "L"=>{ km=S0; starts+=1; },
                        "P"=>{ km=H0; starts+=1; },
                        "r"=>{ km=K1; starts+=1; },
                        "b"=>{ km=Z1; starts+=1; },
                        "g"=>{ km=I1; starts+=1; },
                        "s"=>{ km=N1; starts+=1; },
                        "n"=>{ km=U1; starts+=1; },
                        "l"=>{ km=S1; starts+=1; },
                        "p"=>{ km=H1; starts+=1; },
                        _ => { break 'mg; }, // 持駒部 正常終了
                    }

                    uchu.set_ky0_mg(km, maisu);
                }//if
            }//loop
        }//else

        if 2<(len-starts) && &line[starts..(starts+3)]==" 1 "{
            starts += 3;
        }
    }else{
        g_writeln("'position startpos' でも、'position sfen ' でも始まらなかった。");
        return;
    }
        
    if 4<(len-starts) && &line[starts..(starts+5)]=="moves"{
        starts += 5;
    }

    if 0<(len-starts) && &line[starts..(starts+1)]==" "{
        starts += 1;
    }

    // 初期局面を、現局面にコピーします
    uchu.copy_ky0_to_ky1();

    // 指し手を全部読んでいくぜ☆（＾～＾）手目のカウントも増えていくぜ☆（＾～＾）
    while read_sasite( line, &mut starts, len, uchu) {
        // 手目を戻す
        uchu.teme -= 1;
        // 入っている指し手の通り指すぜ☆（＾～＾）
        let teme = uchu.teme;
        let ss = uchu.kifu[ teme ];
        uchu.do_ss( &ss );

        // 現局面表示
        //let s1 = &uchu.kaku_ky( &KyNums::Current );
        //g_writeln( &s1 );
    }
}