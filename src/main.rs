///
/// きふわらべ将棋2018
///
/// 外部クレートを利用しているので、cargo build でコンパイルすること。rustc main.rs ではコンパイルが成功しない。
/// 実行ファイルは target/debug/kifuwarabe_shogi2018.exe だぜ☆
///
extern crate rand;
#[macro_use]
extern crate lazy_static;

extern crate kifuwarabe_commander;
use kifuwarabe_commander::Command;
use kifuwarabe_commander::Commander;

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
mod searchs;
mod syazo;
mod thinks;
mod teigi;
//mod teiri;
mod tusin;

use actions::command_list::*;
use memory::uchu::*;


// グローバル変数
use std::sync::RwLock;
lazy_static! {
    static ref UCHU_WRAP: RwLock<Uchu> = RwLock::new(Uchu::new());
}

fn main() {

    // 宇宙爆誕。
    UCHU_WRAP.write().unwrap().big_bang();
    
    // コマンド リスト。
    let mut commander = Commander::new();
    commander.action_len_zero = Command {keyword: "".to_string(), callback: do_len_zero};
    commander.command_array.push(Command {keyword: "kmugokidir".to_string(), callback: do_kmugokidir});
    commander.command_array.push(Command { keyword: "usinewgame".to_string(), callback: do_usinewgame});
    commander.command_array.push(Command { keyword: "position".to_string(), callback: do_position});
    commander.command_array.push(Command { keyword: "isready".to_string(), callback: do_isready});
    commander.command_array.push(Command { keyword: "kmugoki".to_string(), callback: do_kmugoki});
    commander.command_array.push(Command { keyword: "hirate".to_string(), callback: do_hirate});
    commander.command_array.push(Command { keyword: "kikisu".to_string(), callback: do_kikisu});
    commander.command_array.push(Command { keyword: "rndkms".to_string(), callback: do_rndkms});
    commander.command_array.push(Command { keyword: "sasite".to_string(), callback: do_sasite});
    commander.command_array.push(Command { keyword: "rndms".to_string(), callback: do_rndms});
    commander.command_array.push(Command { keyword: "teigi::conv".to_string(), callback: do_teigi_conv});
    commander.command_array.push(Command { keyword: "hash".to_string(), callback: do_hash});
    commander.command_array.push(Command { keyword: "kifu".to_string(), callback: do_kifu});
    commander.command_array.push(Command { keyword: "quit".to_string(), callback: do_quit});
    commander.command_array.push(Command { keyword: "rand".to_string(), callback: do_rand});
    commander.command_array.push(Command { keyword: "same".to_string(), callback: do_same});
    commander.command_array.push(Command { keyword: "test".to_string(), callback: do_test});
    commander.command_array.push(Command { keyword: "undo".to_string(), callback: do_undo});
    commander.command_array.push(Command { keyword: "do ".to_string(), callback: do_do});
    commander.command_array.push(Command { keyword: "ky0".to_string(), callback: do_ky0});
    commander.command_array.push(Command { keyword: "usi".to_string(), callback: do_usi});
    commander.command_array.push(Command { keyword: "go".to_string(), callback: do_go});
    commander.command_array.push(Command { keyword: "ky".to_string(), callback: do_ky});

    // [Ctrl]+[C] で強制終了
    commander.run();
}
