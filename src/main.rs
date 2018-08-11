///
/// きふわらべ将棋2018
///
/// 外部クレートを利用しているので、cargo build でコンパイルすること。rustc main.rs ではコンパイルが成功しない。
/// 実行ファイルは target/debug/kifuwarabe_shogi2018.exe だぜ☆
///
extern crate rand;
#[macro_use]
extern crate lazy_static;

extern crate kifuwarabe_shell;
use kifuwarabe_shell::Token;
use kifuwarabe_shell::Shell;

extern crate kifuwarabe_usi;

extern crate kifuwarabe_position;
use kifuwarabe_position::*;

extern crate kifuwarabe_movement;
use kifuwarabe_movement::*;

///
/// Rust言語の mod や ソース置き場の説明
///      「Rust のモジュールシステム」
///      https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
///
/// use したい モジュールは、最初に読み取られる　この main.rs ファイルに並べる

mod actions;
mod config;
mod consoles;
mod kasetu;
mod meidai;
mod mediators;
mod memory;
mod misc;
mod searchs;
mod syazo;
mod thinks;
mod teigi;
//mod teiri;
mod tusin;

use actions::command_list::*;
use memory::uchu::*;
use misc::position_ex::*;


// グローバル変数
use std::sync::RwLock;
lazy_static! {
    static ref UCHU_WRAP: RwLock<Uchu> = RwLock::new(Uchu::new());

    // 初期局面
    static ref INI_POSITION_WRAP: RwLock<Kyokumen> = RwLock::new(Kyokumen::new());
    // 計算中の局面
    pub static ref CUR_POSITION_WRAP: RwLock<Kyokumen> = RwLock::new(Kyokumen::new());
    // 計算中の局面(拡張)
    pub static ref CUR_POSITION_EX_WRAP: RwLock<PositionEx> = RwLock::new(PositionEx::new());
    // 棋譜
    static ref GAME_RECORD_WRAP: RwLock<GameRecord> = RwLock::new(GameRecord::new());
}

fn main() {

    // 宇宙爆誕。
    UCHU_WRAP.try_write().unwrap().big_bang();
    
    // コマンド リスト。
    let mut shell = Shell::new();
    shell.push_token(Token { token: "do ".to_string(), callback: do_do});
    shell.push_token(Token { token: "go".to_string(), callback: do_go});
    shell.push_token(Token { token: "hash".to_string(), callback: do_hash});
    shell.push_token(Token { token: "hirate".to_string(), callback: do_hirate});
    shell.push_token(Token { token: "isready".to_string(), callback: do_isready});
    shell.push_token(Token { token: "kifu".to_string(), callback: do_kifu});
    shell.push_token(Token { token: "kikisu".to_string(), callback: do_kikisu});
    shell.push_token(Token { token: "kmugokidir".to_string(), callback: do_kmugokidir});
    shell.push_token(Token { token: "kmugoki".to_string(), callback: do_kmugoki});
    shell.push_token(Token { token: "ky0".to_string(), callback: do_ky0});
    shell.push_token(Token { token: "ky".to_string(), callback: do_ky});
    shell.push_token(Token { token: "position".to_string(), callback: do_position});
    shell.push_token(Token { token: "quit".to_string(), callback: do_quit});
    shell.push_token(Token { token: "rand".to_string(), callback: do_rand});
    shell.push_token(Token { token: "rndkms".to_string(), callback: do_rndkms});
    shell.push_token(Token { token: "rndms".to_string(), callback: do_rndms});
    shell.push_token(Token { token: "same".to_string(), callback: do_same});
    shell.push_token(Token { token: "sasite".to_string(), callback: do_sasite});
    shell.push_token(Token { token: "setoption".to_string(), callback: do_setoption});
    shell.push_token(Token { token: "teigi::conv".to_string(), callback: do_teigi_conv});
    shell.push_token(Token { token: "test".to_string(), callback: do_test});
    shell.push_token(Token { token: "usinewgame".to_string(), callback: do_usinewgame});
    shell.push_token(Token { token: "undo".to_string(), callback: do_undo});
    shell.push_token(Token { token: "usi".to_string(), callback: do_usi});
    shell.set_other_callback(do_other);

    // [Ctrl]+[C] で強制終了
    shell.run();
}
