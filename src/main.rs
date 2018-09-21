#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

///
/// きふわらべ将棋2018
///
/// 外部クレートを利用しているので、cargo build でコンパイルすること。rustc main.rs ではコンパイルが成功しない。
/// 実行ファイルは target/debug/kifuwarabe_shogi2018.exe だぜ☆
///
extern crate chrono;
extern crate time;
extern crate rand;
#[macro_use]
extern crate lazy_static;

#[macro_use(hashmap)]
extern crate kifuwarabe_shell;
use kifuwarabe_shell::graph::*;
use kifuwarabe_shell::node::*;
use kifuwarabe_shell::shell::*;

extern crate kifuwarabe_usi;
extern crate kifuwarabe_position;
use kifuwarabe_position::*;

extern crate kifuwarabe_movement;
extern crate kifuwarabe_movement_picker;
extern crate kifuwarabe_alpha_beta_search;

///
/// Rust言語の mod や ソース置き場の説明
///      「Rust のモジュールシステム」
///      https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
///
/// use したい モジュールは、最初に読み取られる　この main.rs ファイルに並べる

mod config;
mod consoles;
mod display_impl;
mod kasetu;
mod logger;
mod meidai;
mod mediators;
mod memory;
mod misc;
mod movement_thinks;
mod searcher_impl;
mod shell_impl;
mod thinks;
mod time_manager;
mod teigi;
//mod teiri;
mod tusin;

// use chrono::prelude::*; // DateTime<Local>
use logger::LOGGER;
use memory::uchu::*;
use misc::option::*;
use rand::Rng;
use shell_impl::*;


// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    static ref UCHU_WRAP: RwLock<Uchu> = RwLock::new(Uchu::new());
}




