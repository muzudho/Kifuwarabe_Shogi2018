/**
 * USIプロトコル Rustフレームワーク
 */
use consoles::asserts::*;
use teigi::conv::*;
use teigi::constants::*;
use std::fmt;
use teigi::shogi_syugo::*;
use memory::uchu::*;
use tusin::usi::*;

use UCHU_WRAP;

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
            use teigi::shogi_syugo::KmSyurui;
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
        write!(f, "Movement({}{}{}{})", self.source, self.destination, self.promotion, self.drop)
    }
}

/**
 * 指し手読取
 * 例: 7g7f
 *
 * 読み取った指し手は、棋譜に入れる。
 * 現在の手目のところに入れ、手目のカウントアップも行う。
 */
pub fn read_sasite(
    uchu_w: &mut Uchu,
    line: &String,
    mut starts: &mut usize,
    len: usize
)->bool{
    // 構文解析。
    let umov : UsiMovement = parse_movement(&line, &mut starts, len);

    // 読取成否、移動元。
    use tusin::usi::PieceType;
    match umov.drop {
        PieceType::Space=> {
            if umov.source_file == -1 {
                // 読取失敗時、または 指し手読取終了時にここを通るぜ☆（＾～＾）
                return false;
            }
            uchu_w.set_sasite_src(suji_dan_to_ms(umov.source_file, umov.source_rank));
        },
        _=> {
            // 打のとき。
            uchu_w.set_sasite_src(0);
        },
    }
    // 打。
    uchu_w.set_sasite_drop(pt_to_kms(&umov.drop));
    // 移動先。
    uchu_w.set_sasite_dst(suji_dan_to_ms(umov.destination_file, umov.destination_rank));
    // 成り。
    uchu_w.set_sasite_pro(umov.promotion);

    // 手目。
    uchu_w.teme+=1;
    true
 }

