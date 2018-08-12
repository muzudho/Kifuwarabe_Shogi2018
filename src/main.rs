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
use kifuwarabe_shell::*;

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
    
    let mut shell = Shell::new();
    // コールバック関数を登録する。
    shell.insert_callback("CB_do", do_do);
    shell.insert_callback("CB_go", do_go);
    shell.insert_callback("CB_hash", do_hash);
    shell.insert_callback("CB_hirate", do_hirate);
    shell.insert_callback("CB_isready", do_isready);
    shell.insert_callback("CB_kifu", do_kifu);
    shell.insert_callback("CB_kikisu", do_kikisu);
    shell.insert_callback("CB_kmugokidir", do_kmugokidir);
    shell.insert_callback("CB_kmugoki", do_kmugoki);
    shell.insert_callback("CB_ky0", do_ky0);
    shell.insert_callback("CB_ky", do_ky);
    shell.insert_callback("CB_other", do_other);
    shell.insert_callback("CB_position", do_position);
    shell.insert_callback("CB_quit", do_quit);
    shell.insert_callback("CB_rand", do_rand);
    shell.insert_callback("CB_rndkms", do_rndkms);
    shell.insert_callback("CB_rndms", do_rndms);
    shell.insert_callback("CB_same", do_same);
    shell.insert_callback("CB_sasite", do_sasite);
    shell.insert_callback("CB_setoption", do_setoption);
    shell.insert_callback("CB_teigi_conv", do_teigi_conv);
    shell.insert_callback("CB_test", do_test);
    shell.insert_callback("CB_usinewgame", do_usinewgame);
    shell.insert_callback("CB_undo", do_undo);
    shell.insert_callback("CB_usi", do_usi);
    // 該当なしのときに実行されるコールバック関数を選択。
    shell.set_complementary_callback("CB_other");

    // コマンド トークン構成。
    shell.insert_node("ND_do", "do ", "CB_do", "");
    shell.insert_node("ND_go", "go", "CB_go", "");
    shell.insert_node("ND_hash", "hash", "CB_hash", "");
    shell.insert_node("ND_hirate", "hirate", "CB_hirate", "");
    shell.insert_node("ND_isready", "isready", "CB_isready", "");
    shell.insert_node("ND_kifu", "kifu", "CB_kifu", "");
    shell.insert_node("ND_kikisu", "kikisu", "CB_kikisu", "");
    shell.insert_node("ND_kmugokidir", "kmugokidir", "CB_kmugokidir", "");
    shell.insert_node("ND_kmugoki", "kmugoki", "CB_kmugoki", "");
    shell.insert_node("ND_ky0", "ky0", "CB_ky0", "");
    shell.insert_node("ND_ky", "ky", "CB_ky", "");
    shell.insert_node("ND_position", "position", "CB_position", "");
    shell.insert_node("ND_quit", "quit", "CB_quit", "");
    shell.insert_node("ND_rand", "rand", "CB_rand", "");
    shell.insert_node("ND_rndkms", "rndkms", "CB_rndkms", "");
    shell.insert_node("ND_rndms", "rndms", "CB_rndms", "");
    shell.insert_node("ND_same", "same", "CB_same", "");
    shell.insert_node("ND_sasite", "sasite", "CB_sasite", "");
    shell.insert_node("ND_setoption", "setoption", "CB_setoption", "");
    shell.insert_node("ND_teigi_conv", "teigi::conv", "CB_teigi_conv", "");
    shell.insert_node("ND_test", "test", "CB_test", "");
    shell.insert_node("ND_usinewgame", "usinewgame", "CB_usinewgame", "");
    shell.insert_node("ND_undo", "undo", "CB_undo", "");
    shell.insert_node("ND_usi", "usi", "CB_usi", "");

    // 開始ノードを選択する。
    shell.set_next("ND_do,ND_go,ND_hash,ND_hirate,ND_isready,
    ND_kifu,ND_kikisu,ND_kmugokidir,ND_kmugoki,ND_ky0,ND_ky,ND_position,ND_quit,
    ND_rand,ND_rndkms,ND_rndms,ND_same,ND_sasite,ND_setoption,ND_teigi_conv,
    ND_test,ND_usinewgame,ND_undo,ND_usi");

    // [Ctrl]+[C] で強制終了
    shell.run();
}
