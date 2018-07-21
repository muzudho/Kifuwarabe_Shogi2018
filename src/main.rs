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
use kifuwarabe_shell::TokenMapping;
use kifuwarabe_shell::Shell;

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
    let mut shell = Shell::new();
    shell.other_token_mapping = TokenMapping {token: "".to_string(), callback: do_len_zero};
    shell.token_mapping_array.push(TokenMapping {token: "kmugokidir".to_string(), callback: do_kmugokidir});
    shell.token_mapping_array.push(TokenMapping { token: "usinewgame".to_string(), callback: do_usinewgame});
    shell.token_mapping_array.push(TokenMapping { token: "position".to_string(), callback: do_position});
    shell.token_mapping_array.push(TokenMapping { token: "isready".to_string(), callback: do_isready});
    shell.token_mapping_array.push(TokenMapping { token: "kmugoki".to_string(), callback: do_kmugoki});
    shell.token_mapping_array.push(TokenMapping { token: "hirate".to_string(), callback: do_hirate});
    shell.token_mapping_array.push(TokenMapping { token: "kikisu".to_string(), callback: do_kikisu});
    shell.token_mapping_array.push(TokenMapping { token: "rndkms".to_string(), callback: do_rndkms});
    shell.token_mapping_array.push(TokenMapping { token: "sasite".to_string(), callback: do_sasite});
    shell.token_mapping_array.push(TokenMapping { token: "rndms".to_string(), callback: do_rndms});
    shell.token_mapping_array.push(TokenMapping { token: "teigi::conv".to_string(), callback: do_teigi_conv});
    shell.token_mapping_array.push(TokenMapping { token: "hash".to_string(), callback: do_hash});
    shell.token_mapping_array.push(TokenMapping { token: "kifu".to_string(), callback: do_kifu});
    shell.token_mapping_array.push(TokenMapping { token: "quit".to_string(), callback: do_quit});
    shell.token_mapping_array.push(TokenMapping { token: "rand".to_string(), callback: do_rand});
    shell.token_mapping_array.push(TokenMapping { token: "same".to_string(), callback: do_same});
    shell.token_mapping_array.push(TokenMapping { token: "test".to_string(), callback: do_test});
    shell.token_mapping_array.push(TokenMapping { token: "undo".to_string(), callback: do_undo});
    shell.token_mapping_array.push(TokenMapping { token: "do ".to_string(), callback: do_do});
    shell.token_mapping_array.push(TokenMapping { token: "ky0".to_string(), callback: do_ky0});
    shell.token_mapping_array.push(TokenMapping { token: "usi".to_string(), callback: do_usi});
    shell.token_mapping_array.push(TokenMapping { token: "go".to_string(), callback: do_go});
    shell.token_mapping_array.push(TokenMapping { token: "ky".to_string(), callback: do_ky});

    // [Ctrl]+[C] で強制終了
    shell.run();
}
