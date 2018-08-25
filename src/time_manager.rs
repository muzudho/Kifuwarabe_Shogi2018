use searcher_impl::*;
use std::time::Duration;

/// マージン 1000ミリ秒。
const MARGIN_MILLISECONDS : i32 = 1000;

/// 思考時間の上限。
pub fn get_thought_max_milliseconds(milliseconds: i32) -> i32 {
    // 全体の 10分の1 を思考時間に充てる☆（＾ｑ＾）
    // マージン1秒は取っておく。
    let mut a = milliseconds / 10 - MARGIN_MILLISECONDS;

    // マージン1秒として、 30秒以内に収める。
    if 30_000 - MARGIN_MILLISECONDS < a {
        a = 30_000 - MARGIN_MILLISECONDS;
    }

    a
}

pub fn is_thought_timeout (searcher: &Searcher, end: Duration) -> bool {
    searcher.thought_max_milliseconds < (end.as_secs() * 1000) as i32 + (end.subsec_nanos() / 1_000_000_000) as i32
}
