///
/// きふわらべ将棋2018
///
/// 外部クレートを利用しているので、cargo build でコンパイルすること。rustc main.rs ではコンパイルが成功しない。
/// 実行ファイルは target/debug/kifuwarabe_shogi2018.exe だぜ☆
///
extern crate rand;
#[macro_use]
extern crate lazy_static;


///
/// Rust言語の mod や ソース置き場の説明
///      「Rust のモジュールシステム」
///      https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
///
/// use したい モジュールは、最初に読み取られる　この main.rs ファイルに並べる

mod config;
mod consoles;
mod jotai;
mod kasetu;
mod meidai;
mod siko;
mod syazo;
mod teigi;
//mod teiri;
mod tusin;

use std::collections::HashSet;
use std::io;

use config::*;
use consoles::unit_test::*;
use consoles::visuals::dumps::*;
use consoles::visuals::title::*;
use rand::Rng;
use siko::think::*;
use syazo::sasite_seisei::*;
use teigi::constants::*;
use teigi::conv::*;
use teigi::shogi_syugo::*;
use tusin::usi::*;
use jotai::uchu::*;

type Callback = fn(uchu:&mut Uchu, len:usize, line: &String, starts:&mut usize);

fn do_len_zero(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("len==0");
    if !&uchu.dialogue_mode {
        // 空打ち１回目なら、対話モードへ☆（＾～＾）
        uchu.dialogue_mode = true;
        // タイトル表示
        // １画面は２５行だが、最後の２行は開けておかないと、
        // カーソルが２行分場所を取るんだぜ☆（＾～＾）
        hyoji_title();
    }else{
        // 局面表示
        let s = &uchu.kaku_ky( &KyNums::Current );
        g_writeln( &s );
    }
}
fn do_kmugokidir(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("9<len kmugokidir");
    // 駒の動きの移動元として有りえる方角
    let kms = siko::randommove::rnd_kms();
    g_writeln(&format!("{}のムーブ元", &kms));
    uchu.hyoji_kmugoki_dir( kms );
    g_writeln("");//改行
}
fn do_usinewgame(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    uchu.clear_ky01();
}
fn do_position(uchu:&mut Uchu, _len:usize, line: &String, _starts:&mut usize) {
    // positionコマンドの読取を丸投げ
    tusin::usi::read_position(&line, uchu);
}
fn do_isready(_uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("readyok");
}
fn do_kmugoki(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("6<len kmugoki");
    // 駒の動きを出力
    uchu.hyoji_kmugoki();
}
fn do_hirate(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    // 平手初期局面
    tusin::usi::read_position(&KY1.to_string(), uchu);
}
fn do_kikisu(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    // 利き数表示
    consoles::commands::cmd_kikisu( &uchu );
}
fn do_rndkms(_uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("5<len rndkms");
    // 乱駒種類
    let kms = siko::randommove::rnd_kms();
    g_writeln( &format!("乱駒種類={}", &kms) );
}
fn do_sasite(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    // FIXME 合法手とは限らない
    let mut ss_potential_hashset = HashSet::new();
    insert_potential_move( &uchu, &mut ss_potential_hashset );
    g_writeln("----指し手生成 ここから----");
    hyoji_ss_hashset( &ss_potential_hashset );
    g_writeln("----指し手生成 ここまで----");
}
fn do_rndms(_uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    // 乱升
    let ms = siko::randommove::rnd_ms();
    g_writeln( &format!( "乱升={}", ms) );
}
fn do_teigi_conv(_uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("teigi::convのテスト");

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
fn do_hash(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("局面ハッシュ表示");
    let s = uchu.kaku_ky_hash();
    g_writeln( &s );
}
fn do_kifu(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("棋譜表示");
    let s = uchu.kaku_kifu();
    g_writeln( &s );
}
fn do_rand(_uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("3<len rand");
    // 乱数の試し
    let secret_number = rand::thread_rng().gen_range(1, 101);//1~100
    g_writeln( &format!( "乱数={}", secret_number ) );
}
fn do_same(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    let count = uchu.count_same_ky();
    g_writeln( &format!("同一局面調べ count={}", count));
}
fn do_test(uchu:&mut Uchu, len:usize, line: &String, starts:&mut usize) {
    *starts += 4;
    // 続きにスペース「 」が１つあれば読み飛ばす
    if 0<(len-*starts) && &line[*starts..(*starts+1)]==" " {
        *starts+=1;
    }            
    // いろいろな動作テスト
    g_writeln( &format!("test starts={} len={}", starts, len));
    test( &line, starts, len, uchu);
    //g_writeln( &uchu.pop_command() );
}
fn do_undo(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    if !uchu.undo_ss() {
        g_writeln( &format!("teme={} を、これより戻せません", uchu.teme ) );
    }
}
fn do_do(uchu:&mut Uchu, len:usize, line: &String, starts:&mut usize) {
    *starts += 3;
    // コマンド読取。棋譜に追加され、手目も増える
    if read_sasite( &line, starts, len, uchu) {
        // 手目を戻す
        uchu.teme -= 1;
        // 入っている指し手の通り指すぜ☆（＾～＾）
        let teme = uchu.teme;
        let ss = uchu.kifu[ teme ];
        uchu.do_ss( &ss );
    }
}
fn do_ky0(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    // 初期局面表示
    let s = uchu.kaku_ky( &KyNums::Start );
    g_writeln( &s );
}
fn do_usi(_uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln( &format!("id name {}", ENGINE_NAME) );
    g_writeln( &format!("id author {}", ENGINE_AUTHOR) );
    g_writeln("usiok");
}
fn do_go(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    // 思考開始と、bestmoveコマンドの返却
    // go btime 40000 wtime 50000 binc 10000 winc 10000
    let bestmove = think( uchu );
    // 例： bestmove 7g7f
    g_writeln(&format!("bestmove {}", bestmove));
}
fn do_ky(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    // 現局面表示
    let s = &uchu.kaku_ky( &KyNums::Current );
    g_writeln( &s );            
}

fn main() {

    // 宇宙
    let mut uchu : Uchu = Uchu::new();
    uchu.big_bang();
    
    let fn_len_zero: Callback = do_len_zero;
    let fn_kmugokidir: Callback = do_kmugokidir;
    let fn_usinewgame: Callback = do_usinewgame;
    let fn_position: Callback = do_position;
    let fn_isready: Callback = do_isready;
    let fn_kmugoki: Callback = do_kmugoki;
    let fn_hirate: Callback = do_hirate;
    let fn_kikisu: Callback = do_kikisu;
    let fn_rndkms: Callback = do_rndkms;
    let fn_sasite: Callback = do_sasite;
    let fn_rndms: Callback = do_rndms;
    let fn_teigi_conv: Callback = do_teigi_conv;
    let fn_hash: Callback = do_hash;
    let fn_kifu: Callback = do_kifu;
    let fn_rand: Callback = do_rand;
    let fn_same: Callback = do_same;
    let fn_test: Callback = do_test;
    let fn_undo: Callback = do_undo;
    let fn_do: Callback = do_do;
    let fn_ky0: Callback = do_ky0;
    let fn_usi: Callback = do_usi;
    let fn_go: Callback = do_go;
    let fn_ky: Callback = do_ky;


    // [Ctrl]+[C] で強制終了
    loop{

        let mut line : String;
        if uchu.is_empty_command() {
            line = String::new();
        } else {
            // バッファーに溜まっていれば☆（＾～＾）
            line = uchu.pop_command();
            //g_writeln( &line );
        }

        // まず最初に、コマンドライン入力を待機しろだぜ☆（＾～＾）
        io::stdin().read_line(&mut line)
            .ok()// read_lineの返り値オブジェクトResult の okメソッド
            .expect("info Failed to read line");// OKで無かった場合のエラーメッセージ

        // 末尾の改行を除こうぜ☆（＾～＾）
        // trim すると空白も消えるぜ☆（＾～＾）
        let line : String = line.trim().parse().ok().expect("info Failed to parse");

        // 文字数を調べようぜ☆（＾～＾）
        let len = line.chars().count();
        let mut starts = 0;

        if len==0 {
            fn_len_zero(&mut uchu, len, &line, &mut starts);
        // 文字数の長いものからチェック
        }else if line.starts_with("kmugokidir") {
            fn_kmugokidir(&mut uchu, len, &line, &mut starts);
        }else if 9<len && &line[starts..10] == "usinewgame" {
            fn_usinewgame(&mut uchu, len, &line, &mut starts);
        }else if line.starts_with("position") {
            fn_position(&mut uchu, len, &line, &mut starts);
        }else if 6<len && &line[starts..7] == "isready" {
            fn_isready(&mut uchu, len, &line, &mut starts);
        }else if 6<len && &line[starts..7] == "kmugoki" {
            fn_kmugoki(&mut uchu, len, &line, &mut starts);
        }else if 5<len && &line[starts..6] == "hirate" {
            fn_hirate(&mut uchu, len, &line, &mut starts);
        }else if 5<len && &line[starts..6] == "kikisu" {
            fn_kikisu(&mut uchu, len, &line, &mut starts);
        }else if 5<len && &line[starts..6] == "rndkms" {
            fn_rndkms(&mut uchu, len, &line, &mut starts);
        }else if 5<len && &line[starts..6] == "sasite" {
            fn_sasite(&mut uchu, len, &line, &mut starts);
        }else if 4<len && &line[starts..5] == "rndms" {
            fn_rndms(&mut uchu, len, &line, &mut starts);
        }else if 3<len && &line[starts..4] == "teigi::conv" {
            fn_teigi_conv(&mut uchu, len, &line, &mut starts);            
        }else if 3<len && &line[starts..4] == "hash" {
            fn_hash(&mut uchu, len, &line, &mut starts);
        }else if 3<len && &line[starts..4] == "kifu" {
            fn_kifu(&mut uchu, len, &line, &mut starts);
        }else if 3<len && &line[starts..4] == "quit" {
            // ループを抜けて終了
            break;
        }else if 3<len && &line[starts..4] == "rand" {
            fn_rand(&mut uchu, len, &line, &mut starts);
        }else if 3<len && &line[starts..4] == "same" {
            fn_same(&mut uchu, len, &line, &mut starts);
        }else if 3<len && &line[starts..4] == "test" {
            fn_test(&mut uchu, len, &line, &mut starts);
        }else if 3<len && &line[starts..4] == "undo" {
            fn_undo(&mut uchu, len, &line, &mut starts);
        }else if 2<len && &line[starts..3] == "do " {
            fn_do(&mut uchu, len, &line, &mut starts);
        }else if 2<len && &line[starts..3] == "ky0" {
            fn_ky0(&mut uchu, len, &line, &mut starts);
        }else if 2<len && &line[starts..3] == "usi" {
            fn_usi(&mut uchu, len, &line, &mut starts);
        }else if 1<len && &line[starts..2] == "go" {
            fn_go(&mut uchu, len, &line, &mut starts);
        }else if 1<len && &line[starts..2] == "ky" {
            fn_ky(&mut uchu, len, &line, &mut starts);
        }
    }//loop
}
