extern crate rand;

// デバッグ出力。
const VERBOSE : bool = false;

use config::*;
use consoles;
use consoles::unit_test::*;
use consoles::visuals::dumps::*;
use consoles::visuals::title::*;
use ENGINE_SETTINGS_WRAP;
use GAME_RECORD_WRAP;
use INI_POSITION_WRAP;
use kifuwarabe_position::*;
use kifuwarabe_shell::*;
use kifuwarabe_usi::*;
use memory::uchu::*;
use misc::movement::*;
use rand::Rng;
use std::collections::HashSet;
use syazo::sasite_seisei::*;
use thinks;
use thinks::think::*;
use teigi::constants::*;
use teigi::shogi_syugo::*;
use tusin::us_conv::*;
use UCHU_WRAP;



/*****
 * D *
 *****/

/// 指し手を入れる。
pub fn do_do(line: &Commandline, caret:&mut Caret) {

    // コマンド読取。棋譜に追加され、手目も増える
    let (successful, umov) = parse_movement(&line.contents, &mut caret.starts, line.len);
    let movement = usi_to_movement(successful, &umov);

    // グローバル変数に内容をセット。
    {
        // 書込許可モードで、ロック。
        let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();
        game_record.set_movement(movement);
    }

    if successful {
        // 入っている指し手の通り指すぜ☆（＾～＾）
        make_movement2(&movement, |&_cap|{});
    }
}


/*****
 * G *
 *****/

/// 思考を開始する。bestmoveコマンドを返却する。
pub fn do_go(_line: &Commandline, _caret:&mut Caret) {
    // go btime 40000 wtime 50000 binc 10000 winc 10000
    let bestmove = think();
    // 例： bestmove 7g7f
    g_writeln(&format!("bestmove {}", movement_to_usi(&bestmove)));
}

/*****
 * H *
 *****/

/// 局面ハッシュ表示。
pub fn do_hash(_line: &Commandline, _caret:&mut Caret) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    let s = uchu_r.kaku_ky_hash();
    g_writeln( &s );
}

/// 平手初期局面にする。
pub fn do_hirate(_line: &Commandline, _caret:&mut Caret) {
    // 局面をクリアー。手目も 0 に戻します。
    UCHU_WRAP.try_write().unwrap().clear_ky01();

    parse_position(&KY1.to_string(),
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
            // 盤面コピー
            for file in SUJI_1..SUJI_10 {
                for rank in DAN_1..DAN_10 {
                    UCHU_WRAP.try_write().unwrap().set_ky0_ban_km(
                        file,rank,pc_to_km(&ban[file_rank_to_cell(file,rank)])
                    );
                }
            }

            // 初期局面ハッシュを作り直す
            let ky_hash = create_ky0_hash();

            // グローバル変数に内容をセット。
            {
                let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();
                game_record.set_ky0_hash( ky_hash );
            }

            {
                let mut uchu_w = UCHU_WRAP.try_write().unwrap();
                // 初期局面を、現局面にコピーします
                uchu_w.copy_ky0_to_ky1();            
            }
        },
        |successful, usi_movement|
        {
            let movement = usi_to_movement(successful, &usi_movement);

            {
                let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();
                game_record.set_movement(movement);
            }

            if successful {
                // 入っている指し手の通り指すぜ☆（＾～＾）
                make_movement2(&movement, |&_cap|{});
            }
        }
    );
}

/*****
 * I *
 *****/

/// USIプロトコル参照。
pub fn do_isready(_line: &Commandline, _caret:&mut Caret) {
    g_writeln("readyok");
}

/*****
 * K *
 *****/

/// 棋譜表示。
pub fn do_kifu(_line: &Commandline, _caret:&mut Caret) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    let s = uchu_r.kaku_kifu();
    g_writeln( &s );
}

/// 利き数表示。
pub fn do_kikisu(_line: &Commandline, _caret:&mut Caret) {
    consoles::commands::cmd_kikisu();
}

/// 駒の動きの確認。
pub fn do_kmugokidir(_line: &Commandline, _caret:&mut Caret) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    // 駒の動きの移動元として有りえる方角
    let kms = thinks::randommove::rnd_kms();
    g_writeln(&format!("{}のムーブ元", &kms));
    uchu_r.hyoji_kmugoki_dir( kms );
    g_writeln("");//改行
}

/// 駒の動き確認用。
pub fn do_kmugoki(_line: &Commandline, _caret:&mut Caret) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    // 駒の動きを出力
    uchu_r.hyoji_kmugoki();
}

/// 初期局面表示。
pub fn do_ky0(_line: &Commandline, _caret:&mut Caret) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    let s = uchu_r.kaku_ky( &KyNums::Start );
    g_writeln( &s );
}

/// 現局面表示。
pub fn do_ky(_line: &Commandline, _caret:&mut Caret) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    let s = uchu_r.kaku_ky( &KyNums::Current );
    g_writeln( &s );            
}


/*****
 * O *
 *****/

pub fn do_other(_line: &Commandline, _caret:&mut Caret){
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
        // 局面表示
        let s = &uchu_w.kaku_ky( &KyNums::Current );
        g_writeln( &s );
    }
}



/*****
 * P *
 *****/

