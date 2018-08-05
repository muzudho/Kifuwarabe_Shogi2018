use kifuwarabe_usi::*;
//use memory::uchu::*;
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
        line:&String,
        callback0: fn([Piece;100]),
        callback1: fn(bool, Movement)
        ){

        let mut starts = 0;

        // 全体の長さ
        let len = line.chars().count();



        let ban : [Piece;100];
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

            // 先後も読み飛ばす。
            if 0<(len-starts) && &line[starts..(starts+1)]=="w"{
                starts += 1;
            }else if 0<(len-starts) && &line[starts..(starts+1)]=="b"{
                starts += 1;
            }

            if 0<(len-starts) && &line[starts..(starts+1)]==" "{
                starts += 1;
            }

            // 持ち駒数。増減させたいので、u8 ではなく i8。
            let hand_count_arr = parse_hand_piece(line, &mut starts, len);
            // 持ち駒数コピー。
            let mut i=0;
            for item in HAND_PIECE_ARRAY.iter() {
                let km = pc_to_km(item);
                UCHU_WRAP.try_write().unwrap().set_ky0_mg(km, hand_count_arr[i]);
                i+=1;
            }


            if 2<(len-starts) && &line[starts..(starts+3)]==" 1 "{
                starts += 3;
            }
        }else{
            panic!("'position startpos' でも、'position sfen ' でも始まらなかった。");
        }

        // 盤を返す。
        callback0(ban);

        if 4<(len-starts) && &line[starts..(starts+5)]=="moves"{
            starts += 5;
        }

        if 0<(len-starts) && &line[starts..(starts+1)]==" "{
            starts += 1;
        }

        // 指し手を1つずつ返すぜ☆（＾～＾）
        loop {
            let (successful, umov) = parse_movement(line, &mut starts, len);
            if successful {
                callback1(successful, usi_to_movement(&umov));
            } else {
                // 読取終了時(失敗時)、最後に投了を送るぜ☆（＾～＾）
                callback1(successful, Movement::new());
                break;
            }
        } // loop
    }
}
