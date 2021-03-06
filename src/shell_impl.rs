// デバッグ出力。
const VERBOSE: bool = false;

use consoles;
use consoles::unit_test::*;
use consoles::visuals::dumps::*;
use consoles::visuals::title::*;
use display_impl::*;
use kifuwarabe_movement_picker::*;
use kifuwarabe_usi::*;
use rand::Rng;
use kifuwarabe_shell::diagram::ResponseOption;
use searcher_impl::*;
use std::collections::HashSet;
use thinks;
use thinks::think::*;
use tusin::us_conv::*;
use LOGGER;
use UCHU_WRAP;
use *;

// 任意のオブジェクト。
pub struct ShellVar {
    phase_milliseconds_array: [i32; 2],
    /// 探索部で使う。
    pub searcher: Searcher,
    /// エンジン設定。
    pub engine_settings: EngineSettings,
    /// position コマンド読み取り中に使う。盤。
    pub ban: [Piece; 100],
}
impl ShellVar {
    pub fn new() -> ShellVar {
        use kifuwarabe_usi::Piece::Space;
        ShellVar {
            phase_milliseconds_array: [0, 0],
            searcher: Searcher::new(),
            engine_settings: EngineSettings::new(),
            ban: [
                Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space,
                Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space,
                Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space,
                Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space,
                Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space,
                Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space,
                Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space,
                Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space,
                Space, Space, Space, Space,
            ],
        }
    }
}

/*****
 * C *
 *****/

/// 詰んでいたら真。
pub fn sub_cmate0(shell_var: &mut ShellVar) -> bool {
    // 探索時間は無制限(最大時間)。
    shell_var.phase_milliseconds_array[Sengo::Sen as usize] = <i32>::max_value();
    shell_var.phase_milliseconds_array[Sengo::Go as usize] = <i32>::max_value();

    // 自分の手番
    let turn_num = shell_var.searcher.game_record.get_teban(Jiai::Ji) as usize;

    // 自分の持ち時間。
    let milliseconds = shell_var.phase_milliseconds_array[turn_num];

    // 思考する。
    let bestmove = think(shell_var, milliseconds, 1);

    !bestmove.exists()
}

/// すでに詰んでいるかを調べる。
/// １手探索して投了すれば、すでに詰んでいると分かる。
///
pub fn do_cmate0(
    shell_var: &mut ShellVar,
    _req: &dyn Request,
    _res: &mut dyn Response,
) {
    LOGGER
        .try_write()
        .unwrap()
        .writeln("すでに詰んでいるかを調べる。");
    if sub_cmate0(shell_var) {
        LOGGER
            .try_write()
            .unwrap()
            .writeln("詰んでるぜ☆（＾～＾）ｖ");
    } else {
        LOGGER
            .try_write()
            .unwrap()
            .writeln("詰んでないぜ☆（ー＿－）");
    }
}

/// do_cmate0 を、ずっと続ける。
/// FIXME 強制終了する方法が今のところない。
///
pub fn do_cmate0auto(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    // let old_info_off = shell_var.searcher.info_off;
    shell_var.searcher.info_off = true;

    let mut trial: i64 = 0;
    loop {
        sub_rndpos(shell_var);
        if sub_cmate0(shell_var) {
            sub_ky(shell_var);
            LOGGER.try_write().unwrap().writeln(&format!(
                "詰んでるぜ☆（＾～＾）ｖ trial: {}.",
                trial
            ));
            trial = 0;
        } else {
            // 詰んでなければ無視。
            trial += 1;
        }
    }

    // shell_var.searcher.info_off = old_info_off;
}

/*****
 * D *
 *****/

/// 指し手を入れる。
pub fn do_do(
    shell_var: &mut ShellVar,
    req: &Request,
    _res: &mut dyn Response,
) {
    // コマンド読取。棋譜に追加され、手目も増える
    // let (successful, umov) = parse_movement(&req.get_line(), &mut res.get_caret(), req.get_line_len());
    let (successful, umov) = parse_movement(
        &req.get_line(),
        &mut req.get_caret(),
        req.get_line_len(),
    );
    let movement = usi_to_movement(successful, umov); // &umov

    shell_var.searcher.game_record.set_movement(movement);

    if successful {
        // 入っている指し手の通り指すぜ☆（＾～＾）
        let mut dummy_alpha = 0;
        userdefined_makemove(
            &mut shell_var.searcher,
            movement.to_hash(),
            &mut dummy_alpha,
        );
    }
}

