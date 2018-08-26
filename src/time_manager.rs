use searcher_impl::*;
use std::time::Duration;

/// マージン 1000ミリ秒。
const MARGIN_MILLISECONDS : i32 = 1_000;
const LIMIT_MILLISECONDS : i32 = 20_000;
const MIN_MILLISECONDS : i32 = 1_000;

/// 思考時間の上限。
pub fn get_thought_max_milliseconds(milliseconds: i32) -> i32 {
    // 全体の 30分の1 を思考時間に充てる☆（＾ｑ＾）
    // 10分なら 600秒なので 1手20秒☆（＾～＾）
    let mut a = milliseconds / 30;

    // マージン1秒として、 20秒以内に収める。
    if LIMIT_MILLISECONDS - MARGIN_MILLISECONDS < a {
        a = LIMIT_MILLISECONDS - MARGIN_MILLISECONDS;
    }

    // とはいえ、1秒は使うようにする。
    if a < MIN_MILLISECONDS {
        a = MIN_MILLISECONDS;
    }

    a
}

pub fn is_thought_timeout (searcher: &Searcher, end: Duration) -> bool {
    searcher.thought_max_milliseconds < (end.as_secs() * 1000) as i32 + (end.subsec_nanos() / 1_000_000_000) as i32
}
