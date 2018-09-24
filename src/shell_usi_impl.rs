/// USIプロトコルのパーサー。
// デバッグ出力。
// const VERBOSE: bool = false;

use config::*;
use shell_impl::*;
use LOGGER;
use UCHU_WRAP;
use *;

/*****
 * U *
 *****/

pub fn do_usi(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    LOGGER
        .try_write()
        .unwrap()
        .writeln(&format!("id name {}", ENGINE_NAME));
    LOGGER
        .try_write()
        .unwrap()
        .writeln(&format!("id author {}", ENGINE_AUTHOR));
    LOGGER
        .try_write()
        .unwrap()
        .writeln("option name depth type spin default 1 min 1 max 999");
    LOGGER.try_write().unwrap().writeln("usiok");

    // 空打ちしてもタイトル画面は出さない☆（＾～＾）
    let mut uchu_w = UCHU_WRAP.try_write().unwrap();
    uchu_w.title_dirty = false;
    uchu_w.console_game_mode = false;
}

pub fn do_usinewgame(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    // 初期局面、現局面ともにクリアーします。手目も 0 に戻します。
    shell_var.searcher.ini_position.clear();
    shell_var.searcher.cur_position.clear();
    shell_var.searcher.game_record.set_teme(0);
}
