extern crate lazy_static;
extern crate rand;

// デバッグ出力。
const VERBOSE : bool = false;

use config::*;
use consoles;
use consoles::unit_test::*;
use consoles::visuals::dumps::*;
use consoles::visuals::title::*;
use CUR_POSITION_WRAP;
use ENGINE_SETTINGS_WRAP;
use GAME_RECORD_WRAP;
use INI_POSITION_WRAP;
use kifuwarabe_position::*;
use kifuwarabe_shell::*;
use kifuwarabe_usi::*;
use memory::uchu::*;
use rand::Rng;
use searcher_impl::*;
use std::collections::HashSet;
use syazo::sasite_seisei::*;
use thinks;
use thinks::think::*;
use teigi::constants::*;
// use teigi::shogi_syugo::*;
use tusin::us_conv::*;
use UCHU_WRAP;

// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    static ref SHELL: RwLock<ShellVariable> = RwLock::new(ShellVariable::new());
}

pub struct ShellVariable {
    player_milliseconds_array : [i32; 2],
}
impl ShellVariable {
    pub fn new() -> ShellVariable {
        ShellVariable {
            player_milliseconds_array : [0, 0]
        }
    }
}

/*****
 * D *
 *****/

/// 指し手を入れる。
pub fn do_do(request: &Request, response:&mut Response) {

    // 任意の構造体を作成する。
    let mut searcher = Searcher::new();
    {
        searcher.ini_position = INI_POSITION_WRAP.try_read().unwrap().clone();
        searcher.cur_position = CUR_POSITION_WRAP.try_read().unwrap().clone();
        searcher.game_record = GAME_RECORD_WRAP.try_read().unwrap().clone();
    }

    // コマンド読取。棋譜に追加され、手目も増える
    let (successful, umov) = parse_movement(&request.line, &mut response.caret, request.line_len);
    let movement = usi_to_movement(successful, &umov);

    searcher.game_record.set_movement(movement);

    if successful {
        // 入っている指し手の通り指すぜ☆（＾～＾）
        makemove(&mut searcher, movement.to_hash());

        // クローンからオリジナルへ還元する。
        {
            INI_POSITION_WRAP.try_write().unwrap().set_all(&searcher.ini_position);
            CUR_POSITION_WRAP.try_write().unwrap().set_all(&searcher.cur_position);
            GAME_RECORD_WRAP.try_write().unwrap().set_all(&searcher.game_record);
        }
    }
}


/*****
 * G *
 *****/

/// 思考を開始する。bestmoveコマンドを返却する。
///
/// ### 例。
/// go btime 60000 wtime 50000 byoyomi 10000
pub fn do_go(_request: &Request, response:&mut Response) {
    {
        // 指定しなければ無制限。
        SHELL.try_write().unwrap().player_milliseconds_array[SN_SEN] = <i32>::max_value();
        SHELL.try_write().unwrap().player_milliseconds_array[SN_GO] = <i32>::max_value();
    }

    // 行終了時に実行されるコールバック関数を１つ設定できる。
    response.set_linebreak_controller(do_go_linebreak);

    response.next = "ND_go_btime";
}

pub fn do_go_btime(_request: &Request, response:&mut Response) {
    response.next = "ND_go_btimevar";
}

pub fn do_go_btimevar(_request: &Request, response:&mut Response) {
    let word = &response.groups[0];
    let num: i32 = word.parse().unwrap();
    {
        SHELL.try_write().unwrap().player_milliseconds_array[0] = num;
    }
    response.next = "ND_go_wtime";
}

pub fn do_go_wtime(_request: &Request, response:&mut Response) {
    response.next = "ND_go_wtimevar";
}

pub fn do_go_wtimevar(_request: &Request, response:&mut Response) {
    let word = &response.groups[0];
    let num: i32 = word.parse().unwrap();
    {
        SHELL.try_write().unwrap().player_milliseconds_array[1] = num;
    }
    response.next = "ND_go_binc";
}

