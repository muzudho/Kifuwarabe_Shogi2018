extern crate rand;

use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use tusin::us_conv::*;

pub fn kaku_ky_hash(game_record: &GameRecord)->String{
    let mut s = String::new();
    let teme: usize;
    {
        s.push_str(&format!("[ini] {:20}\n", &game_record.ky0_hash ));
        teme = game_record.teme;
    }
    for i_teme in 0..teme {
        let hash = &game_record.ky_hash[i_teme];
        // 64bitは10進数20桁。改行する
        s.push_str(&format!("[{:3}] {:20}\n", i_teme, hash));
    }
    s
}

/// # Examples.
///
/// let s = kaku_kifu();
/// g_writeln( &s );
pub fn kaku_kifu(game_record: &GameRecord) -> String {
    let mut s = String::new();
    let teme: usize = game_record.teme;
    for i_teme in 0..teme {
        let ss = game_record.moves[i_teme];
        s.push_str(&format!("[{}] {}", i_teme, movement_to_usi(&ss)));
    }
    s
}

/**
    * 表示
    *
    * 後手から見た盤を表示するぜ☆（＾～＾）
    * デカルト座標の第一象限と x,y 方向が一致するメリットがあるぜ☆（＾～＾）
    */
pub fn kaku_ky(position: &Position, game_record: &GameRecord) -> String {

    let teme = game_record.get_teme();
    let teban = game_record.get_teban(Jiai::Ji);
    let same = game_record.count_same_ky();

    let ky = position;
    // 局面表示
    format!("\
表示 {95}手目 {96} 同一局面{97}回目

           +----+----+----+----+----+----+----+----+----+
        i9 |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|
           +----+----+----+----+----+----+----+----+----+
ひx{87:2}   h8 |{9}|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|   ヒx{94:2}
           +----+----+----+----+----+----+----+----+----+
しx{86:2}   g7 |{18}|{19}|{20}|{21}|{22}|{23}|{24}|{25}|{26}|   シx{93:2}
           +----+----+----+----+----+----+----+----+----+
うx{85:2}   f6 |{27}|{28}|{29}|{30}|{31}|{32}|{33}|{34}|{35}|   ウx{92:2}
           +----+----+----+----+----+----+----+----+----+
ねx{84:2}   e5 |{36}|{37}|{38}|{39}|{40}|{41}|{42}|{43}|{44}|   ネx{91:2}
           +----+----+----+----+----+----+----+----+----+
いx{83:2}   d4 |{45}|{46}|{47}|{48}|{49}|{50}|{51}|{52}|{53}|   イx{90:2}
           +----+----+----+----+----+----+----+----+----+
ぞx{82:2}   c3 |{54}|{55}|{56}|{57}|{58}|{59}|{60}|{61}|{62}|   ゾx{89:2}
           +----+----+----+----+----+----+----+----+----+
きx{81:2}   b2 |{63}|{64}|{65}|{66}|{67}|{68}|{69}|{70}|{71}|   キx{88:2}
           +----+----+----+----+----+----+----+----+----+
▼      a1 |{72}|{73}|{74}|{75}|{76}|{77}|{78}|{79}|{80}|   △
           +----+----+----+----+----+----+----+----+----+
            1    2    3    4    5    6    7    8    9\
",
        ky.get_km_by_ms(19),ky.get_km_by_ms(29),ky.get_km_by_ms(39),ky.get_km_by_ms(49),ky.get_km_by_ms(59),ky.get_km_by_ms(69),ky.get_km_by_ms(79),ky.get_km_by_ms(89),ky.get_km_by_ms(99),
        ky.get_km_by_ms(18),ky.get_km_by_ms(28),ky.get_km_by_ms(38),ky.get_km_by_ms(48),ky.get_km_by_ms(58),ky.get_km_by_ms(68),ky.get_km_by_ms(78),ky.get_km_by_ms(88),ky.get_km_by_ms(98),
        ky.get_km_by_ms(17),ky.get_km_by_ms(27),ky.get_km_by_ms(37),ky.get_km_by_ms(47),ky.get_km_by_ms(57),ky.get_km_by_ms(67),ky.get_km_by_ms(77),ky.get_km_by_ms(87),ky.get_km_by_ms(97),
        ky.get_km_by_ms(16),ky.get_km_by_ms(26),ky.get_km_by_ms(36),ky.get_km_by_ms(46),ky.get_km_by_ms(56),ky.get_km_by_ms(66),ky.get_km_by_ms(76),ky.get_km_by_ms(86),ky.get_km_by_ms(96),
        ky.get_km_by_ms(15),ky.get_km_by_ms(25),ky.get_km_by_ms(35),ky.get_km_by_ms(45),ky.get_km_by_ms(55),ky.get_km_by_ms(65),ky.get_km_by_ms(75),ky.get_km_by_ms(85),ky.get_km_by_ms(95),
        ky.get_km_by_ms(14),ky.get_km_by_ms(24),ky.get_km_by_ms(34),ky.get_km_by_ms(44),ky.get_km_by_ms(54),ky.get_km_by_ms(64),ky.get_km_by_ms(74),ky.get_km_by_ms(84),ky.get_km_by_ms(94),
        ky.get_km_by_ms(13),ky.get_km_by_ms(23),ky.get_km_by_ms(33),ky.get_km_by_ms(43),ky.get_km_by_ms(53),ky.get_km_by_ms(63),ky.get_km_by_ms(73),ky.get_km_by_ms(83),ky.get_km_by_ms(93),
        ky.get_km_by_ms(12),ky.get_km_by_ms(22),ky.get_km_by_ms(32),ky.get_km_by_ms(42),ky.get_km_by_ms(52),ky.get_km_by_ms(62),ky.get_km_by_ms(72),ky.get_km_by_ms(82),ky.get_km_by_ms(92),
        ky.get_km_by_ms(11),ky.get_km_by_ms(21),ky.get_km_by_ms(31),ky.get_km_by_ms(41),ky.get_km_by_ms(51),ky.get_km_by_ms(61),ky.get_km_by_ms(71),ky.get_km_by_ms(81),ky.get_km_by_ms(91),
        //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
        ky.mg[Koma::K0 as usize],ky.mg[Koma::Z0 as usize],ky.mg[Koma::I0 as usize],ky.mg[Koma::N0 as usize],ky.mg[Koma::U0 as usize],ky.mg[Koma::S0 as usize],ky.mg[Koma::H0 as usize],
        //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
        ky.mg[Koma::K1 as usize],ky.mg[Koma::Z1 as usize],ky.mg[Koma::I1 as usize],ky.mg[Koma::N1 as usize],ky.mg[Koma::U1 as usize],ky.mg[Koma::S1 as usize],ky.mg[Koma::H1 as usize],
        teme, teban, same
    )
}
