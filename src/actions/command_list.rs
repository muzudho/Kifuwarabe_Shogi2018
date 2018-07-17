extern crate rand;

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


pub struct CommandList {
    pub action_len_zero: Command,
    pub command_array: [Command; 23],
}
impl CommandList{
    pub fn new()->CommandList{
        CommandList{
            action_len_zero: Command { keyword: "".to_string(), callback: do_len_zero1 },
            command_array: [
                Command { keyword: "kmugokidir".to_string(), callback: do_kmugokidir },
                Command { keyword: "usinewgame".to_string(), callback: do_usinewgame },
                Command { keyword: "position".to_string(), callback: do_position },
                Command { keyword: "isready".to_string(), callback: do_isready },
                Command { keyword: "kmugoki".to_string(), callback: do_kmugoki },
                Command { keyword: "hirate".to_string(), callback: do_hirate },
                Command { keyword: "kikisu".to_string(), callback: do_kikisu },
                Command { keyword: "rndkms".to_string(), callback: do_rndkms },
                Command { keyword: "sasite".to_string(), callback: do_sasite },
                Command { keyword: "rndms".to_string(), callback: do_rndms },
                Command { keyword: "teigi::conv".to_string(), callback: do_teigi_conv },
                Command { keyword: "hash".to_string(), callback: do_hash },
                Command { keyword: "kifu".to_string(), callback: do_kifu },
                Command { keyword: "quit".to_string(), callback: do_quit },
                Command { keyword: "rand".to_string(), callback: do_rand },
                Command { keyword: "same".to_string(), callback: do_same },
                Command { keyword: "test".to_string(), callback: do_test },
                Command { keyword: "undo".to_string(), callback: do_undo },
                Command { keyword: "do ".to_string(), callback: do_do },
                Command { keyword: "ky0".to_string(), callback: do_ky0 },
                Command { keyword: "usi".to_string(), callback: do_usi },
                Command { keyword: "go".to_string(), callback: do_go },
                Command { keyword: "ky".to_string(), callback: do_ky },
            ],        
        }
    }
}

type Callback = fn(uchu:&mut Uchu, len:usize, line: &String, starts:&mut usize);

/// [2016-12-10 Idiomatic callbacks in Rust](https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust)
pub struct Command {
    pub keyword: String,
    pub callback: Callback,
}
impl Command {

    pub fn is_matched(&self, len:usize, line: &String, starts:&usize) -> bool {
        return self.keyword.len()<=len && &line[*starts..self.keyword.len()] == self.keyword
    }

    pub fn move_caret_and_go(&self, uchu:&mut Uchu, len:usize, line: &String, starts:&mut usize) {
        *starts += self.keyword.len();
        // 続きにスペース「 」が１つあれば読み飛ばす
        if 0<(len-*starts) && &line[*starts..(*starts+1)]==" " {
            *starts+=1;
        }            

        (self.callback)(uchu, len, line, starts);
    }
}

/**
 * 何とも一致しなかったとき。
 */
