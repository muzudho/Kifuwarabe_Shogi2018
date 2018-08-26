/// 指し手生成☆（＾～＾）

/*
 * 現局面を使った指し手生成
 */

// use consoles::asserts::*;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use searcher_impl::*;
use std::collections::HashSet;
use std::fmt;
// use teigi::shogi_syugo::*;
use teigi::conv::*;













/****************
 * 駒の動く方向 *
 ****************/
/**
 * 後手から見た盤を想像すること。筋、段を第一象限と同じ向きに合わせる。
 * 駒が戻る方向10方向。東から反時計回り。boolは長い利きなら真
 */
#[derive(Clone)]
pub enum KmDir{
    // 東
    E(bool),
    // 北東
    NE(bool),
    // 北北東（桂馬が戻る動き）
    NNE,
    // 北
    N(bool),
    // 北北西（桂馬が戻る動き）
    NNW,
    // 北西
    NW(bool),
    // 西
    W(bool),
    // 南西
    SW(bool),
    // 南南西（桂馬の動き）
    SSW,
    // 南
    S(bool),
    // 南南東（桂馬の動き）
    SSE,
    // 南東
    SE(bool),
    // 要素数より1小さい数。エラー値用に使っても可
    Owari
}
impl fmt::Display for KmDir{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use movement_picker::KmDir::*;
        match *self{
            E(b)        => if b { write!(f,"長東")  }else{ write!(f,"東")  },
            NE(b)       => if b { write!(f,"長北東")}else{ write!(f,"北東")},
            NNE         => {write!(f,"北北東")},
            N(b)        => if b { write!(f,"長北")  }else{ write!(f,"北")  },
            NNW         => { write!(f,"北北西")},
            NW(b)       => if b { write!(f,"長北西")}else{ write!(f,"北西")},
            W(b)        => if b { write!(f,"長西")  }else{ write!(f,"西")  },
            SW(b)       => if b { write!(f,"長南西")}else{ write!(f,"南西")},
            SSW         => { write!(f,"南南西")},
            S(b)        => if b { write!(f,"長南")  }else{ write!(f,"南")  },
            SSE         => { write!(f,"南南東")},
            SE(b)       => if b { write!(f,"長南東")}else{ write!(f,"南東")},
            Owari       => { write!(f,"×")},
        }
    }
}


/************
 * 駒の動き *
 ************/
// 駒が戻る動き
#[allow(dead_code)]
pub struct KmUgoki{
    // 駒種類ごとに、駒の動きを保持。動ける方向は、駒ごとに可変長配列
    pub back:[[KmDir;KM_UGOKI_LN];KMS_LN]
}
/**
 * 駒が戻る動き。投了図から現局面へ逆向きに指す思想。
 * [駒種類][9]
 *
 * （１）この表は、後手から盤面を見たものを想像する。
 * （２）後手から見て、普通に駒の動きが　登録されている。
 *       先手から見たとき、back （後ろ向きの動き）となる。
 */
