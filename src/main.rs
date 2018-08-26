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

extern crate kifuwarabe_alpha_beta_search;

///
/// Rust言語の mod や ソース置き場の説明
///      「Rust のモジュールシステム」
///      https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
///
/// use したい モジュールは、最初に読み取られる　この main.rs ファイルに並べる

mod config;
mod consoles;
mod kasetu;
mod meidai;
mod mediators;
mod memory;
mod misc;
mod searcher_impl;
mod shell_impl;
mod syazo;
mod thinks;
mod time_manager;
mod teigi;
//mod teiri;
mod tusin;

use memory::uchu::*;
use misc::option::*;
use shell_impl::*;


// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    static ref UCHU_WRAP: RwLock<Uchu> = RwLock::new(Uchu::new());

    // エンジン設定。
    static ref ENGINE_SETTINGS_WRAP: RwLock<EngineSettings> = RwLock::new(EngineSettings::new());

    // 初期局面。
    pub static ref INI_POSITION_WRAP: RwLock<Position> = RwLock::new(Position::new());

    // 探索部に渡す局面。
    pub static ref CUR_POSITION_WRAP: RwLock<Position> = RwLock::new(Position::new());

    // 棋譜。
    pub static ref GAME_RECORD_WRAP: RwLock<GameRecord> = RwLock::new(GameRecord::new());
}




fn main() {

    // 宇宙爆誕。
    UCHU_WRAP.try_write().unwrap().big_bang();
    
    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();

    // シェルの作成。
    let mut shell = new_shell();

    // 該当なしのときに実行されるコールバック関数を選択。
    set_complementary_controller(&mut shell, do_other);

    // コマンド トークン構成。
    insert_node(&mut shell, "ND_do", "do ", do_do);

    insert_node(&mut shell, "ND_go", "go", do_go);
    insert_node(&mut shell, "ND_go_btime", "btime", do_go_btime);
    insert_node_re(&mut shell, "ND_go_btimevar", r"(\d+)", do_go_btimevar);
    insert_node(&mut shell, "ND_go_wtime", "wtime", do_go_wtime);
    insert_node_re(&mut shell, "ND_go_wtimevar", r"(\d+)", do_go_wtimevar);
    insert_node(&mut shell, "ND_go_binc", "binc", do_go_binc);
    insert_node_re(&mut shell, "ND_go_bincvar", r"(\d+)", do_go_bincvar);
    insert_node(&mut shell, "ND_go_winc", "winc", do_go_winc);
    insert_node_re(&mut shell, "ND_go_wincvar", r"(\d+)", do_go_wincvar);

    insert_node(&mut shell, "ND_hash", "hash", do_hash);
    insert_node(&mut shell, "ND_hirate", "hirate", do_hirate);
    insert_node(&mut shell, "ND_isready", "isready", do_isready);
    insert_node(&mut shell, "ND_kifu", "kifu", do_kifu);
    insert_node(&mut shell, "ND_kikisu", "kikisu", do_kikisu);
    insert_node(&mut shell, "ND_kmmove", "kmmove", do_kmmove);
    insert_node(&mut shell, "ND_kmugokidir", "kmugokidir", do_kmugokidir);
    insert_node(&mut shell, "ND_kmugoki", "kmugoki", do_kmugoki);
    insert_node(&mut shell, "ND_ky0", "ky0", do_ky0);
    insert_node(&mut shell, "ND_ky", "ky", do_ky);

    insert_node(&mut shell, "ND_position", "position", do_position);
    insert_node(&mut shell, "ND_quit", "quit", do_quit);
    insert_node(&mut shell, "ND_rand", "rand", do_rand);
    insert_node(&mut shell, "ND_rndkms", "rndkms", do_rndkms);
    insert_node(&mut shell, "ND_rndms", "rndms", do_rndms);
    insert_node(&mut shell, "ND_same", "same", do_same);
    insert_node(&mut shell, "ND_sasite", "sasite", do_sasite);

    insert_node(&mut shell, "ND_setoption", "setoption", do_setoption);
    insert_node(&mut shell, "ND_setoption_name", "name", do_setoption_name);
    insert_node_re(&mut shell, "ND_setoption_namevar", r"(\w+)", do_setoption_namevar);
    insert_node(&mut shell, "ND_setoption_value", "value", do_setoption_value);
    insert_node_re(&mut shell, "ND_setoption_valuevar", r"(\w+)", do_setoption_valuevar);

    insert_node(&mut shell, "ND_teigi_conv", "teigi::conv", do_teigi_conv);
    insert_node(&mut shell, "ND_test", "test", do_test);
    insert_node(&mut shell, "ND_usinewgame", "usinewgame", do_usinewgame);
    insert_node(&mut shell, "ND_undo", "undo", do_undo);
    insert_node(&mut shell, "ND_usi", "usi", do_usi);

    // 開始ノードを選択する。
    set_next(&mut shell, "ND_do,ND_go,ND_hash,ND_hirate,ND_isready,
    ND_kifu,ND_kikisu,ND_kmugokidir,ND_kmugoki,ND_ky0,ND_ky,

    ND_setoption, ND_setoption_name, ND_setoption_namevar, ND_setoption_value, ND_setoption_valuevar,

    ND_position,ND_quit,
    ND_rand,ND_rndkms,ND_rndms,ND_same,ND_sasite,ND_teigi_conv,
    ND_test,ND_usinewgame,ND_undo,ND_usi");

    // [Ctrl]+[C] で強制終了
    run(&mut shell, &mut shell_var);
}
