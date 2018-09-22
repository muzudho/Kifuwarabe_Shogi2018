#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

///
/// きふわらべ将棋2018
///
/// 外部クレートを利用しているので、cargo build でコンパイルすること。rustc main.rs ではコンパイルが成功しない。
/// 実行ファイルは target/debug/kifuwarabe_shogi2018.exe だぜ☆
///
// #[macro_use]
// extern crate log;
extern crate env_logger;
// use log::Level;

extern crate chrono;
extern crate rand;
extern crate time;
#[macro_use]
extern crate lazy_static;

#[macro_use(hashmap)]
extern crate kifuwarabe_shell;
use kifuwarabe_shell::graph::*;
use kifuwarabe_shell::shell::*;

extern crate kifuwarabe_position;
extern crate kifuwarabe_usi;
use kifuwarabe_position::*;

extern crate kifuwarabe_alpha_beta_search;
extern crate kifuwarabe_movement;
extern crate kifuwarabe_movement_picker;

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
mod mediators;
mod meidai;
mod memory;
mod misc;
mod movement_thinks;
mod searcher_impl;
mod shell_impl;
mod teigi;
mod thinks;
mod time_manager;
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
    // TODO ロガー
    {
        // https://docs.rs/env_logger/0.5.13/env_logger/
        env_logger::init();
        /*
        debug!("this is a debug {}", "message");
        error!("this is printed by default");

        if log_enabled!(Level::Info) {
            let x = 3 * 4; // expensive computation
            info!("the answer was: {}", x);
        }
        */
        LOGGER
            .try_write()
            .unwrap()
            .set_file_path(&"log-kw", &".log");
        // TODO LOGGER.try_write().unwrap().delete_old_file();
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
                shell_var.searcher.game_record.ky_hash_seed.km[i_ms][i_km] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 持ち駒
        for i_km in 0..Koma::Num as usize {
            for i_mg in 0..MG_MAX {
                shell_var.searcher.game_record.ky_hash_seed.mg[i_km][i_mg] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 先後
        for i_sn in 0..Sengo::Num as usize {
            shell_var.searcher.game_record.ky_hash_seed.sn[i_sn] =
                rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
        }
    }

    // グラフの作成。
    let mut graph = Graph::new();

    // 該当なしのときに実行されるコールバック関数を選択
    graph.insert_node_single("#ND_complementary", do_other);

    // グラフ ノード構成。

    // [C]
    graph.insert_node("ND_cmate0", "cmate0", do_cmate0, hashmap![]);
    graph.insert_node("ND_cmate0auto", "cmate0auto", do_cmate0auto, hashmap![]);

    // [D]
    graph.insert_node("ND_do", "do ", do_do, hashmap![]);

    // [G]
    graph.insert_node("ND_getmate", "getmate", do_getmate, hashmap![]);

    graph.insert_node(
        "ND_go",
        "go",
        do_go,
        hashmap!["next" => "ND_go_btime", "#linebreak" => "ND_go_linebreak"],
    ); // #linebreak コールバック関数は行終了時に実行される。
    graph.insert_node(
        "ND_go_btime",
        "btime",
        do_go_btime,
        hashmap!["next" => "ND_go_btimevar"],
    );
    graph.insert_node_reg(
        "ND_go_btimevar",
        r"(\d+)",
        do_go_btimevar,
        hashmap!["next" => "ND_go_wtime"],
    );
    graph.insert_node(
        "ND_go_wtime",
        "wtime",
        do_go_wtime,
        hashmap!["next" => "ND_go_wtimevar"],
    );
    graph.insert_node_reg(
        "ND_go_wtimevar",
        r"(\d+)",
        do_go_wtimevar,
        hashmap!["next" => "ND_go_binc"],
    );
    graph.insert_node(
        "ND_go_binc",
        "binc",
        do_go_binc,
        hashmap!["next" => "ND_go_bincvar"],
    );
    graph.insert_node_reg(
        "ND_go_bincvar",
        r"(\d+)",
        do_go_bincvar,
        hashmap!["next" => "ND_go_winc"],
    );
    graph.insert_node(
        "ND_go_winc",
        "winc",
        do_go_winc,
        hashmap!["next" => "ND_go_wincvar"],
    );
    graph.insert_node_reg("ND_go_wincvar", r"(\d+)", do_go_wincvar, hashmap![]);
    graph.insert_node_single("ND_go_linebreak", do_go_linebreak);

    // [H]
    graph.insert_node("ND_hash", "hash", do_hash, hashmap![]);
    graph.insert_node("ND_hirate", "hirate", do_hirate, hashmap![]);

    // [I]
    graph.insert_node("ND_isready", "isready", do_isready, hashmap![]);

    // [K]
    graph.insert_node("ND_kifu", "kifu", do_kifu, hashmap![]);
    graph.insert_node("ND_kikisu", "kikisu", do_kikisu, hashmap![]);
    graph.insert_node("ND_kmmove", "kmmove", do_kmmove, hashmap![]);
    graph.insert_node("ND_kmugokidir", "kmugokidir", do_kmugokidir, hashmap![]);
    graph.insert_node("ND_kmugoki", "kmugoki", do_kmugoki, hashmap![]);
    graph.insert_node("ND_ky0", "ky0", do_ky0, hashmap![]);
    graph.insert_node("ND_ky", "ky", do_ky, hashmap![]);

    // [P]
    graph.insert_node("ND_position", "position", do_position, hashmap![]);

    // [Q]
    graph.insert_node("ND_quit", "quit", do_quit, hashmap![]);

    // [R]
    graph.insert_node("ND_rand", "rand", do_rand, hashmap![]);
    graph.insert_node("ND_rndkms", "rndkms", do_rndkms, hashmap![]);
    graph.insert_node("ND_rndms", "rndms", do_rndms, hashmap![]);
    graph.insert_node("ND_rndpos", "rndpos", do_rndpos, hashmap![]);

    // [S]
    graph.insert_node("ND_same", "same", do_same, hashmap![]);
    graph.insert_node("ND_sasite", "sasite", do_sasite, hashmap![]);

    graph.insert_node(
        "ND_setoption",
        "setoption",
        do_setoption,
        hashmap!["next" => "ND_setoption_name", "#linebreak" => "ND_setoption_linebreak"],
    );
    graph.insert_node(
        "ND_setoption_name",
        "name",
        do_setoption_name,
        hashmap!["next" => "ND_setoption_namevar"],
    );
    graph.insert_node_reg(
        "ND_setoption_namevar",
        r"(\w+)",
        do_setoption_namevar,
        hashmap!["next" => "ND_setoption_value"],
    );
    graph.insert_node(
        "ND_setoption_value",
        "value",
        do_setoption_value,
        hashmap!["next" => "ND_setoption_valuevar"],
    );
    graph.insert_node_reg(
        "ND_setoption_valuevar",
        r"([\d\w]+)",
        do_setoption_valuevar,
        hashmap![],
    );
    graph.insert_node_single("ND_setoption_linebreak", do_setoption_linebreak);

    // [T]
    graph.insert_node("ND_teigi_conv", "teigi::conv", do_teigi_conv, hashmap![]);
    graph.insert_node("ND_test", "test", do_test, hashmap![]);

    // [U]
    graph.insert_node("ND_usinewgame", "usinewgame", do_usinewgame, hashmap![]);
    graph.insert_node("ND_undo", "undo", do_undo, hashmap![]);
    graph.insert_node("ND_usi", "usi", do_usi, hashmap![]);

    // 開始ノードを選択する。
    graph.set_entrance("ND_cmate0, ND_cmate0auto,
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

    // シェルの作成。
    let mut shell = Shell::new();

    // [Ctrl]+[C] で強制終了
    shell.run(&graph, &mut shell_var);
}