pub const KM_UGOKI : KmUgoki = KmUgoki{
    back:[
        // 東,北東,北,北西,西,南西,南南西,南,南南東,南東,終わり
        /*ら  */ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),KmDir::SW(false),KmDir::S(false),KmDir::SE(false),KmDir::Owari],
        /*き  */ [KmDir::E(true ),                            KmDir::N(true ),                            KmDir::W(true ),                 KmDir::S(true ),                 KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぞ  */ [                KmDir::NE(true ),                                      KmDir::NW(true ),                KmDir::SW(true ),                KmDir::SE(true ),KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*い  */ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ね  */ [                KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),                KmDir::SW(false),                KmDir::SE(false),KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*う  */ [                                 KmDir::NNE,                KmDir::NNW                 ,                                                                  KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*し  */ [                                            KmDir::N(true )                            ,                                                                  KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ひ  */ [                                            KmDir::N(false)                            ,                                                                  KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぱき*/ [KmDir::E(true ),KmDir::NE(false),           KmDir::N(true ),           KmDir::NW(false),KmDir::W(true ),KmDir::SW(false),KmDir::S(true ),KmDir::SE(false),KmDir::Owari],
        /*ぱぞ*/ [KmDir::E(false),KmDir::NE(true ),           KmDir::N(false),           KmDir::NW(true ),KmDir::W(false),KmDir::SW(true ),KmDir::S(false),KmDir::SE(true ),KmDir::Owari],
        /*ぱね*/ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぱう*/ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぱし*/ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぱひ*/ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*空升*/ [                                                                                                                                                          KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*終り*/ [                                                                                                                                                          KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
    ]
};




















/**
 * 現局面の、任意の移動先升の、
 * - 盤上の駒の移動
 * - 打
 * の指し手を生成。
 *
 * 王手回避漏れや、千日手などのチェックは行っていない
 */
pub fn insert_picked_movement(searcher: &Searcher, ss_hashset:&mut HashSet<u64>) {
    // +----------------+
    // | 盤上の駒の移動 |
    // +----------------+

    // 移動元の升をスキャンする。
    for dan_src in 1..10 {
        for suji_src in 1..10 {

            let ms_src = suji_dan_to_ms( suji_src, dan_src );
            let km_src = searcher.cur_position.get_km_by_ms( ms_src );
            let sn = km_to_sn(&km_src);

            let sn1 = searcher.game_record.get_teban(&Jiai::Ji);

            if match_sn(&sn, &sn1) {
                // 手番の駒

                // [成らず]

                let mut dst_hashset : HashSet<umasu> = HashSet::new();
                // 升と駒から、移動しようとする先を返す。
                insert_dst_by_ms_km(ms_src, &km_src,
                    false, // 成らず
                    &mut dst_hashset,
                    &searcher.cur_position);

                // g_writeln("テスト ポテンシャルムーブ insert_dst_by_ms_km(成らず).");
                // use consoles::visuals::dumps::*;
                // hyoji_ms_hashset( &dst_hashset );

                for ms_dst in &dst_hashset {
                    // 自-->至 の arrow を作成。
                    ss_hashset.insert( Movement{
                        source: ms_src,
                        destination: *ms_dst,
                        promotion: false, // 成らず
                        drop: KmSyurui::Kara,
                    }.to_hash() );
                }

                // [成り]

                dst_hashset.clear();
                insert_dst_by_ms_km(ms_src, &km_src,
                    true, // 成り
                    &mut dst_hashset,
                    &searcher.cur_position);

                for ms_dst in &dst_hashset {
                    // 自-->至 の arrow を作成。
                    ss_hashset.insert( Movement{
                        source: ms_src,
                        destination: *ms_dst,
                        promotion: true, // 成り
                        drop: KmSyurui::Kara,
                    }.to_hash() );
                }
            }
        }
    }

    // +----+
    // | 打 |
    // +----+
    for dan_dst in 1..10 {
        for suji_dst in 1..10 {
            let ms_dst = suji_dan_to_ms( suji_dst, dan_dst );
            let km_dst = searcher.cur_position.get_km_by_ms( ms_dst );
            match km_dst {
                Koma::Kara => {
                    // 駒が無いところに打つ

                    let mut da_kms_hashset = HashSet::new();
                    for kms_motigoma in MGS_ARRAY.iter() {

                        let sn1 = searcher.game_record.get_teban(&Jiai::Ji);
                        let km_motigoma = sn_kms_to_km( &sn1, kms_motigoma );

                        if 0<searcher.cur_position.get_mg( &km_motigoma ) {
                            // 駒を持っていれば
                            insert_da_kms_by_ms_km(&searcher, ms_dst, &km_motigoma, &mut da_kms_hashset);
                        }
                    }
                    for num_kms_da in da_kms_hashset {
                        let kms = num_to_kms( num_kms_da );
                        ss_hashset.insert( Movement{
                            source: SS_SRC_DA,    // 駒大
                            destination: ms_dst,       // どの升へ行きたいか
                            promotion: false,        // 打に成りは無し
                            drop: kms,         // 打った駒種類
                        }.to_hash() );
                    }
                },
                _ => {},
            }            
        }//suji
    }//dan   
}

/**
 * 打の駒種類生成
 *
 * 1. 移動先の升    ms_dst
 * 2. 移動先の駒    km_dst  ※先後が要るので、kmsではなくkm。
 *
 * そこに打てる駒種類を返す。
 */
pub fn insert_da_kms_by_ms_km(searcher: &Searcher, ms_dst:umasu, km_dst:&Koma, result_kms:&mut HashSet<usize>){
    // assert_banjo_ms(ms_dst,"Ｉnsert_da_kms_by_ms_km");

    let kms_dst = km_to_kms(&km_dst);
    if ! kms_can_da( &kms_dst ) {
        return; // 打って出てくることがない駒なら終了
    }

    // +------------------------+
    // | 打ちたいところは空升か |
    // +------------------------+
    let km_banjo = searcher.cur_position.get_km_by_ms( ms_dst );
    match km_banjo {
        Koma::Kara => {},
        _ => { return; },// 駒があるところに打つ手は終了
    }
    // 駒が無いところに打つ

    // +------------------+
    // | 持っている駒か？ |
    // +------------------+
    if searcher.cur_position.get_mg( &km_dst ) < 1 {
        return; // 持っていない駒は打てない
    }

    // 回転していない将棋盤から見た筋番号
    let (suji,dy) = ms_to_suji_dan( ms_dst );
    /*
     * umasu は 将棋盤座標
     *
     * 考えることを打に限れば、先手も、後手も、後手から見た座標を使えば十分だぜ☆（＾～＾）
     *
     * ...
     * 13 23 33
     * 12 22 32
     * 11 21 31 ...
     */
    let sn = km_to_sn( km_dst );
    // let ms = kaiten180_ms_by_ms_sn( ms_dst, &sn );

    // assert_banjo_ms(ms,"Ｉnsert_da_kms_by_ms_km＜その２＞");
    //let (_x,y) = ms_to_suji_dan(ms);

    // 行先の無いところに駒を進めることの禁止☆（＾～＾）
    use kifuwarabe_position::Koma::*;
    match *km_dst {
        U0 => {
            // ▼うさぎ　は１、２段目には進めない
            if dy < DAN_3 {return;}
        },
        // ▼しし、▼ひよこ　は１段目には進めない
        S0 => {
            if dy < DAN_2 {return;}
        },
        H0 => {
            // ▼ひよこ　は２歩できない
            if dy < DAN_2 || searcher.cur_position.exists_fu_by_sn_suji( &sn, suji ) {return;}
        },
        U1 => {
            // △うさぎ　は８、９段目には進めない
            if DAN_7 < dy {return;}
        },
        // △しし、△ひよこ　は９段目には進めない
        S1 => { if DAN_8 < dy {return;} },
        H1 => {
            // △ひよこ　は２歩できない
            if DAN_8 < dy || searcher.cur_position.exists_fu_by_sn_suji( &sn, suji ) {return;}
        },
        _ => {}
    }
    result_kms.insert( kms_to_num(&kms_dst) );
}
/**
 * 移動先升生成
 *
 * 1. 移動元升
 * 2. 移動したい駒
 * 
 * 駒の移動先を取得。合法手生成の動き☆（＾～＾）
 *
 * km_src   : 移動元の駒
 * ms_src   : 移動元の升
 * to_nari  : 成りの手を生成するなら真
 * ky       : 現局面
 */
pub fn insert_dst_by_ms_km(
    ms_src:umasu,
    km_src:&Koma,
    to_nari:bool,
    result:&mut HashSet<umasu>,
    position1: &Position
) {
    // assert_banjo_ms(ms_src,"Ｉnsert_dst_by_ms_km");

    // 移動先の筋、段、駒種類、駒種類インデックス
    let (dx,dy) = ms_to_suji_dan(ms_src);
    let sn = km_to_sn( &km_src );
    let kms_src = km_to_kms(&km_src);

    // +--------------+
    // | 成れる駒か？ |
    // +--------------+
    if to_nari && !kms_can_pro( &kms_src ) {
        return; // 成れる駒でないなら、成りの動きはしない
    }
    
    let kms_num = kms_to_num(&kms_src);

    for i_dir in 0..KM_UGOKI_LN{ // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir : &KmDir;
        if match_sn( &Sengo::Sen, &sn ) {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
            p_kmdir = &_kmdir;
        } else {
            p_kmdir = &KM_UGOKI.back[kms_num][i_dir]
        };

        // 駒の位置を開始地点に、離れていくように調べていく
        use movement_picker::KmDir::*;
        match *p_kmdir {
            // 東
            E  (b)=>if b {
                        // 長東
                        for i_east in 1..9{
                            if dx+i_east<SUJI_10 {
                                let ms_src = suji_dan_to_ms(dx+i_east, dy);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                                if !match_sn( &sn_ms, &Sengo::Owari )  { break; } 
                            }
                        }
                    }else{
                        // 西東
                        if dx+1<SUJI_10 {
                            let ms_src = suji_dan_to_ms(dx+1, dy);
                            let sn_ms = position1.get_sn_by_ms( ms_src );
                            if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                        }
                    },
            // 北東
            NE (b)=>if b {
                        // 長北東
                        for i_ne in 1..9{
                            if dx+i_ne<SUJI_10 && dy+i_ne<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx+i_ne, dy+i_ne);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                                if !match_sn( &sn_ms, &Sengo::Owari )  { break; } 
                            }
                        }
                    }else{
                        // 北東
                        if dx+1<SUJI_10 && dy+1<DAN_10 {
                            let ms_src = suji_dan_to_ms(dx+1, dy+1);
                            let sn_ms = position1.get_sn_by_ms( ms_src );
                            if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                        }
                    },
            // 北北東
            NNE   =>{
                        if dx+1<SUJI_10 && dy+2<DAN_10 {
                            let ms_src = suji_dan_to_ms(dx+1, dy+2);
                            let sn_ms = position1.get_sn_by_ms( ms_src );
                            if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                        }
                    },
            // 北
            N  (b)=>if b {
                        // 長北
                        for i_south in 1..9{
                            if dy+i_south<DAN_10{
                                let ms_src = suji_dan_to_ms(dx, dy+i_south);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                                if !match_sn( &sn_ms, &Sengo::Owari )  { break; } 
                            }
                        }
                    }else{
                        // 北
                        if dy+1<DAN_10 {
                            let ms_src = suji_dan_to_ms(dx, dy+1);
                            let sn_ms = position1.get_sn_by_ms( ms_src );
                            if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                        }
                    },
            // 北北西
            NNW   =>{
                        if SUJI_0<dx-1 && dy+2<DAN_10 {
                            let ms_src = suji_dan_to_ms(dx-1, dy+2);
                            let sn_ms = position1.get_sn_by_ms( ms_src );
                            if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                        }
                    },
            // 北西
            NW (b)=>if b {
                        // 長北西
                        for i_se in 1..9{
                            if SUJI_0<dx-i_se && dy+i_se<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx-i_se, dy+i_se);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                                if !match_sn( &sn_ms, &Sengo::Owari )  { break; } 
                            }
                        }
                    }else{
                        // 北西
                        if dx-1>SUJI_0 && DAN_10>dy+1 {
                            let ms_src = suji_dan_to_ms(dx-1, dy+1);
                            let sn_ms = position1.get_sn_by_ms( ms_src );
                            if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                        }
                    },
            // 西
            W  (b)=>if b {
                        // 長西
                        for i_east in 1..9{
                            if SUJI_0<dx-i_east{
                                let ms_src = suji_dan_to_ms(dx-i_east, dy);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                                if !match_sn( &sn_ms, &Sengo::Owari )  { break; } 
                            }
                        }
                    }else{
                        // 西
                        if SUJI_0<dx-1 {
                            let ms_src = suji_dan_to_ms(dx-1, dy);
                            let sn_ms = position1.get_sn_by_ms( ms_src );
                            if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                        }
                    },
            // 南西
            SW (b)=>if b {
                        // 長南西
                        for i_ne in 1..9{
                            if SUJI_0<dx-i_ne && DAN_0<dy-i_ne {
                                let ms_src = suji_dan_to_ms(dx-i_ne, dy-i_ne);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                                if !match_sn( &sn_ms, &Sengo::Owari )  { break; } 
                            }
                        }
                    }else{
                        // 南西
                        if SUJI_0<dx-1 && DAN_0<dy-1 {
                            let ms_src = suji_dan_to_ms(dx-1, dy-1);
                            let sn_ms = position1.get_sn_by_ms( ms_src );
                            if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                        }
                    },
            // 南南西
            SSW   =>{
                        if SUJI_0<dx-1 && DAN_0<dy-2 {
                            let ms_src = suji_dan_to_ms(dx-1, dy-2);
                            let sn_ms = position1.get_sn_by_ms( ms_src );
                            if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                        }
                    },
            // 南
            S  (b)=>if b {
                        // 長南
                        for i_north in 1..9{
                            if DAN_0<dy-i_north {
                                let ms_src = suji_dan_to_ms(dx, dy-i_north);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                                if !match_sn( &sn_ms, &Sengo::Owari )  { break; } 
                            }
                        }
                    }else{
                        // 南
                        if DAN_0<dy-1 {
                            let ms_src = suji_dan_to_ms(dx, dy-1);
                            let sn_ms = position1.get_sn_by_ms( ms_src );
                            if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                        }
                    },
            // 南南東
            SSE   =>{
                        if dx+1<SUJI_10 && DAN_0<dy-2 {
                            let ms_src = suji_dan_to_ms(dx+1, dy-2);
                            let sn_ms = position1.get_sn_by_ms( ms_src );
                            if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                        }
                    },
            // 南東
            SE (b)=>if b {
                        // 長南東
                        for i_nw in 1..9{
                            if dx+i_nw<SUJI_10 && DAN_0<dy-i_nw {
                                let ms_src = suji_dan_to_ms(dx+i_nw, dy-i_nw);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                                if !match_sn( &sn_ms, &Sengo::Owari )  { break; } 
                            }
                        }
                    }else{
                        // 南東
                        if dx+1<SUJI_10 && DAN_0<dy-1 {
                            let ms_src = suji_dan_to_ms(dx+1, dy-1);
                            let sn_ms = position1.get_sn_by_ms( ms_src );
                            if !match_sn( &sn_ms, &sn )  { result.insert( ms_src); }
                        }
                    },
            Owari =>{ break },
        }                
    }

    if to_nari {
        // +------------------------------+
        // | 成れる動き以外での成りの禁止 |
        // +------------------------------+
        use kifuwarabe_position::Koma::*;
        match *km_src {
            K0 | Z0 | N0 => {
                // ▼きりん、▼ぞう、▼ねこ　は
                // 移動元または移動先が　１～３段目なら成れる
                let mut result2 : HashSet<umasu> = HashSet::new();
                for ms_dst in result.iter() {
                    let (_sx2,sy2) = ms_to_suji_dan( ms_src );
                    let (_dx2,dy2) = ms_to_suji_dan( *ms_dst );
                    if sy2 < DAN_4 && dy2 < DAN_4 { result2.insert( *ms_dst ); }
                }
                // 入れ直し
                result.clear();
                for ms_dst in result2.iter() {
                    result.insert( *ms_dst );
                }
            },
            U0 | S0 | H0 => {
                // ▼うさぎ、▼しし、▼ひよこ　は
                // 移動先が　１～３段目なら成れる
                let mut result2 : HashSet<umasu> = HashSet::new();
                for ms_dst in result.iter() {
                    let (_dx2,dy2) = ms_to_suji_dan( *ms_dst );
                    if dy2 < DAN_4 { result2.insert( *ms_dst ); }
                }
                // 入れ直し
                result.clear();
                for ms_dst in result2.iter() {
                    result.insert( *ms_dst );
                }
            },
            K1 | Z1 | N1 => {
                // △きりん、△ぞう、△ねこ　は
                // 移動元または移動先が　７～９段目なら成れる
                let mut result2 : HashSet<umasu> = HashSet::new();
                for ms_dst in result.iter() {
                    let (_sx2,sy2) = ms_to_suji_dan( ms_src );
                    let (_dx2,dy2) = ms_to_suji_dan( *ms_dst );
                    if DAN_6 < sy2 && DAN_6 < dy2 { result2.insert( *ms_dst ); }
                }
                // 入れ直し
                result.clear();
                for ms_dst in result2.iter() {
                    result.insert( *ms_dst );
                }
            },
            U1 | S1 | H1 => {
                // △うさぎ、△しし、△ひよこ　は
                // 移動先が　７～９段目なら成れる
                let mut result2 : HashSet<umasu> = HashSet::new();
                for ms_dst in result.iter() {
                    let (_dx2,dy2) = ms_to_suji_dan( *ms_dst );
                    if DAN_6 < dy2 { result2.insert( *ms_dst ); }
                }
                // 入れ直し
                result.clear();
                for ms_dst in result2.iter() {
                    result.insert( *ms_dst );
                }
            },
            _ => {},
        }
    } else {
        // +----------------------------------------+
        // | 行先の無いところに駒を進めることの禁止 |
        // +----------------------------------------+
        use kifuwarabe_position::Koma::*;
        match *km_src {
            U0      => {
                // ▼うさぎ　は１、２段目には進めない
                let mut result2 : HashSet<umasu> = HashSet::new();
                for ms_dst in result.iter() {
                    let (_dx2,dy2) = ms_to_suji_dan( *ms_dst );
                    if dy2 < DAN_3 { } else { result2.insert( *ms_dst ); }
                }
                // 入れ直し
                result.clear();
                for ms_dst in result2.iter() {
                    result.insert( *ms_dst );
                }
            },
            S0 | H0 => {
                // ▼しし、▼ひよこ　は１段目には進めない
                let mut result2 : HashSet<umasu> = HashSet::new();
                for ms_dst in result.iter() {
                    let (_dx2,dy2) = ms_to_suji_dan( *ms_dst );
                    if dy2 < DAN_2 { } else { result2.insert( *ms_dst ); }
                }
                // 入れ直し
                result.clear();
                for ms_dst in result2.iter() {
                    result.insert( *ms_dst );
                }
            },
            U1      => {
                // △うさぎ　は８、９段目には進めない
                let mut result2 : HashSet<umasu> = HashSet::new();
                for ms_dst in result.iter() {
                    let (_dx2,dy2) = ms_to_suji_dan( *ms_dst );
                    if DAN_7 < dy2 { } else { result2.insert( *ms_dst ); }
                }
                // 入れ直し
                result.clear();
                for ms_dst in result2.iter() {
                    result.insert( *ms_dst );
                }
            },
            S1 | H1 => {
                // △しし、△ひよこ　は９段目には進めない
                let mut result2 : HashSet<umasu> = HashSet::new();
                for ms_dst in result.iter() {
                    let (_dx2,dy2) = ms_to_suji_dan( *ms_dst );
                    if DAN_8 < dy2 { } else { result2.insert( *ms_dst ); }                
                }
                // 入れ直し
                result.clear();
                for ms_dst in result2.iter() {
                    result.insert( *ms_dst );
                }
            },
            _ => {}
        }
    }
}
/**
 * 移動元升生成
 *
 * 1. 手番の先後    sn
 * 2. 移動先升      ms_dst
 *
 * その升に到達できる駒が居る升を取得☆（＾～＾）
 * TODO 成りの動きも考えたい。升だけではなく、成りの有無☆（＾～＾）
 */
pub fn insert_narazu_src_by_sn_ms(sn:&Sengo, ms_dst:umasu, result:&mut HashSet<umasu>, position1: &Position) {
    // assert_banjo_ms(ms_dst,"Ｉnsert_narazu_src_by_sn_ms");

    // 移動先の筋、段
    let (dx,dy) = ms_to_suji_dan( ms_dst );

    // 駒種類
    for kms in KMS_ARRAY.iter() {

        // 行先の無いところに駒を進めることの禁止☆（＾～＾）
        let km = sn_kms_to_km( &sn, &kms );
        use kifuwarabe_position::Koma::*;
        match km {
            U0      => {
                // ▼うさぎ　は１、２段目には進めない
                if dy < DAN_3 { continue; }
            },
            S0 | H0 => {
                // ▼しし、▼ひよこ　は１段目には進めない
                if dy < DAN_2 { continue; }
            },
            U1      => {
                // △うさぎ　は８、９段目には進めない
                if DAN_7 < dy { continue; }
            },
            S1 | H1 => {
                // △しし、△ひよこ　は９段目には進めない
                if DAN_8 < dy { continue; }
            },
            _ => {}
        }

        let kms_num = kms_to_num( &kms );
        for i_dir in 0..KM_UGOKI_LN{ // 指定の駒種類の、全ての逆向きに動ける方向
            let _kmdir;
            let p_kmdir : &KmDir;
            if match_sn( &Sengo::Sen, &sn ) {
                p_kmdir = &KM_UGOKI.back[kms_num][i_dir];
                // g_writeln(&format!("get_src_by_sn_ms 先手なら kms={} kms_num={} p_kmdir={}",
                //     kms, kms_num, p_kmdir
                // ));
            } else {
                _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
                p_kmdir = &_kmdir;
                // g_writeln(&format!("get_src_by_sn_ms 後手なら kms={} kms_num={} p_kmdir={}",
                //     kms, kms_num, p_kmdir
                // ));
            }

            // 指定升を開始地点に、離れていくように調べていく
            // 指定先後の駒があれば追加
            use movement_picker::KmDir::*;
            match *p_kmdir {
                // 東
                E  (b)=>if b {
                            // 長東
                            for i_east in 1..9{
                                if dx+i_east<SUJI_10 {
                                    let ms_src = suji_dan_to_ms(dx+i_east, dy);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 東
                            if dx+1<SUJI_10 {
                                let ms_src = suji_dan_to_ms(dx+1, dy);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 北東
                NE (b)=>if b {
                            // 長北東
                            for i_ne in 1..9{
                                if dx+i_ne<SUJI_10 && dy+i_ne<DAN_10 {
                                    let ms_src = suji_dan_to_ms(dx+i_ne, dy+i_ne);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 北東
                            if dx+1<SUJI_10 && dy+1<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx+1, dy+1);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 北北東
                NNE   =>{
                            if dx+1<SUJI_10 && dy+2<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx+1, dy+2);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 北
                N  (b)=>if b {
                            // 長北
                            for i_south in 1..9{
                                if dy+i_south<DAN_10{
                                    let ms_src = suji_dan_to_ms(dx, dy+i_south);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 北
                            if dy+1<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx, dy+1);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                // g_writeln(&format!("get_src_by_sn_ms 北 ms_src={} sn_ms=>{} kms_ms={} match_sn={} match_kms={}",
                                //     ms_src, sn_ms, kms_ms, match_sn( &sn_ms, &sn ), match_kms( &kms_ms, &kms )
                                // ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 北北西
                NNW   =>{
                            if SUJI_0<dx-1 && dy+2<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx-1, dy+2);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 北西
                NW (b)=>if b {
                            // 長北西
                            for i_se in 1..9{
                                if SUJI_0<dx-i_se && dy+i_se<DAN_10 {
                                    let ms_src = suji_dan_to_ms(dx-i_se, dy+i_se);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 北西
                            if dx-1>SUJI_0 && DAN_10>dy+1 {
                                let ms_src = suji_dan_to_ms(dx-1, dy+1);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 西
                W  (b)=>if b {
                            // 長西
                            for i_east in 1..9{
                                if SUJI_0<dx-i_east{
                                    let ms_src = suji_dan_to_ms(dx-i_east, dy);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 西
                            if SUJI_0<dx-1 {
                                let ms_src = suji_dan_to_ms(dx-1, dy);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 南西
                SW (b)=>if b {
                            // 長南西
                            for i_ne in 1..9{
                                if SUJI_0<dx-i_ne && DAN_0<dy-i_ne {
                                    let ms_src = suji_dan_to_ms(dx-i_ne, dy-i_ne);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 南西
                            if SUJI_0<dx-1 && DAN_0<dy-1 {
                                let ms_src = suji_dan_to_ms(dx-1, dy-1);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 南南西
                SSW   =>{
                            if SUJI_0<dx-1 && DAN_0<dy-2 {
                                let ms_src = suji_dan_to_ms(dx-1, dy-2);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 南
                S  (b)=>if b {
                            // 長南
                            for i_north in 1..9{
                                if DAN_0<dy-i_north {
                                    let ms_src = suji_dan_to_ms(dx, dy-i_north);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 南
                            if DAN_0<dy-1 {
                                let ms_src = suji_dan_to_ms(dx, dy-1);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                // g_writeln(&format!("get_src_by_sn_ms 南 kms={} kms_num={} ms_src={} sn_ms=>{} kms_ms={} match_sn={} match_kms={}",
                                //     kms, kms_num, ms_src, sn_ms, kms_ms, match_sn( &sn_ms, &sn ), match_kms( &kms_ms, &kms )
                                // ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 南南東
                SSE   =>{
                            if dx+1<SUJI_10 && DAN_0<dy-2 {
                                let ms_src = suji_dan_to_ms(dx+1, dy-2);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 南東
                SE (b)=>if b {
                            // 長南東
                            for i_nw in 1..9{
                                if dx+i_nw<SUJI_10 && DAN_0<dy-i_nw {
                                    let ms_src = suji_dan_to_ms(dx+i_nw, dy-i_nw);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 南東
                            if dx+1<SUJI_10 && DAN_0<dy-1 {
                                let ms_src = suji_dan_to_ms(dx+1, dy-1);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                Owari =>{ break },
            }
        }
    }
}
/**
 * 移動元升生成（成る前）
 */
pub fn insert_narumae_src_by_sn_ms(sn:&Sengo, ms_dst:umasu, result:&mut HashSet<umasu>, position1: &Position) {
    // assert_banjo_ms(ms_dst,"Ｉnsert_narumae_src_by_sn_ms");

    // 移動先の筋、段
    let (dx,dy) = ms_to_suji_dan( ms_dst );

    // 駒種類
    for kms in KMS_ARRAY.iter() {
        let km_src = sn_kms_to_km( &sn, &kms );

        // +--------------------+
        // | 移動前は非成駒か？ |
        // +--------------------+
        let kms_src = km_to_kms(&km_src);
        if kms_is_pro( &kms_src ) {
            continue; // 成る前に成駒なら、成りの動きをしていない
        }

        let prokm_src = km_to_prokm( &km_src );
        match prokm_src {
            Koma::Kara  => { continue; },// 成れない駒は、成る動きを考えなくていいぜ☆（＾～＾）
            _           => {},// 成れる駒は、成る前の駒の動きも調べる
        }

        // 成り駒に、行先の無いところは無いぜ☆

        let kms_num = kms_to_num( &kms );
        for i_dir in 0..KM_UGOKI_LN{ // 指定の駒種類の、全ての逆向きに動ける方向
            let _kmdir;
            let p_kmdir : &KmDir;
            if match_sn( &Sengo::Sen, &sn ) {
                p_kmdir = &KM_UGOKI.back[kms_num][i_dir];
                // g_writeln(&format!("get_src_by_sn_ms 先手なら kms={} kms_num={} p_kmdir={}",
                //     kms, kms_num, p_kmdir
                // ));
            } else {
                _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
                p_kmdir = &_kmdir;
                // g_writeln(&format!("get_src_by_sn_ms 後手なら kms={} kms_num={} p_kmdir={}",
                //     kms, kms_num, p_kmdir
                // ));
            }

            // 指定升を開始地点に、離れていくように調べていく
            // 指定先後の駒があれば追加
            use movement_picker::KmDir::*;
            match *p_kmdir {
                // 東
                E  (b)=>if b {
                            // 長東
                            for i_east in 1..9{
                                if dx+i_east<SUJI_10 {
                                    let ms_src = suji_dan_to_ms(dx+i_east, dy);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 東
                            if dx+1<SUJI_10 {
                                let ms_src = suji_dan_to_ms(dx+1, dy);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 北東
                NE (b)=>if b {
                            // 長北東
                            for i_ne in 1..9{
                                if dx+i_ne<SUJI_10 && dy+i_ne<DAN_10 {
                                    let ms_src = suji_dan_to_ms(dx+i_ne, dy+i_ne);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 北東
                            if dx+1<SUJI_10 && dy+1<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx+1, dy+1);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 北北東
                NNE   =>{
                            if dx+1<SUJI_10 && dy+2<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx+1, dy+2);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 北
                N  (b)=>if b {
                            // 長北
                            for i_south in 1..9{
                                if dy+i_south<DAN_10{
                                    let ms_src = suji_dan_to_ms(dx, dy+i_south);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 北
                            if dy+1<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx, dy+1);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                // g_writeln(&format!("get_src_by_sn_ms 北 ms_src={} sn_ms=>{} kms_ms={} match_sn={} match_kms={}",
                                //     ms_src, sn_ms, kms_ms, match_sn( &sn_ms, &sn ), match_kms( &kms_ms, &kms )
                                // ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 北北西
                NNW   =>{
                            if SUJI_0<dx-1 && dy+2<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx-1, dy+2);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 北西
                NW (b)=>if b {
                            // 長北西
                            for i_se in 1..9{
                                if SUJI_0<dx-i_se && dy+i_se<DAN_10 {
                                    let ms_src = suji_dan_to_ms(dx-i_se, dy+i_se);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 北西
                            if dx-1>SUJI_0 && DAN_10>dy+1 {
                                let ms_src = suji_dan_to_ms(dx-1, dy+1);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 西
                W  (b)=>if b {
                            // 長西
                            for i_east in 1..9{
                                if SUJI_0<dx-i_east{
                                    let ms_src = suji_dan_to_ms(dx-i_east, dy);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 西
                            if SUJI_0<dx-1 {
                                let ms_src = suji_dan_to_ms(dx-1, dy);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 南西
                SW (b)=>if b {
                            // 長南西
                            for i_ne in 1..9{
                                if SUJI_0<dx-i_ne && DAN_0<dy-i_ne {
                                    let ms_src = suji_dan_to_ms(dx-i_ne, dy-i_ne);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 南西
                            if SUJI_0<dx-1 && DAN_0<dy-1 {
                                let ms_src = suji_dan_to_ms(dx-1, dy-1);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 南南西
                SSW   =>{
                            if SUJI_0<dx-1 && DAN_0<dy-2 {
                                let ms_src = suji_dan_to_ms(dx-1, dy-2);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 南
                S  (b)=>if b {
                            // 長南
                            for i_north in 1..9{
                                if DAN_0<dy-i_north {
                                    let ms_src = suji_dan_to_ms(dx, dy-i_north);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 南
                            if DAN_0<dy-1 {
                                let ms_src = suji_dan_to_ms(dx, dy-1);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                // g_writeln(&format!("get_src_by_sn_ms 南 kms={} kms_num={} ms_src={} sn_ms=>{} kms_ms={} match_sn={} match_kms={}",
                                //     kms, kms_num, ms_src, sn_ms, kms_ms, match_sn( &sn_ms, &sn ), match_kms( &kms_ms, &kms )
                                // ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 南南東
                SSE   =>{
                            if dx+1<SUJI_10 && DAN_0<dy-2 {
                                let ms_src = suji_dan_to_ms(dx+1, dy-2);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                // 南東
                SE (b)=>if b {
                            // 長南東
                            for i_nw in 1..9{
                                if dx+i_nw<SUJI_10 && DAN_0<dy-i_nw {
                                    let ms_src = suji_dan_to_ms(dx+i_nw, dy-i_nw);
                                    let sn_ms = position1.get_sn_by_ms( ms_src );
                                    let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                    if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                                    if !match_sn( &sn_ms, &Sengo::Owari ) { break; } 
                                }
                            }
                        }else{
                            // 南東
                            if dx+1<SUJI_10 && DAN_0<dy-1 {
                                let ms_src = suji_dan_to_ms(dx+1, dy-1);
                                let sn_ms = position1.get_sn_by_ms( ms_src );
                                let kms_ms = km_to_kms( &position1.get_km_by_ms( ms_src ));
                                if match_sn( &sn_ms, &sn ) && match_kms( &kms_ms, &kms ) { result.insert( ms_src); }
                            }
                },
                Owari =>{ break },
            }
        }
    }
}

/*
 * 合い駒スペースを算出
 *
 * sn_atk  : 攻めている方の先後
 * ms_atk  : 攻め駒の居る升
 * ms_tgt  : 狙われている駒の居る升
 * kms_atk : 攻め駒の駒種類
 */
/*
#[allow(dead_code)]
pub fn get_ms_vec_as_aigoma(
    sn_atk:&Sengo,
    ms_atk:umasu,
    ms_tgt:umasu,
    kms_atk:&KmSyurui
    )->Vec<umasu> {
    let vec = Vec::new();

    use teigi::shogi_syugo::KmSyurui::*;
    match *kms_atk {
        K => {
            // 北方向
            // 西方向
            // 南方向
            // 東方向
        },
        Z => {
            // 北東方向
            // 北西方向
            // 南西方向
            // 南東方向
        },
        S => {
            if match_sn(&Sengo::Sen, &sn_atk) {
                // 北方向

            } else {
                // 南方向

            }
        },
        PK => {
            // 北方向
            // 西方向
            // 南方向
            // 東方向
        },
        PZ => {
            // 北東方向
            // 北西方向
            // 南西方向
            // 南東方向
        },
        _ => {}
    }
    vec
}
*/
