/// 思考時間の上限。
pub fn get_thought_max_milliseconds(milliseconds: u64) -> u64 {
    let mut a = milliseconds / 2;

    // マージン1秒として、 30秒以内に収める。
    if 29_000 < a {
        a = 29_000;
    }

    a
}