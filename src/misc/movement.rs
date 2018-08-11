use CUR_POSITION_WRAP;
use GAME_RECORD_WRAP;
use INI_POSITION_WRAP;
use kifuwarabe_movement::*;
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

/// 入れた指し手の通り指すぜ☆（＾～＾）
/// callback(取った駒)
pub fn make_movement2<F1>(movement: &Movement, mut callback: F1)
    where F1 : FnMut(&KmSyurui)
{
    // 取った駒を記録するために、棋譜に入れる☆

    let teme: usize;
    let cap;
    let sn;
    {
        let game_record = GAME_RECORD_WRAP.try_read().unwrap();
        sn = game_record.get_teban(&Jiai::Ji);
    }

    {
        let mut position = CUR_POSITION_WRAP.try_write().unwrap();
        cap = make_movement(&sn, movement, &mut position);
    }

    {
        let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();
        teme = game_record.teme;
        game_record.moves[teme] = *movement;
        game_record.set_cap(teme, cap);
    }

    // 局面ハッシュを作り直す
    let ky_hash = create_ky1_hash();

    {
        let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();
        game_record.set_ky1_hash( ky_hash );
        game_record.teme += 1;
    }

    callback(&km_to_kms(&cap));
}

pub fn unmake_movement2<F1>(mut callback: F1) -> bool
    where F1 : FnMut(&KmSyurui)
{
    let mut teme: usize;
    {
        teme = GAME_RECORD_WRAP.try_write().unwrap().teme;
    }

    if 0 < teme {
        // 棋譜から読取、手目も減る
        let cap;
        let sn;
        let ss;
        {
            teme -= 1;
            let mut game_record = GAME_RECORD_WRAP.try_write().unwrap();
            game_record.teme = teme;
            cap = game_record.cap[teme];
            sn = game_record.get_teban(&Jiai::Ji);
            ss = game_record.get_sasite();
        }

        {
            let mut position = CUR_POSITION_WRAP.try_write().unwrap();
            unmake_movement(&sn, &ss, &cap, &mut position);
        }

        callback(&km_to_kms(&cap));

        // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
        true
    } else {
        false
    }
}

/// 千日手を調べるために、
/// 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
pub fn count_same_ky() -> i8 {
    let mut count = 0;
    {
        let game_record = &GAME_RECORD_WRAP.try_read().unwrap();
        if game_record.get_teme() < 1 { return 0; }
        let last_teme = game_record.get_teme() - 1;
        let new_teme = game_record.get_teme();
        // g_writeln( &format!( "Ｃount_same_ky last_teme={} new_teme={}", last_teme ,new_teme ) );
        for i_teme in 0..new_teme {
            let t = last_teme - i_teme;
            // g_writeln( &format!( "i_teme={} t={}", i_teme, t ) );
            if game_record.ky_hash[t] == game_record.ky_hash[last_teme] {
                count+=1;
            }
        }

        // 初期局面のハッシュ
        if game_record.ky0_hash == game_record.ky_hash[last_teme] {
            count+=1;
        }
    }

    count
}
