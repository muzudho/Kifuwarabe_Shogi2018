extern crate rand;

use kifuwarabe_shell::Response;

use std::collections::HashSet;
use config::*;
use consoles;
use consoles::unit_test::*;
use consoles::visuals::dumps::*;
use consoles::visuals::title::*;
use memory::uchu::*;
use rand::Rng;
use thinks;
use thinks::think::*;
use syazo::sasite_seisei::*;
use teigi::constants::*;
use teigi::conv::*;
use teigi::shogi_syugo::*;
use tusin;
use tusin::usi::*;

use UCHU_WRAP;




pub fn do_len_zero(__line: &String, _starts:&mut usize, _len:usize, _response:&mut Response){
    // 書込許可モードで、ロック。
    let mut uchu_w = UCHU_WRAP.write().unwrap();
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

/**
 * 駒の動きの確認。
 */
pub fn do_kmugokidir(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.read().unwrap();

    // 駒の動きの移動元として有りえる方角
    let kms = thinks::randommove::rnd_kms();
    g_writeln(&format!("{}のムーブ元", &kms));
    uchu_r.hyoji_kmugoki_dir( kms );
    g_writeln("");//改行
}

/**
 * USIプロトコル参照。
 */
pub fn do_usinewgame(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    // 書込許可モードで、ロック。
    let mut uchu_w = UCHU_WRAP.write().unwrap();
    
    uchu_w.clear_ky01();
}

/**
 * USIプロトコル参照。
 */
pub fn do_position(line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    // positionコマンドの読取を丸投げ
    tusin::usi::read_position(&line);
}

/**
 * USIプロトコル参照。
 */
pub fn do_isready(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    g_writeln("readyok");
}

/**
 * 駒の動き確認用。
 */
pub fn do_kmugoki(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.read().unwrap();

    // 駒の動きを出力
    uchu_r.hyoji_kmugoki();
}

/**
 * 平手初期局面にする。
 */
pub fn do_hirate(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    tusin::usi::read_position(&KY1.to_string());
}

/**
 * 利き数表示。
 */
pub fn do_kikisu(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    consoles::commands::cmd_kikisu();
}

/**
 * 駒種類をランダムで出す。
 */
pub fn do_rndkms(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    let kms = thinks::randommove::rnd_kms();
    g_writeln( &format!("乱駒種類={}", &kms) );
}

/**
 * 合法手を確認する。
 */
pub fn do_sasite(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    // FIXME 合法手とは限らない
    let mut ss_potential_hashset = HashSet::new();
    insert_potential_move(&mut ss_potential_hashset );
    g_writeln("----指し手生成 ここから----");
    hyoji_ss_hashset( &ss_potential_hashset );
    g_writeln("----指し手生成 ここまで----");
}

/**
 * マスをランダムで返す。
 */
pub fn do_rndms(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    let ms = thinks::randommove::rnd_ms();
    g_writeln( &format!( "乱升={}", ms) );
}

/**
 * convのテスト。
 */
pub fn do_teigi_conv(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
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

/**
 * 局面ハッシュ表示。
 */
pub fn do_hash(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.read().unwrap();

    let s = uchu_r.kaku_ky_hash();
    g_writeln( &s );
}

/**
 * 棋譜表示。
 */
pub fn do_kifu(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.read().unwrap();

    let s = uchu_r.kaku_kifu();
    g_writeln( &s );
}

/**
 * 終了。
 */
pub fn do_quit(_line: &String, _starts:&mut usize, _len:usize, response:&mut Response){
    response.quits = true;
}

/**
 * 乱数の試し確認。
 */
pub fn do_rand(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    let secret_number = rand::thread_rng().gen_range(1, 101);//1~100
    g_writeln( &format!( "乱数={}", secret_number ) );
}

/**
 * 同一局面回数調べ。
 */
pub fn do_same(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.read().unwrap();

    let count = uchu_r.count_same_ky();
    g_writeln( &format!("同一局面調べ count={}", count));
}

/**
 * いろいろな動作テストをしたいときに汎用的に使う。
 */
pub fn do_test(line: &String, starts:&mut usize, len:usize, _response:&mut Response) {
    g_writeln( &format!("test starts={} len={}", starts, len));
    test( &line, starts, len);
}

/**
 * 指した手を１手戻す。
 */
pub fn do_undo(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    // 書込許可モードで、ロック。
    let mut uchu_w = UCHU_WRAP.write().unwrap();

    if !uchu_w.undo_ss() {
        g_writeln( &format!("teme={} を、これより戻せません", uchu_w.teme ) );
    }
}

/**
 * 指し手を入れる。
 */
pub fn do_do(line: &String, starts:&mut usize, len:usize, _response:&mut Response) {
    // 書込許可モードで、ロック。
    let mut uchu_w = UCHU_WRAP.write().unwrap();
    // コマンド読取。棋譜に追加され、手目も増える
    if read_sasite(&mut* uchu_w, &line, starts, len) {
        // 手目を戻す
        uchu_w.teme -= 1;
        // 入っている指し手の通り指すぜ☆（＾～＾）
        let teme = uchu_w.teme;
        let ss = uchu_w.kifu[ teme ];
        uchu_w.do_ss( &ss );
    }
}

/**
 * 初期局面表示。
 */
pub fn do_ky0(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.read().unwrap();

    let s = uchu_r.kaku_ky( &KyNums::Start );
    g_writeln( &s );
}

/**
 * USIプロトコル参照。
 */
pub fn do_usi(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    g_writeln( &format!("id name {}", ENGINE_NAME) );
    g_writeln( &format!("id author {}", ENGINE_AUTHOR) );
    g_writeln("usiok");
}

/**
 * 思考を開始する。bestmoveコマンドを返却する。
 */
pub fn do_go(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    // go btime 40000 wtime 50000 binc 10000 winc 10000
    let bestmove = think();
    // 例： bestmove 7g7f
    g_writeln(&format!("bestmove {}", bestmove));
}

/**
 * 現局面表示。
 */
pub fn do_ky(_line: &String, _starts:&mut usize, _len:usize, _response:&mut Response) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.read().unwrap();

    let s = uchu_r.kaku_ky( &KyNums::Current );
    g_writeln( &s );            
}
