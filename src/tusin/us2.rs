use kifuwarabe_usi::*;
use memory::uchu::*;
use models::movement::*;
use teigi::constants::*;
//use teigi::conv::*;
use teigi::shogi_syugo::*;
use tusin::us_conv::*;
use UCHU_WRAP;

/*
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
*/
/*
type MovementCallback = fn(
    source_file:i8,
    source_rank:i8,
    destination_file:i8,
    destination_rank:i8,
    drop:PieceType,
    promotion:bool
    ) -> bool;
*/
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



        let ban;
        if 16<(len-starts) && &line[starts..(starts+17)]=="position startpos"{
            // 'position startpos' を読み飛ばし
            starts += 17;
            // 別途用意した平手初期局面文字列を読取
            let mut local_starts = 0;

            // position コマンド 盤上部分のみ 読取
            ban = parse_banjo(&STARTPOS.to_string(), &mut local_starts, STARTPOS_LN);

            if 0<(len-starts) && &line[starts..(starts+1)]==" "{
                // ' ' を読み飛ばした。
                starts += 1;
            }
        }else if 13<(len-starts) && &line[starts..(starts+14)]=="position sfen "{
            starts += 14; // 'position sfen ' を読み飛ばし

            // position コマンド 盤上部分のみ 読取
            ban = parse_banjo(&line, &mut starts, len);

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

            // 持ち駒数。持ち駒に使える、成らずの駒の部分だけ使用。
            // 増減させたいので、u8 ではなく i8。
            let mut hand_count_arr = [0; HAND_PIECE_ARRAY_LN];

            // 持ち駒の読取
            if 0<(len-starts) && &line[starts..(starts+1)]=="-"{
                starts += 1;
            } else {
                'mg:loop{
                    if 0<(len-starts){
                        // 持ち駒の枚数。
                        let mut count = 1;
                        match &line[starts..(starts+1)]{
                            "1"=>{
                                // 1枚のときは数字は付かないので、10～18 と確定☆
                                match &line[starts..(starts+1)]{
                                    "0"=>{count=10; starts+=2;},
                                    "1"=>{count=11; starts+=2;},
                                    "2"=>{count=12; starts+=2;},
                                    "3"=>{count=13; starts+=2;},
                                    "4"=>{count=14; starts+=2;},
                                    "5"=>{count=15; starts+=2;},
                                    "6"=>{count=16; starts+=2;},
                                    "7"=>{count=17; starts+=2;},
                                    "8"=>{count=18; starts+=2;},
                                    _ => { g_writeln(&format!("持駒部(0) '{}' だった。", &line[starts..(starts+2)])); return;},
                                }
                            },
                            "2"=>{count=2; starts+=1;},
                            "3"=>{count=3; starts+=1;},
                            "4"=>{count=4; starts+=1;},
                            "5"=>{count=5; starts+=1;},
                            "6"=>{count=6; starts+=1;},
                            "7"=>{count=7; starts+=1;},
                            "8"=>{count=8; starts+=1;},
                            "9"=>{count=9; starts+=1;},
                            _ => {},// 駒の名前か、エラーなら次へ
                        }

                        use kifuwarabe_usi::Piece::*;
                        let piece : Piece;
                        match &line[starts..(starts+1)]{
                            "R"=>{ piece=R0; starts+=1; },
                            "B"=>{ piece=B0; starts+=1; },
                            "G"=>{ piece=G0; starts+=1; },
                            "S"=>{ piece=S0; starts+=1; },
                            "N"=>{ piece=N0; starts+=1; },
                            "L"=>{ piece=L0; starts+=1; },
                            "P"=>{ piece=P0; starts+=1; },
                            "r"=>{ piece=R1; starts+=1; },
                            "b"=>{ piece=B1; starts+=1; },
                            "g"=>{ piece=G1; starts+=1; },
                            "s"=>{ piece=S1; starts+=1; },
                            "n"=>{ piece=N1; starts+=1; },
                            "l"=>{ piece=L1; starts+=1; },
                            "p"=>{ piece=P1; starts+=1; },
                            _ => { break 'mg; }, // 持駒部 正常終了
                        }

                        hand_count_arr[hand_piece_to_num(piece)] = count;
                    }//if
                }//loop

                // コピー
                let mut i=0;
                for item in HAND_PIECE_ARRAY.iter() {
                    let km = pc_to_km(item);
                    UCHU_WRAP.try_write().unwrap().set_ky0_mg(km, hand_count_arr[i]);
                    i+=1;
                }

            }//else

            if 2<(len-starts) && &line[starts..(starts+3)]==" 1 "{
                starts += 3;
            }
        }else{
            panic!("'position startpos' でも、'position sfen ' でも始まらなかった。");
        }


        // 盤面コピー
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
}
