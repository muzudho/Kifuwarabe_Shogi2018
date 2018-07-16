///
/// きふわらべ将棋2018
///
/// 外部クレートを利用しているので、cargo build でコンパイルすること。rustc main.rs ではコンパイルが成功しない。
/// 実行ファイルは target/debug/kifuwarabe_shogi2018.exe だぜ☆
///
extern crate rand;
#[macro_use]
extern crate lazy_static;


///
/// Rust言語の mod や ソース置き場の説明
///      「Rust のモジュールシステム」
///      https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
///
/// use したい モジュールは、最初に読み取られる　この main.rs ファイルに並べる

mod actions;
mod config;
mod consoles;
mod memory;
mod kasetu;
mod meidai;
mod mediators;
mod thinks;
mod syazo;
mod teigi;
//mod teiri;
mod tusin;

use std::io;

use actions::command_list::*;
use memory::uchu::*;




fn main() {

    // 宇宙。
    let mut uchu : Uchu = Uchu::new();
    uchu.big_bang();
    
    // コマンド リスト。
    let command_list : CommandList = CommandList::new();



    // [Ctrl]+[C] で強制終了
    loop{

        let mut line : String;
        if uchu.is_empty_command() {
            line = String::new();
        } else {
            // バッファーに溜まっていれば☆（＾～＾）
            line = uchu.pop_command();
        }

        // まず最初に、コマンドライン入力を待機しろだぜ☆（＾～＾）
        io::stdin().read_line(&mut line)
            .ok()// read_lineの返り値オブジェクトResult の okメソッド
            .expect("info Failed to read line");// OKで無かった場合のエラーメッセージ

        // 末尾の改行を除こうぜ☆（＾～＾）
        // trim すると空白も消えるぜ☆（＾～＾）
        let line : String = line.trim().parse().ok().expect("info Failed to parse");

        // 文字数を調べようぜ☆（＾～＾）
        let len = line.chars().count();
        let mut starts = 0;

        let mut is_done = false;

        for element in command_list.command_array.iter() {
            if element.is_matched(len, &line, &starts) {
                element.move_caret_and_go(&mut uchu, len, &line, &mut starts);
                is_done = true;
                break;
            }
        }

        // 何とも一致しなかったら実行する。
        if !is_done {
            command_list.action_len_zero.move_caret_and_go(&mut uchu, len, &line, &mut starts);
        }

        if uchu.is_quit {
            // ループを抜けて終了
            break;
        }
    }//loop
}
