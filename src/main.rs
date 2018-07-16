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
mod jotai;
mod kasetu;
mod meidai;
mod siko;
mod syazo;
mod teigi;
//mod teiri;
mod tusin;

use std::io;

use actions::command_list::*;
use jotai::uchu::*;




type Callback = fn(uchu:&mut Uchu, len:usize, line: &String, starts:&mut usize);

/// [2016-12-10 Idiomatic callbacks in Rust](https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust)
struct Command {
    pub keyword: String,
    callback: Callback,
}
impl Command {

    fn is_matched(&self, len:usize, line: &String, starts:&usize) -> bool {
        return self.keyword.len()<=len && &line[*starts..self.keyword.len()] == self.keyword
    }

    fn move_caret_and_go(&self, uchu:&mut Uchu, len:usize, line: &String, starts:&mut usize) {
        *starts += self.keyword.len();
        // 続きにスペース「 」が１つあれば読み飛ばす
        if 0<(len-*starts) && &line[*starts..(*starts+1)]==" " {
            *starts+=1;
        }            

        (self.callback)(uchu, len, line, starts);
    }
}





fn main() {

    // 宇宙
    let mut uchu : Uchu = Uchu::new();
    uchu.big_bang();
    
    let p_len_zero = Command { keyword: "".to_string(), callback: do_len_zero };
    let command_array = [
        Command { keyword: "kmugokidir".to_string(), callback: do_kmugokidir },
        Command { keyword: "usinewgame".to_string(), callback: do_usinewgame },
        Command { keyword: "position".to_string(), callback: do_position },
        Command { keyword: "isready".to_string(), callback: do_isready },
        Command { keyword: "kmugoki".to_string(), callback: do_kmugoki },
        Command { keyword: "hirate".to_string(), callback: do_hirate },
        Command { keyword: "kikisu".to_string(), callback: do_kikisu },
        Command { keyword: "rndkms".to_string(), callback: do_rndkms },
        Command { keyword: "sasite".to_string(), callback: do_sasite },
        Command { keyword: "rndms".to_string(), callback: do_rndms },
        Command { keyword: "teigi::conv".to_string(), callback: do_teigi_conv },
        Command { keyword: "hash".to_string(), callback: do_hash },
        Command { keyword: "kifu".to_string(), callback: do_kifu },
        Command { keyword: "quit".to_string(), callback: do_quit },
        Command { keyword: "rand".to_string(), callback: do_rand },
        Command { keyword: "same".to_string(), callback: do_same },
        Command { keyword: "test".to_string(), callback: do_test },
        Command { keyword: "undo".to_string(), callback: do_undo },
        Command { keyword: "do ".to_string(), callback: do_do },
        Command { keyword: "ky0".to_string(), callback: do_ky0 },
        Command { keyword: "usi".to_string(), callback: do_usi },
        Command { keyword: "go".to_string(), callback: do_go },
        Command { keyword: "ky".to_string(), callback: do_ky },
    ];


    // [Ctrl]+[C] で強制終了
    loop{

        let mut line : String;
        if uchu.is_empty_command() {
            line = String::new();
        } else {
            // バッファーに溜まっていれば☆（＾～＾）
            line = uchu.pop_command();
            //g_writeln( &line );
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

        for element in command_array.iter() {
            if element.is_matched(len, &line, &starts) {
                element.move_caret_and_go(&mut uchu, len, &line, &mut starts);
                is_done = true;
                break;
            }
        }

        // 何とも一致しなかったら実行する。
        if !is_done {
            p_len_zero.move_caret_and_go(&mut uchu, len, &line, &mut starts);
        }

        if uchu.is_quit {
            // ループを抜けて終了
            break;
        }
    }//loop
}