pub fn do_go_binc(_request: &Request, response:&mut Response) {
    response.next = "ND_go_bincvar";
}

pub fn do_go_bincvar(_request: &Request, response:&mut Response) {
    let word = &response.groups[0];
    let num: i32 = word.parse().unwrap();
    {
        SHELL.try_write().unwrap().player_milliseconds_array[0] += num;
    }
    response.next = "ND_go_winc";
}

pub fn do_go_winc(_request: &Request, response:&mut Response) {
    response.next = "ND_go_wincvar";
}

pub fn do_go_wincvar(_request: &Request, response:&mut Response) {
    let word = &response.groups[0];
    let num: i32 = word.parse().unwrap();
    {
        SHELL.try_write().unwrap().player_milliseconds_array[1] += num;
    }
}

pub fn do_go_linebreak(_request: &Request, _response:&mut Response) {
    // 自分の手番
    let turn_num;
    {
        turn_num = sn_to_num( &GAME_RECORD_WRAP.try_read().unwrap().get_teban(&Jiai::Ji));
    }

    // 自分の持ち時間。
    let milliseconds;
    {
        milliseconds = SHELL.try_read().unwrap().player_milliseconds_array[turn_num];
    }

    // 思考する。
    let bestmove = think(milliseconds);

    // 例： bestmove 7g7f
    g_writeln(&format!("bestmove {}", movement_to_usi(&bestmove)));
}

/*****
 * H *
 *****/

/// 局面ハッシュ表示。
pub fn do_hash(_request: &Request, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    let s = uchu_r.kaku_ky_hash();
    g_writeln( &s );
}

/// 平手初期局面にする。
pub fn do_hirate(_request: &Request, _response:&mut Response) {
    // 局面をクリアー。手目も 0 に戻します。
    UCHU_WRAP.try_write().unwrap().clear_ky01();

    let mut searcher = Searcher::new();

    parse_position(
        &mut searcher,
        &KY1.to_string(),
        |hand_count_arr : [i8; HAND_PIECE_ARRAY_LN]|
        {
            // 持ち駒数コピー。
            let mut i=0;
            for item in HAND_PIECE_ARRAY.iter() {
                let km = pc_to_km(item);

                INI_POSITION_WRAP.try_write().unwrap().set_mg(km, hand_count_arr[i]);
                i+=1;
            }
        },
        |ban: [Piece;100]|
        {
            // 局面のクローンを作成。
            let mut position0;
            {
                position0 = INI_POSITION_WRAP.try_write().unwrap().clone();
            }

            // 盤面コピー
            for file in SUJI_1..SUJI_10 {
                for rank in DAN_1..DAN_10 {
                    position0.set_km_by_ms(suji_dan_to_ms(file, rank), pc_to_km(&ban[file_rank_to_cell(file,rank)]));
                }
            }

            {
                INI_POSITION_WRAP.try_write().unwrap().set_all(&position0);
            }

            // 初期局面ハッシュを作り直す
            let ky_hash;
            {

                ky_hash = GAME_RECORD_WRAP.try_read().unwrap().create_ky0_hash(&position0);
            }

            // グローバル変数に内容をセット。
            {
                let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();
                game_record.set_ky0_hash( ky_hash );
            }

            // 初期局面を、現局面にコピーします。
            {
                CUR_POSITION_WRAP.try_write().unwrap().set_all(&INI_POSITION_WRAP.try_read().unwrap());
            }
        },
        |successful, usi_movement, mut searcher|
        {
            let movement = usi_to_movement(successful, &usi_movement);

            // グローバル変数を使う。
            {
                let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();
                game_record.set_movement(movement);
            }

            if successful {
                // 入っている指し手の通り指すぜ☆（＾～＾）
                makemove(&mut searcher, movement.to_hash());

                // 局面のクローンで上書き。
                {
                    CUR_POSITION_WRAP.try_write().unwrap().set_all(&searcher.cur_position);
                }
            }
        }
    );
}

