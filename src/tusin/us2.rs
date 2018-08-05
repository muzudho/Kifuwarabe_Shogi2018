use kifuwarabe_usi::*;
use memory::uchu::*;
use models::movement::*;
use teigi::constants::*;
use teigi::conv::*;
use teigi::shogi_syugo::*;
use tusin::us_conv::*;
use UCHU_WRAP;

/// 関数を返す関数
fn create_movement_function() -> fn(
    source_file:i8,
    source_rank:i8,
    destination_file:i8,
    destination_rank:i8,
    drop:PieceType,
    promotion:bool
    ) -> bool
{
    |source_file,
    source_rank,
    destination_file,
    destination_rank,
    drop:PieceType,
    promotion|
    {
        // 読取成否、移動元。
        use kifuwarabe_usi::PieceType;
        let src_ms;
        match drop {
            PieceType::Space=> {
                if source_file == -1 {
                    // 読取失敗時、または 指し手読取終了時にここを通るぜ☆（＾～＾）
                    return false;
                }
                src_ms = suji_dan_to_ms(source_file, source_rank);
            },
            _=> {
                // 打のとき。
                src_ms = 0;
            },
        }

        // グローバル変数に内容をセット。
        {
            // デッドロックしてしまう。
            let mut uchu_w = UCHU_WRAP.try_write().unwrap();
            uchu_w.set_sasite_src(src_ms);
            uchu_w.set_sasite_drop(pt_to_kms(&drop));
            uchu_w.set_sasite_dst(suji_dan_to_ms(destination_file, destination_rank));
            uchu_w.set_sasite_pro(promotion);
            uchu_w.teme+=1;
        }

        true
    }
}


type MovementCallback = fn(
    source_file:i8,
    source_rank:i8,
    destination_file:i8,
    destination_rank:i8,
    drop:PieceType,
    promotion:bool
    ) -> bool;
pub struct PositionParser{
    //movement_callback: MovementCallback,
}
impl PositionParser{
    pub fn new(
        //movement_callback: MovementCallback
        )->PositionParser{
        PositionParser{
            //movement_callback: movement_callback
        }
    }

    /// position コマンド読取
    pub fn read_position(
        &self,
        line:&String){

        let mut starts = 0;

        // 全体の長さ
        let len = line.chars().count();

        if 16<(len-starts) && &line[starts..(starts+17)]=="position startpos"{
            // 'position startpos' を読み飛ばし
            starts += 17;
            // 別途用意した平手初期局面文字列を読取
            let mut local_starts = 0;

            &self.read_banjo( &STARTPOS.to_string(), &mut local_starts, STARTPOS_LN);

            if 0<(len-starts) && &line[starts..(starts+1)]==" "{
                // ' ' を読み飛ばした。
                starts += 1;
            }
        }else if 13<(len-starts) && &line[starts..(starts+14)]=="position sfen "{
            starts += 14; // 'position sfen ' を読み飛ばし
            &self.read_banjo( line, &mut starts, len);

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

                        UCHU_WRAP.try_write().unwrap().set_ky0_mg(km, maisu);
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

        {
            // 初期局面を、現局面にコピーします
            UCHU_WRAP.try_write().unwrap().copy_ky0_to_ky1();
        }

        // 指し手を全部読んでいくぜ☆（＾～＾）手目のカウントも増えていくぜ☆（＾～＾）
        loop {
            let (successful, umov) = parse_movement(line, &mut starts, len);
            let mov;
            if successful {
                mov = usi_to_movement(&umov);
            } else {
                // 読取失敗時、または 指し手読取終了時に successfulの外を通るぜ☆（＾～＾）
                mov = Movement::new();
            }

            // グローバル変数に内容をセット。
            {
                let mut uchu_w = UCHU_WRAP.try_write().unwrap();
                uchu_w.set_sasite_src(mov.source);
                uchu_w.set_sasite_drop(mov.drop);
                uchu_w.set_sasite_dst(mov.destination);
                uchu_w.set_sasite_pro(mov.promotion);
            
                if successful {
                    // 入っている指し手の通り指すぜ☆（＾～＾）
                    let teme = uchu_w.teme;
                    let ss = uchu_w.kifu[ teme ];
                    uchu_w.do_ss( &ss );
                } else {
                    // 読取失敗時、または 指し手読取終了時に successfulの外を通るぜ☆（＾～＾）
                    break;
                }
            }

            // 現局面表示
            //let s1 = &UCHU_WRAP.try_read().unwrap().kaku_ky( &KyNums::Current );
            //g_writeln( &s1 );
        }
    }

    /**
     * 
     */
    pub fn read_banjo(
        &self,
        line:&String, starts:&mut usize, len:usize){

        // position コマンド 盤上部分のみ 読取
        let ban = parse_banjo(&line, starts, len);

        // コピー
        for file in SUJI_1..SUJI_10 {
            for rank in DAN_1..DAN_10 {
                UCHU_WRAP.try_write().unwrap().set_ky0_ban_km(
                    file,rank,pc_to_km(&ban[file_rank_to_cell(file,rank)])
                );
            }
        }

        // 初期局面ハッシュを作り直す
        let ky_hash = UCHU_WRAP.try_write().unwrap().create_ky0_hash();
        UCHU_WRAP.try_write().unwrap().set_ky0_hash( ky_hash );
    }
}