pub fn do_len_zero1(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
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
pub fn do_len_zero2(_len:usize, _line: &String, _starts:&mut usize){
    g_writeln("len==0");
    if !&UCHU_WRAP.read().unwrap().dialogue_mode {
        // 空打ち１回目なら、対話モードへ☆（＾～＾）
        UCHU_WRAP.write().unwrap().dialogue_mode = true;
        // タイトル表示
        // １画面は２５行だが、最後の２行は開けておかないと、
        // カーソルが２行分場所を取るんだぜ☆（＾～＾）
        hyoji_title();
    }else{
        // 局面表示
        let s = &UCHU_WRAP.read().unwrap().kaku_ky( &KyNums::Current );
        g_writeln( &s );
    }
}

/**
 * 駒の動きの確認。
 */
pub fn do_kmugokidir(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("9<len kmugokidir");
    // 駒の動きの移動元として有りえる方角
    let kms = thinks::randommove::rnd_kms();
    g_writeln(&format!("{}のムーブ元", &kms));
    uchu.hyoji_kmugoki_dir( kms );
    g_writeln("");//改行
}

/**
 * USIプロトコル参照。
 */
pub fn do_usinewgame(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    uchu.clear_ky01();
}

/**
 * USIプロトコル参照。
 */
pub fn do_position(uchu:&mut Uchu, _len:usize, line: &String, _starts:&mut usize) {
    // positionコマンドの読取を丸投げ
    tusin::usi::read_position(&line, uchu);
}

/**
 * USIプロトコル参照。
 */
pub fn do_isready(_uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("readyok");
}

/**
 * 駒の動き確認用。
 */
pub fn do_kmugoki(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("6<len kmugoki");
    // 駒の動きを出力
    uchu.hyoji_kmugoki();
}

/**
 * 平手初期局面にする。
 */
pub fn do_hirate(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    tusin::usi::read_position(&KY1.to_string(), uchu);
}

/**
 * 利き数表示。
 */
pub fn do_kikisu(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    consoles::commands::cmd_kikisu( &uchu );
}

/**
 * 駒種類をランダムで出す。
 */
pub fn do_rndkms(_uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("5<len rndkms");
    let kms = thinks::randommove::rnd_kms();
    g_writeln( &format!("乱駒種類={}", &kms) );
}

/**
 * 合法手を確認する。
 */
pub fn do_sasite(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    // FIXME 合法手とは限らない
    let mut ss_potential_hashset = HashSet::new();
    insert_potential_move( &uchu, &mut ss_potential_hashset );
    g_writeln("----指し手生成 ここから----");
    hyoji_ss_hashset( &ss_potential_hashset );
    g_writeln("----指し手生成 ここまで----");
}

/**
 * マスをランダムで返す。
 */
pub fn do_rndms(_uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    let ms = thinks::randommove::rnd_ms();
    g_writeln( &format!( "乱升={}", ms) );
}

/**
 * convのテスト。
 */
pub fn do_teigi_conv(_uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
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

/**
 * 局面ハッシュ表示。
 */
pub fn do_hash(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("局面ハッシュ表示");
    let s = uchu.kaku_ky_hash();
    g_writeln( &s );
}

/**
 * 棋譜表示。
 */
pub fn do_kifu(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("棋譜表示");
    let s = uchu.kaku_kifu();
    g_writeln( &s );
}

/**
 * 終了。
 */
pub fn do_quit(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize){
    uchu.is_quit = true;
}

/**
 * 乱数の試し確認。
 */
pub fn do_rand(_uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln("3<len rand");
    let secret_number = rand::thread_rng().gen_range(1, 101);//1~100
    g_writeln( &format!( "乱数={}", secret_number ) );
}

/**
 * 同一局面回数調べ。
 */
pub fn do_same(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    let count = uchu.count_same_ky();
    g_writeln( &format!("同一局面調べ count={}", count));
}

/**
 * いろいろな動作テストをしたいときに汎用的に使う。
 */
pub fn do_test(uchu:&mut Uchu, len:usize, line: &String, starts:&mut usize) {
    g_writeln( &format!("test starts={} len={}", starts, len));
    test( &line, starts, len, uchu);
}

/**
 * 指した手を１手戻す。
 */
pub fn do_undo(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    if !uchu.undo_ss() {
        g_writeln( &format!("teme={} を、これより戻せません", uchu.teme ) );
    }
}

/**
 * 指し手を入れる。
 */
pub fn do_do(uchu:&mut Uchu, len:usize, line: &String, starts:&mut usize) {
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

/**
 * 初期局面表示。
 */
pub fn do_ky0(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    let s = uchu.kaku_ky( &KyNums::Start );
    g_writeln( &s );
}

/**
 * USIプロトコル参照。
 */
pub fn do_usi(_uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    g_writeln( &format!("id name {}", ENGINE_NAME) );
    g_writeln( &format!("id author {}", ENGINE_AUTHOR) );
    g_writeln("usiok");
}

/**
 * 思考を開始する。bestmoveコマンドを返却する。
 */
pub fn do_go(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    // go btime 40000 wtime 50000 binc 10000 winc 10000
    let bestmove = think( uchu );
    // 例： bestmove 7g7f
    g_writeln(&format!("bestmove {}", bestmove));
}

/**
 * 現局面表示。
 */
pub fn do_ky(uchu:&mut Uchu, _len:usize, _line: &String, _starts:&mut usize) {
    let s = &uchu.kaku_ky( &KyNums::Current );
    g_writeln( &s );            
}