/*****
 * I *
 *****/

/// USIプロトコル参照。
pub fn do_isready(_request: &Request, _response:&mut Response) {
    g_writeln("readyok");
}

/*****
 * K *
 *****/

/// 棋譜表示。
pub fn do_kifu(_request: &Request, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    let s = uchu_r.kaku_kifu();
    g_writeln( &s );
}

/// 利き数表示。
pub fn do_kikisu(_request: &Request, _response:&mut Response) {
    consoles::commands::cmd_kikisu();
}

/// 駒の動きの確認。
pub fn do_kmugokidir(_request: &Request, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    // 駒の動きの移動元として有りえる方角
    let kms = thinks::randommove::rnd_kms();
    g_writeln(&format!("{}のムーブ元", &kms));
    uchu_r.hyoji_kmugoki_dir( kms );
    g_writeln("");//改行
}

/// 駒の動き確認用。
pub fn do_kmugoki(_request: &Request, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    // 駒の動きを出力
    uchu_r.hyoji_kmugoki();
}

/// 初期局面表示。
pub fn do_ky0(_request: &Request, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    // 局面のクローンを作成。
    let position0;
    {
        position0 = INI_POSITION_WRAP.try_read().unwrap().clone();
    }

    let s = uchu_r.kaku_ky(&position0, true);
    g_writeln( &s );
}

/// 現局面表示。
pub fn do_ky(_request: &Request, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    // 局面のクローンを作成。
    let position1;
    {
        position1 = CUR_POSITION_WRAP.try_read().unwrap().clone();
    }

    let s = uchu_r.kaku_ky(&position1, true);
    g_writeln( &s );            
}


/*****
 * O *
 *****/

pub fn do_other(_request: &Request, _response:&mut Response){
    // 書込許可モードで、ロック。
    let mut uchu_w = UCHU_WRAP.try_write().unwrap();
    if !&uchu_w.dialogue_mode {
        // 空打ち１回目なら、対話モードへ☆（＾～＾）
        uchu_w.dialogue_mode = true;
        // タイトル表示
        // １画面は２５行だが、最後の２行は開けておかないと、
        // カーソルが２行分場所を取るんだぜ☆（＾～＾）
        hyoji_title();
    }else{
        // 局面のクローンを作成。
        let position1;
        {
            position1 = CUR_POSITION_WRAP.try_read().unwrap().clone();
        }

        // 局面表示
        let s = &uchu_w.kaku_ky(&position1, true);
        g_writeln( &s );
    }
}



/*****
 * P *
 *****/

/// USIプロトコル参照。
pub fn do_position(request: &Request, response:&mut Response) {
    // 局面をクリアー。手目も 0 に戻します。
    UCHU_WRAP.try_write().unwrap().clear_ky01();

    let mut searcher = Searcher::new();

    // positionコマンド読取。
    parse_position(
        &mut searcher,
        &request.line,
        // 持ち駒数読取。
        |hand_count_arr : [i8; HAND_PIECE_ARRAY_LN]|{
            let mut i=0;
            for item in HAND_PIECE_ARRAY.iter() {
                let km = pc_to_km(item);

                INI_POSITION_WRAP.try_write().unwrap().set_mg(km, hand_count_arr[i]);
                i+=1;
            }
        },
        // 盤面読取。
        |ban: [Piece;100]|{
            // 局面のクローンを作成。
            let mut position0;
            {
                position0 = INI_POSITION_WRAP.try_write().unwrap().clone();
            }

            for file in SUJI_1..SUJI_10 {
                for rank in DAN_1..DAN_10 {
                    position0.set_km_by_ms(suji_dan_to_ms(file, rank), pc_to_km(&ban[file_rank_to_cell(file,rank)]));
                }
            }

            {
                INI_POSITION_WRAP.try_write().unwrap().set_all(&position0);
            }

            // 初期局面ハッシュを作り直す
            let ky_hash;
            {

                ky_hash = GAME_RECORD_WRAP.try_read().unwrap().create_ky0_hash(&position0);
            }

            // グローバル変数に内容をセット。
            {
                let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();
                game_record.set_ky0_hash( ky_hash );
            }

            // 初期局面を、現局面にコピーします。
            {
                CUR_POSITION_WRAP.try_write().unwrap().set_all(&INI_POSITION_WRAP.try_read().unwrap());
            }
        },
        // 指し手読取。
        |successful, usi_movement, mut searcher|{
            let movement = usi_to_movement(successful, &usi_movement);

            // 棋譜に書き込み。
            {
                let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();
                game_record.set_movement(movement);
            }

            if successful {
                // 指し手を指すぜ☆（＾～＾）
                makemove(&mut searcher, movement.to_hash());

                // 局面のクローンで上書き。
                {
                    CUR_POSITION_WRAP.try_write().unwrap().set_all(&searcher.cur_position);
                }
            }
        }
    );

    response.done_line = true;
}