/*****
 * G *
 *****/

/// 何手詰めかを調べる。
///
pub fn do_getmate(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    let mate = -1;
    LOGGER
        .try_write()
        .unwrap()
        .writeln(&format!("{}手詰め。", mate));
}

/// 思考を開始する。bestmoveコマンドを返却する。
///
/// ### 例。
/// go btime 60000 wtime 50000 byoyomi 10000
pub fn do_go(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    // 指定しなければ無制限。
    shell_var.phase_milliseconds_array[Sengo::Sen as usize] = <i32>::max_value();
    shell_var.phase_milliseconds_array[Sengo::Go as usize] = <i32>::max_value();
}

pub fn do_go_btime(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
}

pub fn do_go_btimevar(
    shell_var: &mut ShellVar,
    req: &Request,
    _res: &mut dyn Response,
) {
    let word = &req.get_groups()[0];
    let num: i32 = word.parse().unwrap();
    shell_var.phase_milliseconds_array[0] = num;
}

pub fn do_go_wtime(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
}

pub fn do_go_wtimevar(
    shell_var: &mut ShellVar,
    req: &Request,
    _res: &mut dyn Response,
) {
    let word = &req.get_groups()[0];
    let num: i32 = word.parse().unwrap();
    shell_var.phase_milliseconds_array[1] = num;
}

pub fn do_go_binc(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
}

pub fn do_go_bincvar(
    shell_var: &mut ShellVar,
    req: &Request,
    _res: &mut dyn Response,
) {
    let word = &req.get_groups()[0];
    let num: i32 = word.parse().unwrap();
    shell_var.phase_milliseconds_array[0] += num;
}

pub fn do_go_winc(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
}

pub fn do_go_wincvar(
    shell_var: &mut ShellVar,
    req: &Request,
    _res: &mut dyn Response,
) {
    let word = &req.get_groups()[0];
    let num: i32 = word.parse().unwrap();
    shell_var.phase_milliseconds_array[1] += num;
}

pub fn do_go_linebreak(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    // 自分の手番
    let turn_num = shell_var.searcher.game_record.get_teban(Jiai::Ji) as usize;

    // 自分の持ち時間。
    let milliseconds = shell_var.phase_milliseconds_array[turn_num];

    // 思考する。
    let max_depth = get_max_depth(shell_var);
    let bestmove = think(shell_var, milliseconds, max_depth);

    // 例： bestmove 7g7f
    LOGGER
        .try_write()
        .unwrap()
        .writeln(&format!("bestmove {}", movement_to_usi(&bestmove)));
}

/*****
 * H *
 *****/

/// 局面ハッシュ表示。
pub fn do_hash(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    let s = kaku_ky_hash(&shell_var.searcher.game_record);
    LOGGER.try_write().unwrap().writeln(&s);
}

/*****
 * I *
 *****/

/// USIプロトコル参照。
pub fn do_isready(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    LOGGER.try_write().unwrap().writeln("readyok");
}

/*****
 * K *
 *****/

/// 棋譜表示。
pub fn do_kifu(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    let s = kaku_kifu(&shell_var.searcher.game_record);
    LOGGER.try_write().unwrap().writeln(&s);
}

/// 利き数表示。
pub fn do_kikisu(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    consoles::commands::cmd_kikisu();
}

/// TODO 升と駒を指定して、移動先の確認。
pub fn do_kmmove(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
}

/// 駒の動きの確認。
pub fn do_kmugokidir(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    // 駒の動きの移動元として有りえる方角
    let kms = thinks::randommove::rnd_kms();
    LOGGER
        .try_write()
        .unwrap()
        .writeln(&format!("{}のムーブ元", &kms));
    uchu_r.hyoji_kmugoki_dir(*kms);
    LOGGER.try_write().unwrap().writeln(""); //改行
}

