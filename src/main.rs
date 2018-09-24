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
mod shell_usi_impl;
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
use shell_usi_impl::*;

// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    static ref UCHU_WRAP: RwLock<Uchu> = RwLock::new(Uchu::new());
}

const GRAPH_JSON_FILE: &str = "graph.json";
fn main() {
    //println!("main: begin.");
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
        {
            let mut logger = LOGGER.try_write().unwrap();
            logger.set_file_path(&"log-kw", &".log");
            logger.delete_old_file(3);
        }
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
    //println!("main: Graph::new().");
    let mut graph = Graph::new();
    // コントローラーを登録。
    // [C]
    graph.insert_fn("do_cmate0", do_cmate0);
    graph.insert_fn("do_cmate0auto", do_cmate0auto);
    // [D]
    graph.insert_fn("do_do", do_do);
    // [G]
    graph.insert_fn("do_getmate", do_getmate);
    // [H]
    graph.insert_fn("do_hash", do_hash);
    // [K]
    graph.insert_fn("do_kifu", do_kifu);
    graph.insert_fn("do_kikisu", do_kikisu);
    graph.insert_fn("do_kmmove", do_kmmove);
    graph.insert_fn("do_kmugokidir", do_kmugokidir);
    graph.insert_fn("do_kmugoki", do_kmugoki);
    graph.insert_fn("do_ky0", do_ky0);
    graph.insert_fn("do_ky", do_ky);
    // [O]
    graph.insert_fn("do_other", do_other);
    // [Q]
    graph.insert_fn("do_quit", do_quit);
    // [R]
    graph.insert_fn("do_rand", do_rand);
    graph.insert_fn("do_reload", do_reload);
    graph.insert_fn("do_rndkms", do_rndkms);
    graph.insert_fn("do_rndms", do_rndms);
    graph.insert_fn("do_rndpos", do_rndpos);
    // [S]
    graph.insert_fn("do_same", do_same);
    graph.insert_fn("do_sasite", do_sasite);
    // [T]
    graph.insert_fn("do_teigi_conv", do_teigi_conv);
    graph.insert_fn("do_test", do_test);
    // [U]
    graph.insert_fn("do_undo", do_undo);

    // #### USI ####
    // [G]
    graph.insert_fn("do_go", do_go);
    graph.insert_fn("do_go_btime", do_go_btime);
    graph.insert_fn("do_go_btimevar", do_go_btimevar);
    graph.insert_fn("do_go_wtime", do_go_wtime);
    graph.insert_fn("do_go_wtimevar", do_go_wtimevar);
    graph.insert_fn("do_go_binc", do_go_binc);
    graph.insert_fn("do_go_bincvar", do_go_bincvar);
    graph.insert_fn("do_go_winc", do_go_winc);
    graph.insert_fn("do_go_wincvar", do_go_wincvar);
    graph.insert_fn("do_go_linebreak", do_go_linebreak);
    // [I]
    graph.insert_fn("do_isready", do_isready);
    // [P]
    graph.insert_fn("do_position", do_position);
    graph.insert_fn("do_position_sfen_board", do_position_sfen_board);
    graph.insert_fn("do_position_sfen_hands", do_position_sfen_hands);
    graph.insert_fn("do_position_sfen_movevar", do_position_sfen_movevar);
    graph.insert_fn("do_position_startpos", do_position_startpos);
    // [S]
    graph.insert_fn("do_setoption", do_setoption);
    graph.insert_fn("do_setoption_name", do_setoption_name);
    graph.insert_fn("do_setoption_namevar", do_setoption_namevar);
    graph.insert_fn("do_setoption_value", do_setoption_value);
    graph.insert_fn("do_setoption_valuevar", do_setoption_valuevar);
    graph.insert_fn("do_setoption_linebreak", do_setoption_linebreak);
    // [U]
    graph.insert_fn("do_usinewgame", do_usinewgame);
    graph.insert_fn("do_usi", do_usi);

    // ファイルからグラフのノード構成を読取。
    //println!("main: read_graph_file.");
    graph.read_graph_file(GRAPH_JSON_FILE.to_string());
    // - 正規表現は、うまく作れていない。全体を丸括弧で囲む。1個だけ。
    // - #linebreak コールバック関数は行終了時に実行される。

    // シェルの作成。
    //println!("main: Shell:new().");
    let mut shell = Shell::new();

    // [Ctrl]+[C] で強制終了
    //println!("main: shell.run:");
    shell.run(&graph, &mut shell_var);
}