/*****
 * Q *
 *****/

/// 終了。
pub fn do_quit(_request: &Request, response:&mut Response){
    response.quits = true;
}

/*****
 * R *
 *****/

/// 乱数の試し確認。
pub fn do_rand(_request: &Request, _response:&mut Response) {
    let secret_number = rand::thread_rng().gen_range(1, 101);//1~100
    g_writeln( &format!( "乱数={}", secret_number ) );
}

/// 駒種類をランダムで出す。
pub fn do_rndkms(_request: &Request, _response:&mut Response) {
    let kms = thinks::randommove::rnd_kms();
    g_writeln( &format!("乱駒種類={}", &kms) );
}

/// マスをランダムで返す。
pub fn do_rndms(_request: &Request, _response:&mut Response) {
    let ms = thinks::randommove::rnd_ms();
    g_writeln( &format!( "乱升={}", ms) );
}

/*****
 * S *
 *****/

/// 同一局面回数調べ。
pub fn do_same(_request: &Request, _response:&mut Response) {
    g_writeln( &format!("同一局面調べ count={}", GAME_RECORD_WRAP.try_read().unwrap().count_same_ky()));
}


/// 合法手を確認する。
pub fn do_sasite(_request: &Request, _response:&mut Response) {
    // 任意の構造体を作成する。
    let mut searcher = Searcher::new();
    {
        searcher.ini_position = INI_POSITION_WRAP.try_read().unwrap().clone();
        searcher.cur_position = CUR_POSITION_WRAP.try_read().unwrap().clone();
        searcher.game_record = GAME_RECORD_WRAP.try_read().unwrap().clone();
    }

    // FIXME 合法手とは限らない
    let mut ss_potential_hashset = HashSet::new();
    insert_potential_move(&searcher, &mut ss_potential_hashset);
    g_writeln("----指し手生成 ここから----");
    hyoji_ss_hashset( &ss_potential_hashset );
    g_writeln("----指し手生成 ここまで----");
}