/**
 * position コマンド 盤上部分のみ 読取
 */
 pub fn read_banjo(line:&String, starts:&mut usize, len:usize){

    // 盤部
    let ban = parse_banjo(&line, starts, len);

    // コピー
    for file in SUJI_1..SUJI_10 {
        for rank in DAN_1..DAN_10 {
            UCHU_WRAP.write().unwrap().set_ky0_ban_km(
                file,rank,pc_to_km(&ban[file_rank_to_cell(file,rank)])
            );
        }
    }

    // 初期局面ハッシュを作り直す
    let ky_hash = UCHU_WRAP.write().unwrap().create_ky0_hash();
    UCHU_WRAP.write().unwrap().set_ky0_hash( ky_hash );
}

 /**
  * position コマンド読取
  */
 pub fn read_position(line:&String){

    let mut starts = 0;

    // 全体の長さ
    let len = line.chars().count();

    // 局面をクリアー。手目も 0 に戻します。
    UCHU_WRAP.write().unwrap().clear_ky01();

    if 16<(len-starts) && &line[starts..(starts+17)]=="position startpos"{
        // 'position startpos' を読み飛ばし
        starts += 17;
        // 別途用意した平手初期局面文字列を読取
        let mut local_starts = 0;

        read_banjo( &STARTPOS.to_string(), &mut local_starts, STARTPOS_LN);

        if 0<(len-starts) && &line[starts..(starts+1)]==" "{
            // ' ' を読み飛ばした。
            starts += 1;
        }
    }else if 13<(len-starts) && &line[starts..(starts+14)]=="position sfen "{
        starts += 14; // 'position sfen ' を読み飛ばし
        read_banjo( line, &mut starts, len);

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

                    UCHU_WRAP.write().unwrap().set_ky0_mg(km, maisu);
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
    UCHU_WRAP.write().unwrap().copy_ky0_to_ky1();

    // 指し手を全部読んでいくぜ☆（＾～＾）手目のカウントも増えていくぜ☆（＾～＾）
    while read_sasite(&mut* UCHU_WRAP.write().unwrap(), line, &mut starts, len) {
        // 手目を戻す
        UCHU_WRAP.write().unwrap().teme -= 1;
        // 入っている指し手の通り指すぜ☆（＾～＾）
        let teme = UCHU_WRAP.read().unwrap().teme;
        let ss = UCHU_WRAP.read().unwrap().kifu[ teme ];
        UCHU_WRAP.write().unwrap().do_ss( &ss );

        // 現局面表示
        //let s1 = &UCHU_WRAP.read().unwrap().kaku_ky( &KyNums::Current );
        //g_writeln( &s1 );
    }
}

pub fn kms_to_pt (kms: &KmSyurui) -> PieceType {
    use teigi::shogi_syugo::KmSyurui::*;
    use tusin::usi::PieceType;
    match *kms{
        R => PieceType::K,
        K => PieceType::R,
        Z => PieceType::B,
        I => PieceType::G,
        N => PieceType::S,
        U => PieceType::N,
        S => PieceType::L,
        H => PieceType::P,
        PK => PieceType::PR,
        PZ => PieceType::PB,
        PN => PieceType::PS,
        PU => PieceType::PN,
        PS => PieceType::PL,
        PH => PieceType::PP,
        Kara => PieceType::Space,
        Owari => PieceType::Num,
    }
}

pub fn pt_to_kms (pt: &PieceType) -> KmSyurui {
    use tusin::usi::PieceType::*;
    use teigi::shogi_syugo::KmSyurui;
    match *pt{
        K => KmSyurui::R,
        R => KmSyurui::K,
        B => KmSyurui::Z,
        G => KmSyurui::I,
        S => KmSyurui::N,
        N => KmSyurui::U,
        L => KmSyurui::S,
        P => KmSyurui::H,
        PR => KmSyurui::PK,
        PB => KmSyurui::PZ,
        PS => KmSyurui::PN,
        PN => KmSyurui::PU,
        PL => KmSyurui::PS,
        PP => KmSyurui::PH,
        Space => KmSyurui::Kara,
        Num => KmSyurui::Owari,
    }
}



pub fn km_to_pc (km: &Koma) -> Piece {
    use teigi::shogi_syugo::Koma::*;
    use tusin::usi::Piece;
    match *km{
        R0 => Piece::K0,
        K0 => Piece::R0,
        Z0 => Piece::B0,
        I0 => Piece::G0,
        N0 => Piece::S0,
        U0 => Piece::N0,
        S0 => Piece::L0,
        H0 => Piece::P0,
        PK0 => Piece::PR0,
        PZ0 => Piece::PB0,
        PN0 => Piece::PS0,
        PU0 => Piece::PN0,
        PS0 => Piece::PL0,
        PH0 => Piece::PP0,
        R1 => Piece::K1,
        K1 => Piece::R1,
        Z1 => Piece::B1,
        I1 => Piece::G1,
        N1 => Piece::S1,
        U1 => Piece::N1,
        S1 => Piece::L1,
        H1 => Piece::P1,
        PK1 => Piece::PR1,
        PZ1 => Piece::PB1,
        PN1 => Piece::PS1,
        PU1 => Piece::PN1,
        PS1 => Piece::PL1,
        PH1 => Piece::PP1,
        Kara => Piece::Space,
        Owari => Piece::Num,
    }
}

pub fn pc_to_km (pc: &Piece) -> Koma {
    use tusin::usi::Piece::*;
    use teigi::shogi_syugo::Koma;
    match *pc{
        K0 => Koma::R0,
        R0 => Koma::K0,
        B0 => Koma::Z0,
        G0 => Koma::I0,
        S0 => Koma::N0,
        N0 => Koma::U0,
        L0 => Koma::S0,
        P0 => Koma::H0,
        PR0 => Koma::PK0,
        PB0 => Koma::PZ0,
        PS0 => Koma::PN0,
        PN0 => Koma::PU0,
        PL0 => Koma::PS0,
        PP0 => Koma::PH0,
        K1 => Koma::R1,
        R1 => Koma::K1,
        B1 => Koma::Z1,
        G1 => Koma::I1,
        S1 => Koma::N1,
        N1 => Koma::U1,
        L1 => Koma::S1,
        P1 => Koma::H1,
        PR1 => Koma::PK1,
        PB1 => Koma::PZ1,
        PS1 => Koma::PN1,
        PN1 => Koma::PU1,
        PL1 => Koma::PS1,
        PP1 => Koma::PH1,
        Space => Koma::Kara,
        Num => Koma::Owari,
    }
}



pub fn usi_to_movement(mv: &UsiMovement) -> Movement {
    let source : umasu = match mv.drop {
        PieceType::Space => suji_dan_to_ms(mv.source_file, mv.source_rank),
        _ => 0,
    };

    let drop : KmSyurui = match mv.drop {
        PieceType::Space => KmSyurui::Kara,
        _ => pt_to_kms(&mv.drop),
    };

    Movement {
        source : source,
        destination : suji_dan_to_ms(mv.destination_file, mv.destination_rank),
        promotion : mv.promotion,
        drop : drop,
    }
}

pub fn movement_to_usi(mv: &Movement) -> UsiMovement {
    let (src_file, src_rank, drop) = match mv.drop {
        KmSyurui::Kara => {
            let (src_file, src_rank) = ms_to_suji_dan(mv.source);
            (src_file, src_rank, PieceType::Space)
        },
        _ => (-1, -1, PieceType::Space),
    };

    let (dst_file, dst_rank) = ms_to_suji_dan(mv.destination);

    UsiMovement{
        source_file : src_file,
        source_rank : src_rank,
        drop : drop,
        destination_file : dst_file,
        destination_rank : dst_rank,
        promotion : mv.promotion,
    }
}