fn main() {
    
    // ロガー
    {
        LOGGER.try_write().unwrap().set_file_path(&"log-kw", &".log");
        LOGGER.try_write().unwrap().delete_old_file();
    }

    // 任意の構造体を作成する。
    let mut shell_var = ShellVar::new();
    // グローバル変数と内容を合わせなくても、初期状態は同じ。

    // 局面ハッシュの種をリセット
    {
        // 盤上の駒
        for i_ms in MASU_0..BAN_SIZE {
            for i_km in 0..Koma::Num as usize {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                shell_var.searcher.game_record.ky_hash_seed.km[i_ms][i_km] = rand::thread_rng().gen_range(0,18_446_744_073_709_551_615);
            }
        }
        // 持ち駒
        for i_km in 0..Koma::Num as usize {
            for i_mg in 0..MG_MAX {
                shell_var.searcher.game_record.ky_hash_seed.mg[i_km][i_mg] = rand::thread_rng().gen_range(0,18_446_744_073_709_551_615);
            }
        }
        // 先後
        for i_sn in 0..Sengo::Num as usize {
            shell_var.searcher.game_record.ky_hash_seed.sn[i_sn] = rand::thread_rng().gen_range(0,18_446_744_073_709_551_615);
        }
    }

    // グラフの作成。
    let mut graph = new_graph();

    // 該当なしのときに実行されるコールバック関数を選択。
    set_complementary_controller(&mut graph, do_other);

    // グラフ ノード構成。

    // [C]
    insert_node(&mut graph, "ND_cmate0", "cmate0", do_cmate0, hashmap![]);
    insert_node(&mut graph, "ND_cmate0auto", "cmate0auto", do_cmate0auto, hashmap![]);

    // [D]
    insert_node(&mut graph, "ND_do", "do ", do_do, hashmap![]);

    // [G]
    insert_node(&mut graph, "ND_getmate", "getmate", do_getmate, hashmap![]);

    insert_node(&mut graph, "ND_go", "go", do_go, hashmap!["next" => "ND_go_btime"]);
    insert_node(&mut graph, "ND_go_btime", "btime", do_go_btime, hashmap!["next" => "ND_go_btimevar"]);
    insert_node_re(&mut graph, "ND_go_btimevar", r"(\d+)", do_go_btimevar, hashmap!["next" => "ND_go_wtime"]);
    insert_node(&mut graph, "ND_go_wtime", "wtime", do_go_wtime, hashmap!["next" => "ND_go_wtimevar"]);
    insert_node_re(&mut graph, "ND_go_wtimevar", r"(\d+)", do_go_wtimevar, hashmap!["next" => "ND_go_binc"]);
    insert_node(&mut graph, "ND_go_binc", "binc", do_go_binc, hashmap!["next" => "ND_go_bincvar"]);
    insert_node_re(&mut graph, "ND_go_bincvar", r"(\d+)", do_go_bincvar, hashmap!["next" => "ND_go_winc"]);
    insert_node(&mut graph, "ND_go_winc", "winc", do_go_winc, hashmap!["next" => "ND_go_wincvar"]);
    insert_node_re(&mut graph, "ND_go_wincvar", r"(\d+)", do_go_wincvar, hashmap![]);

    // [H]
    insert_node(&mut graph, "ND_hash", "hash", do_hash, hashmap![]);
    insert_node(&mut graph, "ND_hirate", "hirate", do_hirate, hashmap![]);

    // [I]
    insert_node(&mut graph, "ND_isready", "isready", do_isready, hashmap![]);

    // [K]
    insert_node(&mut graph, "ND_kifu", "kifu", do_kifu, hashmap![]);
    insert_node(&mut graph, "ND_kikisu", "kikisu", do_kikisu, hashmap![]);
    insert_node(&mut graph, "ND_kmmove", "kmmove", do_kmmove, hashmap![]);
    insert_node(&mut graph, "ND_kmugokidir", "kmugokidir", do_kmugokidir, hashmap![]);
    insert_node(&mut graph, "ND_kmugoki", "kmugoki", do_kmugoki, hashmap![]);
    insert_node(&mut graph, "ND_ky0", "ky0", do_ky0, hashmap![]);
    insert_node(&mut graph, "ND_ky", "ky", do_ky, hashmap![]);

    // [P]
    insert_node(&mut graph, "ND_position", "position", do_position, hashmap![]);

    // [Q]
    insert_node(&mut graph, "ND_quit", "quit", do_quit, hashmap![]);

    // [R]
    insert_node(&mut graph, "ND_rand", "rand", do_rand, hashmap![]);
    insert_node(&mut graph, "ND_rndkms", "rndkms", do_rndkms, hashmap![]);
    insert_node(&mut graph, "ND_rndms", "rndms", do_rndms, hashmap![]);
    insert_node(&mut graph, "ND_rndpos", "rndpos", do_rndpos, hashmap![]);

    // [S]
    insert_node(&mut graph, "ND_same", "same", do_same, hashmap![]);
    insert_node(&mut graph, "ND_sasite", "sasite", do_sasite, hashmap![]);

    insert_node(&mut graph, "ND_setoption", "setoption", do_setoption, hashmap!["next" => "ND_setoption_name"]);
    insert_node(&mut graph, "ND_setoption_name", "name", do_setoption_name, hashmap!["next" => "ND_setoption_namevar"]);
    insert_node_re(&mut graph, "ND_setoption_namevar", r"(\w+)", do_setoption_namevar, hashmap!["next" => "ND_setoption_value"]);
    insert_node(&mut graph, "ND_setoption_value", "value", do_setoption_value, hashmap!["next" => "ND_setoption_valuevar"]);
    insert_node_re(&mut graph, "ND_setoption_valuevar", r"(\w+)", do_setoption_valuevar, hashmap![]);

    // [T]
    insert_node(&mut graph, "ND_teigi_conv", "teigi::conv", do_teigi_conv, hashmap![]);
    insert_node(&mut graph, "ND_test", "test", do_test, hashmap![]);

    // [U]
    insert_node(&mut graph, "ND_usinewgame", "usinewgame", do_usinewgame, hashmap![]);
    insert_node(&mut graph, "ND_undo", "undo", do_undo, hashmap![]);
    insert_node(&mut graph, "ND_usi", "usi", do_usi, hashmap![]);



    // シェルの作成。
    let mut shell = new_shell();

    // 開始ノードを選択する。
    set_next(&mut shell, "ND_cmate0, ND_cmate0auto,
    ND_do,
    ND_getmate,
    ND_go,
    ND_hash,ND_hirate,ND_isready,
    ND_kifu,ND_kikisu,ND_kmugokidir,ND_kmugoki,ND_ky0,ND_ky,

    ND_setoption, ND_setoption_name, ND_setoption_namevar, ND_setoption_value, ND_setoption_valuevar,

    ND_position,ND_quit,
    ND_rand,ND_rndkms,ND_rndms,ND_rndpos,
    ND_same,ND_sasite,ND_teigi_conv,
    ND_test,ND_usinewgame,ND_undo,ND_usi");

    // [Ctrl]+[C] で強制終了
    run(&mut graph, &mut shell, &mut shell_var);
}