/// USI
pub fn do_setoption(_request: &Request, response:&mut Response) {
    if VERBOSE { println!("Setoption begin."); }
    response.next = "ND_setoption_name";
    response.set_linebreak_controller(do_setoption_lineend);
    if VERBOSE { println!("Setoption end."); }
}
pub fn do_setoption_name(_request: &Request, response:&mut Response) {
    if VERBOSE { println!("Name."); }
    response.next = "ND_setoption_namevar";
}
pub fn do_setoption_namevar(_request: &Request, response:&mut Response) {
    let name = &response.groups[0];
    if VERBOSE { println!("Namevar begin. [{}]", name); }

    let mut eng = ENGINE_SETTINGS_WRAP.try_write().unwrap();
    eng.buffer_name = name.to_string();
    response.next = "ND_setoption_value";
    if VERBOSE { println!("Namevar end."); }
}
pub fn do_setoption_value(_request: &Request, response:&mut Response) {
    if VERBOSE { println!("Value."); }
    response.next = "ND_setoption_valuevar";
}
pub fn do_setoption_valuevar(_request: &Request, response:&mut Response) {
    let value = &response.groups[0];
    if VERBOSE { println!("Valuevar begin. [{}]", value); }

    let mut eng = ENGINE_SETTINGS_WRAP.try_write().unwrap();
    eng.buffer_value = value.to_string();
    response.done_line = true;
    if VERBOSE { println!("Valuevar end."); }
}
pub fn do_setoption_lineend(_request: &Request, _response:&mut Response) {
    if VERBOSE { println!("Lineend begin."); }
    let mut eng = ENGINE_SETTINGS_WRAP.try_write().unwrap();
    eng.flush();
    if VERBOSE { println!("Lineend end."); }
}




/*****
 * T *
 *****/

/// convのテスト。
pub fn do_teigi_conv(_request: &Request, _response:&mut Response) {
    for ms in 11..19 {
        for hash in 0..10 {
            let next = push_ms_to_hash(hash,ms);
            let (hash_orig,ms_orig) = pop_ms_from_hash(next);
            g_writeln( &format!("push_ms_to_hash(0b{:4b},0b{:5b})=0b{:11b} pop_ms_from_hash(...)=(0b{:4b},0b{:5b})"
                ,hash
                ,ms
                ,next
                ,hash_orig
                ,ms_orig
            ));
        }
    }
}

/// いろいろな動作テストをしたいときに汎用的に使う。
pub fn do_test(request: &Request, response:&mut Response) {
    // 任意の構造体を作成する。
    let mut searcher = Searcher::new();
    {
        searcher.ini_position = INI_POSITION_WRAP.try_read().unwrap().clone();
        searcher.cur_position = CUR_POSITION_WRAP.try_read().unwrap().clone();
        searcher.game_record = GAME_RECORD_WRAP.try_read().unwrap().clone();
    }

    g_writeln( &format!("test caret={} len={}", request.caret, request.line_len));
    test(&searcher, &request.line, &mut response.caret, request.line_len);
}


/*****
 * U *
 *****/

/// 指した手を１手戻す。
pub fn do_undo(_request: &Request, _response:&mut Response) {

    // 任意の構造体を作成する。
    let mut searcher = Searcher::new();
    {
        searcher.ini_position = INI_POSITION_WRAP.try_read().unwrap().clone();
        searcher.cur_position = CUR_POSITION_WRAP.try_read().unwrap().clone();
        searcher.game_record = GAME_RECORD_WRAP.try_read().unwrap().clone();
    }

    let (successful, _cap_kms) = unmakemove(&mut searcher);

    if successful {
        // クローンからオリジナルへ還元する。
        {
            INI_POSITION_WRAP.try_write().unwrap().set_all(&searcher.ini_position);
            CUR_POSITION_WRAP.try_write().unwrap().set_all(&searcher.cur_position);
            GAME_RECORD_WRAP.try_write().unwrap().set_all(&searcher.game_record);
        }

    } else {
        let teme = searcher.game_record.teme;
        g_writeln( &format!("teme={} を、これより戻せません", teme));

    }
}

/// USIプロトコル参照。
pub fn do_usinewgame(_request: &Request, _response:&mut Response) {
    // 書込許可モードで、ロック。
    let mut uchu_w = UCHU_WRAP.try_write().unwrap();
    
    uchu_w.clear_ky01();
}

/// USIプロトコル参照。
pub fn do_usi(_request: &Request, _response:&mut Response) {
    g_writeln( &format!("id name {}", ENGINE_NAME) );
    g_writeln( &format!("id author {}", ENGINE_AUTHOR) );
    g_writeln("option name depth type spin default 1 min 1 max 999");
    g_writeln("usiok");
}