/// 駒の動き確認用。
pub fn do_kmugoki(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    // 読取許可モードで、ロック。
    let uchu_r = UCHU_WRAP.try_read().unwrap();

    // 駒の動きを出力
    uchu_r.hyoji_kmugoki();
}

/// 初期局面表示。
pub fn do_ky0(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    let s = kaku_ky(
        &shell_var.searcher.ini_position,
        &shell_var.searcher.game_record,
    );
    LOGGER.try_write().unwrap().writeln(&s);
}

pub fn sub_ky(shell_var: &mut ShellVar) {
    let s = kaku_ky(
        &shell_var.searcher.cur_position,
        &shell_var.searcher.game_record,
    );
    LOGGER.try_write().unwrap().writeln(&s);
}
/// 現局面表示。
pub fn do_ky(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    sub_ky(shell_var);
}

/*****
 * O *
 *****/

pub fn do_other(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    // 書込許可モードで、ロック。
    let mut uchu_w = UCHU_WRAP.try_write().unwrap();
    if uchu_w.title_dirty {
        // 空打ちでタイトル画面を出すなら☆（＾～＾）
        uchu_w.title_dirty = false;
        // タイトル表示
        // １画面は２５行だが、最後の２行は開けておかないと、
        // カーソルが２行分場所を取るんだぜ☆（＾～＾）
        hyoji_title();
    } else if uchu_w.console_game_mode {
        // 局面表示
        let s = kaku_ky(
            &shell_var.searcher.cur_position,
            &shell_var.searcher.game_record,
        );
        LOGGER.try_write().unwrap().writeln(&s);
    }
}

/*****
 * P *
 *****/

/// USIプロトコル参照。
pub fn do_position(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    //println!("Position: begin.");
    // 初期局面、現局面ともにクリアーします。手目も 0 に戻します。
    shell_var.searcher.ini_position.clear();
    shell_var.searcher.cur_position.clear();
    shell_var.searcher.game_record.set_teme(0);
}

fn sub_load_board(
    shell_var: &mut ShellVar,
) {
    //println!("Load board: begin.");
    // 盤面読取。
    for file in SUJI_1..SUJI_10 {
        for rank in DAN_1..DAN_10 {
            shell_var.searcher.ini_position.set_km_by_ms(
                suji_dan_to_ms(file, rank),
                pc_to_km(shell_var.ban[file_rank_to_cell(file, rank)]),
            );
        }
    }
    // 初期局面ハッシュを作り直す
    let hash_pos = shell_var.searcher.game_record.create_ky0_hash(&shell_var.searcher.ini_position);
    shell_var.searcher.game_record.set_ky0_hash(hash_pos);
    // 初期局面を、現局面に写す。
    shell_var.searcher.cur_position.set_all(&shell_var.searcher.ini_position);
}

/// USIプロトコル参照。
pub fn do_position_sfen_board(
    shell_var: &mut ShellVar,
    req: &Request,
    _res: &mut dyn Response,
) {
    //println!("Sfen board: begin.");
    let mut starts = 0;
    let line = &req.get_groups()[0];
    let len = line.chars().count();

    // position コマンド 盤上部分のみ 読取
    shell_var.ban = parse_board(&line, &mut starts, len);
    sub_load_board(shell_var);
}

/// USIプロトコル参照。
pub fn do_position_sfen_hands(
    shell_var: &mut ShellVar,
    req: &Request,
    _res: &mut dyn Response,
) {
    //println!("Sfen hands: begin.");
    let mut starts = 0;
    let line = &req.get_groups()[0];
    let len = line.chars().count();

    // 持ち駒数。増減させたいので、u8 ではなく i8。
    let hand_count_arr: [i8; HAND_PIECE_ARRAY_LN] = parse_hand_piece(&line, &mut starts, len);
    // 持ち駒数読取。
    for (i, item) in HAND_PIECE_ARRAY.iter().enumerate() {
        let km = pc_to_km(*item);

        shell_var
            .searcher
            .ini_position
            .set_mg(km, hand_count_arr[i]);
    }
}

