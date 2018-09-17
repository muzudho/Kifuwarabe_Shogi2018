/// ロガー。
use config::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
// use std::sync::Mutex;
use std::sync::RwLock;

/**
 * グローバル定数
 *
 * 使い方（lazy_static!マクロ）
 * ============================
 * 定数の値を実行時に決めることができる。
 *
 * Cargo.toml に１行追記
 * > [dependencies]
 * > lazy_static = "1.0.0"
 *
 * main.rs の冒頭あたりに次の２行を記述
 * > #[macro_use]
 * > extern crate lazy_static;
 *
 * 「How can I use mutable lazy_static?」
 * https://users.rust-lang.org/t/how-can-i-use-mutable-lazy-static/3751/3
 */
lazy_static! {
    /**
      * ログ・ファイル
    pub static ref LOGFILE: Mutex<File> = {
        // File::createの返り値は`io::Result<File>` なので .unwrap() で中身を取り出す
        Mutex::new(File::create(Path::new(LOG_FILE_PATH)).unwrap())
    };
      */

    pub static ref LOGGER: RwLock<Logger> = RwLock::new(Logger::new());
}


pub struct Logger {
    pub file_path: String,
    pub log_file: File
}
impl Logger {
    // FIXME 初回に要らないファイルを作ってしまう。
    pub fn new()->Logger {
        Logger {
            file_path: "log-kw2018-default.log".to_string(),
            log_file: File::create(Path::new(&"log-kw2018-default.log".to_string())).unwrap()
        }
    }

    pub fn set_file_path(&mut self, file_path2: &str) {
        self.file_path = file_path2.to_string();
        // File::createの返り値は`io::Result<File>` なので .unwrap() で中身を取り出す
        self.log_file = File::create(Path::new(&self.file_path)).unwrap();
    }    
} 

#[allow(dead_code)]
pub fn g_write(s:&str){
    println!("{}",s);
    if LOG_ENABLE {
        // write_allメソッドを使うには use std::io::Write; が必要
        if let Err(_why) = LOGGER.try_write().unwrap().log_file.write_all(s.as_bytes()) {}
        // 大会向けに、ログ書き込み失敗は出力しないことにする
        //panic!("couldn't write log. : {}",Error::description(&why)),
        /*
        // write_allメソッドを使うには use std::io::Write; が必要
        match LOGFILE.lock().unwrap().write_all(s.as_bytes()) {
            // 大会向けに、ログ書き込み失敗は出力しないことにする
            Err(_why) => {},//panic!("couldn't write log. : {}",Error::description(&why)),
            Ok(_) => {},
        }
         */
    }
}
#[allow(dead_code)]
pub fn g_writeln(s:&str){
    println!("{}",s);
    if LOG_ENABLE {
        if let Err(_why) = LOGGER.try_write().unwrap().log_file.write_all(format!("{}\n",s).as_bytes()) {
        }
    }
}
