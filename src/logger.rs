/// ロガー。
// use config::*;
// use chrono::{DateTime, TimeZone, NaiveDateTime, UTC};
use chrono::prelude::*; // DateTime<Local>
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
// use std::sync::Mutex;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH}; //Duration
use time::Duration;

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
    // FIXME 初回に要らないファイルを作ってしまう。
    pub fn new()->Logger {
        Logger {
            directory: "./logs/".to_string(),
            base_file_name: "log-default".to_string(),
            extension: ".log".to_string(),
            file_path: "log-default-YYYY-MM-DD.log".to_string(),
            chrono: Local::now(),
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

    /// TODO 古いログファイルを消す。
    pub fn delete_old_file(&self) {
        // TODO ファイルを一覧する。
        // let mut files: Vec<String> = Vec::new();
        for path in fs::read_dir(&self.directory).unwrap() {
            use std::fs;

            let path_str = path.unwrap().path().display().to_string();
            //files.push(path_str);
            let metadata = fs::metadata(path_str.to_string());
            // println!("metadata: {:?}", metadata);

            if let Ok(unix_time) = metadata.unwrap().modified() {
                // TODO println!("logger.rs: metadata.modified: {:?}", unix_time);
                let local_time = self.system_time_to_date_time(unix_time);
                // TODO println!("logger.rs: local_time: {:?}", local_time);

                let today = Local::now();

                if local_time < today - Duration::days(10) {
                    // TODO println!("logger.rs: I want remove file. {}", path_str.to_string());
                    // fs::remove_file("a.txt")?;
                //} else {
                    //println!("path_str: {:?}", path_str);
                }
            } else {
                // TODO println!("logger.rs: Not supported on this platform");
            }
            
        }

        /*
        files.sort();
        let strings = files.iter()
            .fold(String::new(), |joined, s| {
                if joined == String::new() { s.to_string() } else { joined + "  " + s }
            });

        println!("{}", strings)        
        */
        // TODO 日付を確認する。
        // TODO 10日前のファイルなら消す。
    }

    pub fn set_file_path(&mut self, base_file_name2: &str, extension2: &str) {
        self.base_file_name = base_file_name2.to_string();
        self.extension = extension2.to_string();
        self.chrono = Local::now();
        // File::createの返り値は`io::Result<File>` なので .unwrap() で中身を取り出す
        self.file_path = format!("{}{}-{:04}-{:02}-{:02}{}", self.directory, self.base_file_name, self.chrono.year(), self.chrono.month(), self.chrono.day(), self.extension).to_string();
        self.log_file = File::create(Path::new(&self.file_path)).unwrap();
    }    

    pub fn refresh_filepath(&mut self) {
        let local = Local::now();
        if self.chrono.day() != local.day() || self.chrono.month() != local.month() || self.chrono.year() == local.year() {
            // 日付が変わっていれば更新。
            self.chrono = local;
            self.file_path = format!("{}{}-{:04}-{:02}-{:02}{}", self.directory, self.base_file_name, self.chrono.year(), self.chrono.month(), self.chrono.day(), self.extension).to_string();
        }
        self.log_file = File::create(Path::new(&self.file_path)).unwrap();
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