/// USIプロトコル参照。
/// 指し手１つ分のパース。
pub fn do_position_sfen_movevar(
    shell_var: &mut ShellVar,
    req: &Request,
    _res: &mut dyn Response,
) {
    //println!("Sfen movevar: begin.");
    let mut starts = 0;
    let line = &req.get_groups()[0];
    let len = line.chars().count();

    // 指し手を1つずつ返すぜ☆（＾～＾）
    let (successful, usi_movement) = parse_movement(&line, &mut starts, len);
    let movement = usi_to_movement(successful, usi_movement);
    // 棋譜に書き込み。
    shell_var.searcher.game_record.set_movement(movement);
    if successful {
        // 指し手が付いていれば、指し手を指すぜ☆（＾～＾）
        let mut dummy_alpha = 0;
        userdefined_makemove(&mut shell_var.searcher, movement.to_hash(), &mut dummy_alpha);
    }
    //println!("Sfen movevar: end.");
}

/// USIプロトコル参照。
pub fn do_position_startpos(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    //println!("Position startpos: begin.");
    // 別途用意した平手初期局面文字列を読取
    let mut local_starts = 0;

    // position コマンド 盤上部分のみ 読取
    shell_var.ban = parse_board(&STARTPOS.to_string(), &mut local_starts, STARTPOS_LN);
    sub_load_board(shell_var);
    //println!("Position startpos: end.");
}

/*****
 * Q *
 *****/

/// 終了。
pub fn do_quit(
    _shell_var: &mut ShellVar,
    _req: &Request,
    res: &mut dyn Response,
) {
    res.set_option(ResponseOption::Quits);
}

/*****
 * R *
 *****/

/// 乱数の試し確認。
pub fn do_rand(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    let secret_number = rand::thread_rng().gen_range(1, 101); //1~100
    LOGGER
        .try_write()
        .unwrap()
        .writeln(&format!("乱数={}", secret_number));
}

pub fn do_reload(
    _shell_var: &mut ShellVar,
    _req: &Request,
    res: &mut dyn Response,
) {
    res.set_option(ResponseOption::Reloads(GRAPH_JSON_FILE.to_string()));
}

/// 駒種類をランダムで出す。
pub fn do_rndkms(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    let kms = thinks::randommove::rnd_kms();
    LOGGER
        .try_write()
        .unwrap()
        .writeln(&format!("乱駒種類={}", &kms));
}

/// マスをランダムで返す。
pub fn do_rndms(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    let ms = thinks::randommove::rnd_ms();
    LOGGER
        .try_write()
        .unwrap()
        .writeln(&format!("乱升={}", ms));
}

pub fn sub_rndpos(shell_var: &mut ShellVar) {
    // 手目を 0 に戻す。
    shell_var.searcher.game_record.set_teme(0);

    let mut pos = shell_var.searcher.ini_position;
    // 盤上の駒をシャッフルする。
    for _i in 0..1000 {
        // ランダムな升を２つ。
        let ms_dst = thinks::randommove::rnd_ms();
        let ms_src = thinks::randommove::rnd_ms();

        // その駒が２つ。
        let km_dst = pos.get_km_by_ms(ms_dst);
        let km_src = pos.get_km_by_ms(ms_src);
        // LOGGER.try_write().unwrap().writeln( &format!( "{}{}<---->{}{}", ms_src, km_src, ms_dst, km_dst) );

        // 入れ替え。
        pos.set_km_by_ms(ms_dst, km_src);
        pos.set_km_by_ms(ms_src, km_dst);
    }

    // 初期局面ハッシュを作り直す
    let hash_pos = shell_var.searcher.game_record.create_ky0_hash(&pos);
    shell_var.searcher.game_record.set_ky0_hash(hash_pos);

    // 初期局面を、現局面に写す。
    shell_var.searcher.cur_position.set_all(&pos);
}

/// ランダムな初期局面を作る。
pub fn do_rndpos(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    LOGGER
        .try_write()
        .unwrap()
        .writeln(&"ランダムな初期局面を作る。");
    sub_rndpos(shell_var);
}

/*****
 * S *
 *****/

/// 同一局面回数調べ。
pub fn do_same(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    LOGGER.try_write().unwrap().writeln(&format!(
        "同一局面調べ count={}",
        shell_var.searcher.game_record.count_same_ky()
    ));
}