/// USIプロトコル参照。
pub fn do_position(line: &Commandline, caret:&mut Caret) {
    // 局面をクリアー。手目も 0 に戻します。
    UCHU_WRAP.try_write().unwrap().clear_ky01();

    // positionコマンド読取。
    parse_position(&line.contents,
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
            for file in SUJI_1..SUJI_10 {
                for rank in DAN_1..DAN_10 {
                    UCHU_WRAP.try_write().unwrap().set_ky0_ban_km(
                        file,rank,pc_to_km(&ban[file_rank_to_cell(file,rank)])
                    );
                }
            }

            // 初期局面ハッシュを作り直す
            let ky_hash = create_ky0_hash();

            // グローバル変数に内容をセット。
            {
                let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();
                game_record.set_ky0_hash( ky_hash );
            }

            {
                // 初期局面を、現局面にコピーします
                let mut uchu_w = UCHU_WRAP.try_write().unwrap();
                uchu_w.copy_ky0_to_ky1();            
            }
        },
        // 指し手読取。
        |successful, usi_movement|{
            let movement = usi_to_movement(successful, &usi_movement);

            // 棋譜に書き込み。
            {
                let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();
                game_record.set_movement(movement);
            }

            if successful {
                // 指し手を指すぜ☆（＾～＾）
                make_movement2(&movement, |&_cap|{});
            }
        }
    );

    caret.done_line = true;
}


/*****
 * Q *
 *****/

/// 終了。
pub fn do_quit(_line: &Commandline, caret:&mut Caret){
    caret.quits = true;
}

/*****
 * R *
 *****/

/// 乱数の試し確認。
pub fn do_rand(_line: &Commandline, _caret:&mut Caret) {
    let secret_number = rand::thread_rng().gen_range(1, 101);//1~100
    g_writeln( &format!( "乱数={}", secret_number ) );
}

/// 駒種類をランダムで出す。
pub fn do_rndkms(_line: &Commandline, _caret:&mut Caret) {
    let kms = thinks::randommove::rnd_kms();
    g_writeln( &format!("乱駒種類={}", &kms) );
}

/// マスをランダムで返す。
pub fn do_rndms(_line: &Commandline, _caret:&mut Caret) {
    let ms = thinks::randommove::rnd_ms();
    g_writeln( &format!( "乱升={}", ms) );
}

/*****
 * S *
 *****/

/// 同一局面回数調べ。
pub fn do_same(_line: &Commandline, _caret:&mut Caret) {
    g_writeln( &format!("同一局面調べ count={}", count_same_ky()));
}


/// 合法手を確認する。
pub fn do_sasite(_line: &Commandline, _caret:&mut Caret) {
    // FIXME 合法手とは限らない
    let mut ss_potential_hashset = HashSet::new();
    insert_potential_move(&mut ss_potential_hashset );
    g_writeln("----指し手生成 ここから----");
    hyoji_ss_hashset( &ss_potential_hashset );
    g_writeln("----指し手生成 ここまで----");
}




/// USI
pub fn do_setoption(_line: &Commandline, caret:&mut Caret) {
    if VERBOSE { println!("Setoption begin."); }
    caret.next = "ND_setoption_name";
    caret.set_line_end_controller(do_setoption_lineend);
    if VERBOSE { println!("Setoption end."); }
}
pub fn do_setoption_name(_line: &Commandline, caret:&mut Caret) {
    if VERBOSE { println!("Name."); }
    caret.next = "ND_setoption_namevar";
}
pub fn do_setoption_namevar(_line: &Commandline, caret:&mut Caret) {
    let name = &caret.groups[0];
    if VERBOSE { println!("Namevar begin. [{}]", name); }

    let mut eng = ENGINE_SETTINGS_WRAP.try_write().unwrap();
    eng.buffer_name = name.to_string();
    caret.next = "ND_setoption_value";
    if VERBOSE { println!("Namevar end."); }
}
pub fn do_setoption_value(_line: &Commandline, caret:&mut Caret) {
    if VERBOSE { println!("Value."); }
    caret.next = "ND_setoption_valuevar";
}
pub fn do_setoption_valuevar(_line: &Commandline, caret:&mut Caret) {
    let value = &caret.groups[0];
    if VERBOSE { println!("Valuevar begin. [{}]", value); }

    let mut eng = ENGINE_SETTINGS_WRAP.try_write().unwrap();
    eng.buffer_value = value.to_string();
    caret.done_line = true;
    if VERBOSE { println!("Valuevar end."); }
}
pub fn do_setoption_lineend(_line: &Commandline, _caret:&mut Caret) {
    if VERBOSE { println!("Lineend begin."); }
    let mut eng = ENGINE_SETTINGS_WRAP.try_write().unwrap();
    eng.flush();
    if VERBOSE { println!("Lineend end."); }
}




/*****
 * T *
 *****/

/// convのテスト。
pub fn do_teigi_conv(_line: &Commandline, _caret:&mut Caret) {
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
pub fn do_test(line: &Commandline, caret:&mut Caret) {
    g_writeln( &format!("test starts={} len={}", caret.starts, line.len));
    test( &line.contents, &mut caret.starts, line.len);
}


/*****
 * U *
 *****/

/// 指した手を１手戻す。
pub fn do_undo(_line: &Commandline, _caret:&mut Caret) {
    if !unmake_movement2(|&_cap|{}) {
        let teme = GAME_RECORD_WRAP.try_read().unwrap().teme;
        g_writeln( &format!("teme={} を、これより戻せません", teme));
    }
}

/// USIプロトコル参照。
pub fn do_usinewgame(_line: &Commandline, _caret:&mut Caret) {
    // 書込許可モードで、ロック。
    let mut uchu_w = UCHU_WRAP.try_write().unwrap();
    
    uchu_w.clear_ky01();
}

/// USIプロトコル参照。
pub fn do_usi(_line: &Commandline, _caret:&mut Caret) {
    g_writeln( &format!("id name {}", ENGINE_NAME) );
    g_writeln( &format!("id author {}", ENGINE_AUTHOR) );
    g_writeln("option name depth type spin default 1 min 1 max 3");
    g_writeln("usiok");
}













