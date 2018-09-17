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
extern crate kifuwarabe_shell;
use kifuwarabe_shell::*;

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

    // シェルの作成。
    let mut shell = new_shell();

    // 該当なしのときに実行されるコールバック関数を選択。
    set_complementary_controller(&mut shell, do_other);

    // コマンド トークン構成。

    // [C]
    insert_node(&mut shell, "ND_cmate0", "cmate0", do_cmate0);
    insert_node(&mut shell, "ND_cmate0auto", "cmate0auto", do_cmate0auto);

    // [D]
    insert_node(&mut shell, "ND_do", "do ", do_do);

    // [G]
    insert_node(&mut shell, "ND_getmate", "getmate", do_getmate);

    insert_node(&mut shell, "ND_go", "go", do_go);
    insert_node(&mut shell, "ND_go_btime", "btime", do_go_btime);
    insert_node_re(&mut shell, "ND_go_btimevar", r"(\d+)", do_go_btimevar);
    insert_node(&mut shell, "ND_go_wtime", "wtime", do_go_wtime);
    insert_node_re(&mut shell, "ND_go_wtimevar", r"(\d+)", do_go_wtimevar);
    insert_node(&mut shell, "ND_go_binc", "binc", do_go_binc);
    insert_node_re(&mut shell, "ND_go_bincvar", r"(\d+)", do_go_bincvar);
    insert_node(&mut shell, "ND_go_winc", "winc", do_go_winc);
    insert_node_re(&mut shell, "ND_go_wincvar", r"(\d+)", do_go_wincvar);

    // [H]
    insert_node(&mut shell, "ND_hash", "hash", do_hash);
    insert_node(&mut shell, "ND_hirate", "hirate", do_hirate);

    // [I]
    insert_node(&mut shell, "ND_isready", "isready", do_isready);

    // [K]
    insert_node(&mut shell, "ND_kifu", "kifu", do_kifu);
    insert_node(&mut shell, "ND_kikisu", "kikisu", do_kikisu);
    insert_node(&mut shell, "ND_kmmove", "kmmove", do_kmmove);
    insert_node(&mut shell, "ND_kmugokidir", "kmugokidir", do_kmugokidir);
    insert_node(&mut shell, "ND_kmugoki", "kmugoki", do_kmugoki);
    insert_node(&mut shell, "ND_ky0", "ky0", do_ky0);
    insert_node(&mut shell, "ND_ky", "ky", do_ky);

    // [P]
    insert_node(&mut shell, "ND_position", "position", do_position);

    // [Q]
    insert_node(&mut shell, "ND_quit", "quit", do_quit);

    // [R]
    insert_node(&mut shell, "ND_rand", "rand", do_rand);
    insert_node(&mut shell, "ND_rndkms", "rndkms", do_rndkms);
    insert_node(&mut shell, "ND_rndms", "rndms", do_rndms);
    insert_node(&mut shell, "ND_rndpos", "rndpos", do_rndpos);

    // [S]
    insert_node(&mut shell, "ND_same", "same", do_same);
    insert_node(&mut shell, "ND_sasite", "sasite", do_sasite);

    insert_node(&mut shell, "ND_setoption", "setoption", do_setoption);
    insert_node(&mut shell, "ND_setoption_name", "name", do_setoption_name);
    insert_node_re(&mut shell, "ND_setoption_namevar", r"(\w+)", do_setoption_namevar);
    insert_node(&mut shell, "ND_setoption_value", "value", do_setoption_value);
    insert_node_re(&mut shell, "ND_setoption_valuevar", r"(\w+)", do_setoption_valuevar);

    // [T]
    insert_node(&mut shell, "ND_teigi_conv", "teigi::conv", do_teigi_conv);
    insert_node(&mut shell, "ND_test", "test", do_test);

    // [U]
    insert_node(&mut shell, "ND_usinewgame", "usinewgame", do_usinewgame);
    insert_node(&mut shell, "ND_undo", "undo", do_undo);
    insert_node(&mut shell, "ND_usi", "usi", do_usi);

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
    run(&mut shell, &mut shell_var);
}