/// 合法手を確認する。
pub fn do_sasite(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    // FIXME 合法手とは限らない
    let mut ss_potential_hashset = HashSet::new();

    insert_picked_movement(
        &shell_var.searcher.cur_position,
        &shell_var.searcher.game_record,
        &mut ss_potential_hashset,
        &mut shell_var.searcher.movepicker_hashset_work,
        &mut shell_var.searcher.movepicker_hashset_result,
        &mut shell_var.searcher.movepicker_hashset_drop,
    );
    LOGGER
        .try_write()
        .unwrap()
        .writeln("----指し手生成 ここから----");
    hyoji_ss_hashset(&ss_potential_hashset);
    LOGGER
        .try_write()
        .unwrap()
        .writeln("----指し手生成 ここまで----");
}

/// USI
pub fn do_setoption(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    if VERBOSE {
        println!("#Setoption: begin.");
    }
    if VERBOSE {
        println!("#Setoption: end.");
    }
}
pub fn do_setoption_name(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    if VERBOSE {
        println!("#Setoption name: begin.");
    }
    if VERBOSE {
        println!("#Setoption name: end.");
    }
}
pub fn do_setoption_namevar(
    shell_var: &mut ShellVar,
    req: &dyn Request,
    _res: &mut dyn Response,
) {
    let name = &req.get_groups()[0];
    if VERBOSE {
        println!("#Setoption namevar: begin. [{}]", name);
    }
    shell_var.engine_settings.buffer_name = name.to_string();
    if VERBOSE {
        println!("#Setoption namevar: end.");
    }
}
pub fn do_setoption_value(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    if VERBOSE {
        println!("#Setoption value: begin.");
    }
    if VERBOSE {
        println!("#Setoption value: end.");
    }
}
pub fn do_setoption_valuevar(
    shell_var: &mut ShellVar,
    req: &dyn Request,
    _res: &mut dyn Response,
) {
    let value = &req.get_groups()[0];
    if VERBOSE {
        println!("#Setoption valuevar: begin. [{}]", value);
    }
    shell_var.engine_settings.buffer_value = value.to_string();
    if VERBOSE {
        println!("#Setoption valuevar: end.");
    }
}
pub fn do_setoption_linebreak(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    if VERBOSE {
        println!("#Setoption linebreak: begin.");
    }
    shell_var.engine_settings.flush();
    if VERBOSE {
        println!("#Setoption linebreak: end.");
    }
}

/*****
 * T *
 *****/

/// convのテスト。
pub fn do_teigi_conv(
    _shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    for ms in 11..19 {
        for hash in 0..10 {
            let next = push_ms_to_hash(hash, ms);
            let (hash_orig, ms_orig) = pop_ms_from_hash(next);
            LOGGER.try_write().unwrap().writeln(&format!(
                "push_ms_to_hash(0b{:4b},0b{:5b})=0b{:11b} pop_ms_from_hash(...)=(0b{:4b},0b{:5b})",
                hash, ms, next, hash_orig, ms_orig
            ));
        }
    }
}

/// いろいろな動作テストをしたいときに汎用的に使う。
pub fn do_test(
    shell_var: &mut ShellVar,
    req: &Request,
    _res: &mut dyn Response,
) {
    LOGGER.try_write().unwrap().writeln(&format!(
        "test caret={} len={}",
        req.get_caret(),
        req.get_line_len()
    ));
    // test(&shell_var.searcher, &req.get_line(), &mut res.get_caret(), req.get_line_len());
    test(
        &shell_var.searcher,
        &req.get_line(),
        &mut req.get_caret(),
        req.get_line_len(),
    );
}

/*****
 * U *
 *****/

/// 指した手を１手戻す。
pub fn do_undo(
    shell_var: &mut ShellVar,
    _req: &Request,
    _res: &mut dyn Response,
) {
    let (successful, _cap_kms) = unmakemove(&mut shell_var.searcher);

    if !successful {
        let teme = shell_var.searcher.game_record.teme;
        LOGGER
            .try_write()
            .unwrap()
            .writeln(&format!("teme={} を、これより戻せません", teme));
    }
}
