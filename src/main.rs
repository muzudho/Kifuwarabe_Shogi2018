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
use misc::option::*;
use misc::position_ex::*;


// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    static ref UCHU_WRAP: RwLock<Uchu> = RwLock::new(Uchu::new());

    // エンジン設定。
    static ref ENGINE_SETTINGS_WRAP: RwLock<EngineSettings> = RwLock::new(EngineSettings::new());
    // 初期局面。
    static ref INI_POSITION_WRAP: RwLock<Kyokumen> = RwLock::new(Kyokumen::new());
    // 計算中の局面。
    pub static ref CUR_POSITION_WRAP: RwLock<Kyokumen> = RwLock::new(Kyokumen::new());
    // 計算中の局面(拡張)。
    pub static ref CUR_POSITION_EX_WRAP: RwLock<PositionEx> = RwLock::new(PositionEx::new());
    // 棋譜。
    static ref GAME_RECORD_WRAP: RwLock<GameRecord> = RwLock::new(GameRecord::new());
}

fn main() {

    // 宇宙爆誕。
    UCHU_WRAP.try_write().unwrap().big_bang();
    
    let mut shell = Shell::new();

    // 該当なしのときに実行されるコールバック関数を選択。
    shell.set_complementary_controller(do_other);

    // コマンド トークン構成。
    shell.insert_node("ND_do", "do ", do_do);
    shell.insert_node("ND_go", "go", do_go);
    shell.insert_node("ND_hash", "hash", do_hash);
    shell.insert_node("ND_hirate", "hirate", do_hirate);
    shell.insert_node("ND_isready", "isready", do_isready);
    shell.insert_node("ND_kifu", "kifu", do_kifu);
    shell.insert_node("ND_kikisu", "kikisu", do_kikisu);
    shell.insert_node("ND_kmugokidir", "kmugokidir", do_kmugokidir);
    shell.insert_node("ND_kmugoki", "kmugoki", do_kmugoki);
    shell.insert_node("ND_ky0", "ky0", do_ky0);
    shell.insert_node("ND_ky", "ky", do_ky);

    shell.insert_node("ND_option", "option", do_option);
    shell.insert_node("ND_option_name", "name", do_option_name);
    shell.insert_node_re("ND_option_namevar", r"(\w+)", do_option_namevar);
    shell.insert_node("ND_option_type", "type", do_option_type);
    shell.insert_node_re("ND_option_typevar", r"(\w+)", do_option_typevar);
    shell.insert_node("ND_option_default", "default", do_option_default);
    shell.insert_node_re("ND_option_defaultvar", r"(\d+)", do_option_defaultvar);
    shell.insert_node("ND_option_min", "min", do_option_min);
    shell.insert_node_re("ND_option_minvar", r"(\d+)", do_option_minvar);
    shell.insert_node("ND_option_max", "max", do_option_max);
    shell.insert_node_re("ND_option_maxvar", r"(\d+)", do_option_maxvar);

    shell.insert_node("ND_position", "position", do_position);
    shell.insert_node("ND_quit", "quit", do_quit);
    shell.insert_node("ND_rand", "rand", do_rand);
    shell.insert_node("ND_rndkms", "rndkms", do_rndkms);
    shell.insert_node("ND_rndms", "rndms", do_rndms);
    shell.insert_node("ND_same", "same", do_same);
    shell.insert_node("ND_sasite", "sasite", do_sasite);

    shell.insert_node("ND_teigi_conv", "teigi::conv", do_teigi_conv);
    shell.insert_node("ND_test", "test", do_test);
    shell.insert_node("ND_usinewgame", "usinewgame", do_usinewgame);
    shell.insert_node("ND_undo", "undo", do_undo);
    shell.insert_node("ND_usi", "usi", do_usi);

    // 開始ノードを選択する。
    shell.set_next("ND_do,ND_go,ND_hash,ND_hirate,ND_isready,
    ND_kifu,ND_kikisu,ND_kmugokidir,ND_kmugoki,ND_ky0,ND_ky,
    ND_option,ND_position,ND_quit,
    ND_rand,ND_rndkms,ND_rndms,ND_same,ND_sasite,ND_teigi_conv,
    ND_test,ND_usinewgame,ND_undo,ND_usi");

    // [Ctrl]+[C] で強制終了
    shell.run();
}
