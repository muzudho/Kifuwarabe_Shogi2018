use kifuwarabe_usi::*;
use models::movement::*;
use teigi::constants::*;
use tusin::us_conv::*;

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
        line: &String,
        callback0: fn([i8; HAND_PIECE_ARRAY_LN]),
        callback1: fn([Piece;100]),
        callback2: fn(bool, Movement)
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
            let hand_count_arr : [i8; HAND_PIECE_ARRAY_LN] = parse_hand_piece(line, &mut starts, len);
            callback0(hand_count_arr);


            if 2<(len-starts) && &line[starts..(starts+3)]==" 1 "{
                starts += 3;
            }
        }else{
            panic!("'position startpos' でも、'position sfen ' でも始まらなかった。");
        }

        // 盤を返す。
        callback1(ban);

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
                callback2(successful, usi_to_movement(&umov));
            } else {
                // 読取終了時(失敗時)、最後に投了を送るぜ☆（＾～＾）
                callback2(successful, Movement::new());
                break;
            }
        } // loop
    }
}
