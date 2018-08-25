/// マージン 1000ミリ秒。
const MARGIN_MILLISECONDS : u64 = 1000;

/// 思考時間の上限。
pub fn get_thought_max_milliseconds(milliseconds: u64) -> u64 {
    // マージン1秒は取っておく。
    let mut a = milliseconds / 2 - MARGIN_MILLISECONDS;

    // マージン1秒として、 30秒以内に収める。
    if 30_000 - MARGIN_MILLISECONDS < a {
        a = 30_000 - MARGIN_MILLISECONDS;
    }

    a
}