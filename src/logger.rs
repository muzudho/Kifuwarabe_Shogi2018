/// ロガー。
// use config::*;
// use chrono::{DateTime, TimeZone, NaiveDateTime, UTC};
use chrono::prelude::*; // DateTime<Local>
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
// use std::sync::Mutex;
use std::ffi::OsStr;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH}; //Duration
use time::Duration;
use std::fs::OpenOptions;

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

    /// # Examples.
    /// 
    /// ```
    /// LOGGER.try_write().unwrap().writeln("Hello!");
    /// ```
    pub static ref LOGGER: RwLock<Logger> = RwLock::new(Logger::new());
}


pub struct Logger {
    pub directory: String,
    pub base_file_name: String,
    pub extension: String,
    pub file_path: String,
    pub chrono: DateTime<Local>,
    pub log_file: File,
    pub enable: bool,
}
impl Logger {
    pub fn new()->Logger {
        Logger {
            directory: "./logs/".to_string(),
            base_file_name: "log-default".to_string(),
            extension: ".log".to_string(),
            file_path: "log-default-YYYY-MM-DD.log".to_string(),
            chrono: Local::now(),
            // FIXME 初回に要らないファイルを作ってしまう。
            log_file: File::create(Path::new(&"log-default-YYYY-MM-DD.log".to_string())).unwrap(),
            enable: true,
        }
    }

    /// https://users.rust-lang.org/t/convert-std-time-systemtime-to-chrono-datetime-datetime/7684/2
    pub fn system_time_to_date_time(&self, t: SystemTime) -> DateTime<Local> { //Utc
        let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
            Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
            Err(e) => { // unlikely but should be handled
                let dur = e.duration();
                let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
                if nsec == 0 {
                    (-sec, 0)
                } else {
                    (-sec - 1, 1_000_000_000 - nsec)
                }
            },
        };
        Local.timestamp(sec, nsec) // Utc
    }

    /// 古いログファイルを消す。
    pub fn delete_old_file(&self, days: i64) {
        let today = Local::now();
        for path in fs::read_dir(&self.directory).unwrap() {
            use std::fs;

            let path_str = path.unwrap().path().display().to_string();
            let metadata = fs::metadata(path_str.to_string());

            match metadata.unwrap().modified() {
                Ok(unix_time) => {
                    let local_time = self.system_time_to_date_time(unix_time);
                    // 何日か前のファイルなら消す。
                    if local_time < today - Duration::days(days) {
                        let file_path = Path::new(&path_str);
                        // 拡張子を見て、ログファイルであることを確かめる。
                        if file_path.extension() == Some(OsStr::new("log")) {
                            match fs::remove_file(path_str.to_string()) {
                                Ok(_n) => {},
                                Err(err) => panic!("Remove log file error. {}", err),
                            };
                            // USIで メッセージがでると反則負けになってしまう。
                            // println!("Removed: {}", path_str.to_string());
                        }
                    }
                },
                Err(err) => {
                    panic!("logger.rs: Metadata not supported on this platform. {}", err);
                }
            }            
        }
    }

    pub fn set_file_path(&mut self, base_file_name2: &str, extension2: &str) {
        self.base_file_name = base_file_name2.to_string();
        self.extension = extension2.to_string();
        self.chrono = Local::now();
        // File::createの返り値は`io::Result<File>` なので .unwrap() で中身を取り出す
        self.file_path = format!("{}{}-{:04}-{:02}-{:02}{}", self.directory, self.base_file_name, self.chrono.year(), self.chrono.month(), self.chrono.day(), self.extension).to_string();
        // 追加書き込み。
        self.log_file = match OpenOptions::new().create(true).append(true).open(&self.file_path) {
            Ok(file) => file,
            Err(err) => panic!("Log file open error. {}", err),
        };
    }    

    pub fn refresh_filepath(&mut self) {
        let local = Local::now();
        if self.chrono.day() != local.day() || self.chrono.month() != local.month() || self.chrono.year() == local.year() {
            // 日付が変わっていれば更新。
            self.chrono = local;
            self.file_path = format!("{}{}-{:04}-{:02}-{:02}{}", self.directory, self.base_file_name, self.chrono.year(), self.chrono.month(), self.chrono.day(), self.extension).to_string();
        }
        // 追加書き込み。
        self.log_file = match OpenOptions::new().create(true).append(true).open(&self.file_path) {
            Ok(file) => file,
            Err(err) => panic!("Log file open error. {}", err),
        };
    }

    #[allow(dead_code)]
    pub fn write(&mut self, s:&str){
        println!("{}",s);
        if self.enable {
            self.refresh_filepath();
            // write_allメソッドを使うには use std::io::Write; が必要
            if let Err(_why) = self.log_file.write_all(s.as_bytes()) {}
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

    pub fn writeln(&mut self, s:&str){
        println!("{}",s);
        if self.enable {
            self.refresh_filepath();
            if let Err(_why) = self.log_file.write_all(format!("{}\n",s).as_bytes()) {
            }
        }
    }
} 

