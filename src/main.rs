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

// #[macro_use(hashmap)]
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

const graph_json_file : &'static str = "graph.json";
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
    // コントローラーを登録。
    // [C]
    graph.insert_controller("do_cmate0", do_cmate0);
    graph.insert_controller("do_cmate0auto", do_cmate0auto);
    // [D]
    graph.insert_controller("do_do", do_do);
    // [G]
    graph.insert_controller("do_getmate", do_getmate);
    graph.insert_controller("do_go", do_go);
    graph.insert_controller("do_go_btime", do_go_btime);
    graph.insert_controller("do_go_btimevar", do_go_btimevar);
    graph.insert_controller("do_go_wtime", do_go_wtime);
    graph.insert_controller("do_go_wtimevar", do_go_wtimevar);
    graph.insert_controller("do_go_binc", do_go_binc);
    graph.insert_controller("do_go_bincvar", do_go_bincvar);
    graph.insert_controller("do_go_winc", do_go_winc);
    graph.insert_controller("do_go_wincvar", do_go_wincvar);
    graph.insert_controller("do_go_linebreak", do_go_linebreak);
    // [H]
    graph.insert_controller("do_hash", do_hash);
    graph.insert_controller("do_hirate", do_hirate);
    // [I]
    graph.insert_controller("do_isready", do_isready);
    // [K]
    graph.insert_controller("do_kifu", do_kifu);
    graph.insert_controller("do_kikisu", do_kikisu);
    graph.insert_controller("do_kmmove", do_kmmove);
    graph.insert_controller("do_kmugokidir", do_kmugokidir);
    graph.insert_controller("do_kmugoki", do_kmugoki);
    graph.insert_controller("do_ky0", do_ky0);
    graph.insert_controller("do_ky", do_ky);
    // [O]
    graph.insert_controller("do_other", do_other);
    // [P]
    graph.insert_controller("do_position", do_position);
    // [Q]
    graph.insert_controller("do_quit", do_quit);
    // [R]
    graph.insert_controller("do_rand", do_rand);
    graph.insert_controller("do_reload", do_reload);
    graph.insert_controller("do_rndkms", do_rndkms);
    graph.insert_controller("do_rndms", do_rndms);
    graph.insert_controller("do_rndpos", do_rndpos);
    // [S]
    graph.insert_controller("do_same", do_same);
    graph.insert_controller("do_sasite", do_sasite);
    graph.insert_controller("do_setoption", do_setoption);
    graph.insert_controller("do_setoption_name", do_setoption_name);
    graph.insert_controller("do_setoption_namevar", do_setoption_namevar);
    graph.insert_controller("do_setoption_value", do_setoption_value);
    graph.insert_controller("do_setoption_valuevar", do_setoption_valuevar);
    graph.insert_controller("do_setoption_linebreak", do_setoption_linebreak);
    // [T]
    graph.insert_controller("do_teigi_conv", do_teigi_conv);
    graph.insert_controller("do_test", do_test);
    // [U]
    graph.insert_controller("do_usinewgame", do_usinewgame);
    graph.insert_controller("do_undo", do_undo);
    graph.insert_controller("do_usi", do_usi);

    // ファイルからグラフのノード構成を読取。
    graph.read_graph_file(graph_json_file.to_string());
    // - 正規表現は、うまく作れていない。全体を丸括弧で囲む。1個だけ。
    // - #linebreak コールバック関数は行終了時に実行される。

    // シェルの作成。
    let mut shell = Shell::new();

    // [Ctrl]+[C] で強制終了
    shell.run(&graph, &mut shell_var);
}
