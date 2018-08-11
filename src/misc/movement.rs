use CUR_POSITION_WRAP;
use GAME_RECORD_WRAP;
use INI_POSITION_WRAP;
use kifuwarabe_position::*;

/// 初期局面ハッシュを作り直す。先後込み。
pub fn create_ky0_hash() -> u64 {
    let game_record = GAME_RECORD_WRAP.try_read().unwrap();
    let hash_seed = &game_record.ky_hash_seed;
    let mut hash : u64;
    // グローバル変数を使う。
    {
        hash = INI_POSITION_WRAP.try_read().unwrap().create_hash(&hash_seed);
    }

    // 手番ハッシュ（後手固定）
    hash ^= hash_seed.sn[SN_GO];

    hash
}

/// 局面ハッシュを作り直す。先後込み。
pub fn create_ky1_hash() -> u64 {
    let game_record = GAME_RECORD_WRAP.try_read().unwrap();
    let hash_seed = &game_record.ky_hash_seed;
    let mut hash : u64;
    // グローバル変数を使う。
    {
        hash = CUR_POSITION_WRAP.try_read().unwrap().create_hash(&hash_seed);
    }

    use kifuwarabe_position::Sengo::*;
    match game_record.get_teban(&Jiai::Ji) {
        Sen => { hash ^= hash_seed.sn[SN_SEN] },
        Go => { hash ^= hash_seed.sn[SN_GO] },
        _ => {},
    }

    hash
}
