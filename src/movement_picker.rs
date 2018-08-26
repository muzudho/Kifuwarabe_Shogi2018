/// 指し手生成☆（＾～＾）

/*
 * 現局面を使った指し手生成
 */
extern crate rand;

// use consoles::asserts::*;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use rand::Rng;
use searcher_impl::*;
use std::collections::HashSet;
use teigi::shogi_syugo::*;
use teigi::conv::*;
use thinks::results::komatori_result::*;

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
 * 成る前の移動元升生成
 *
 * 1. 移動先の升        ms_dst
 * 2. 移動先にある駒    km_dst
 *
 * 成り　の動きでその結果になるような、元の升を返す☆（＾～＾）
 */
pub fn insert_narumae_src_by_ms_km(
    gen_ky: &Position,
    ms_dst: umasu,
    km_dst: &Koma,
    result: &mut HashSet<umasu>
){
    // assert_banjo_ms(ms_dst,"Ｉnsert_narumae_src_by_ms_km");

    // +--------------------+
    // | 移動後は成り駒か？ |
    // +--------------------+
    let kms_dst = km_to_kms(&km_dst);
    if !kms_is_pro( &kms_dst ) {
        return; // 成り駒でないなら、成りの動きをしていない
    }

    // +--------------------+
    // | 移動前は成る前の駒 |
    // +--------------------+
    let sn = km_to_sn( km_dst );
    let kms_src = prokms_to_kms( &kms_dst );
    let km_src = sn_kms_to_km( &sn, &kms_src );

    /*
     * umasu は 将棋盤座標
     *
     * ...
     * 13 23 33
     * 12 22 32
     * 11 21 31 ...
     *
     * x,y を使うと混乱するので、s,d を使う
     */
    // 移動先の筋、段、駒種類、駒種類インデックス
    let (dx,dy) = ms_to_suji_dan(ms_dst);

    // 例えば移動先の駒種類が「ぱひ」なら、「ぱひ」が動いた可能性の他に、
    // 「ひ」が動いたのかもしれない。
    // 「ぱひ」は、敵陣の１～３段目にいて、動きが北だった場合、元が「ひ」の可能性がある。
    let kms_src_narumae = prokms_to_kms( &kms_dst );

    use kifuwarabe_position::KmSyurui::*;
    match kms_src_narumae {
        Kara    => { return; },// 成れない駒は、成る動きを考えなくていいぜ☆（＾～＾）
        _       => {},// 成れる駒は、成る前の駒の動きも調べる
    }

    let kms_narumae_num = kms_to_num(&kms_src_narumae);

    for i_dir in 0..KM_UGOKI_LN{ // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir : &KmDir;
        if match_sn( &Sengo::Sen, &sn ) {
            p_kmdir = &KM_UGOKI.back[kms_narumae_num][i_dir]
        } else {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_narumae_num][i_dir]);
            p_kmdir = &_kmdir;
        };

        // 移動先を開始地点にして、駒の位置を終了地点にする
        use teigi::shogi_syugo::KmDir::*;
        match *p_kmdir {
            // 東
            E  (b)=>if b {
                        // 長東
                        for i_east in 1..9{
                            if dx+i_east<SUJI_10 {
                                let ms_src = suji_dan_to_ms(dx+i_east, dy);
                                if gen_ky.has_ms_km( ms_src, &km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    }else{
                        // 西東
                        if dx+1<SUJI_10 {
                            let ms_src = suji_dan_to_ms(dx+1, dy);
                            if gen_ky.has_ms_km( ms_src, &km_src ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 北東
            NE (b)=>if b {
                        // 長北東
                        for i_ne in 1..9{
                            if dx+i_ne<SUJI_10 && dy+i_ne<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx+i_ne, dy+i_ne);
                                if gen_ky.has_ms_km( ms_src, &km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    }else{
                        // 北東
                        if dx+1<SUJI_10 && dy+1<DAN_10 {
                            let ms_src = suji_dan_to_ms(dx+1, dy+1);
                            if gen_ky.has_ms_km( ms_src, &km_src ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 北北東
            NNE   =>{
                        if dx+1<SUJI_10 && dy+2<DAN_10 {
                            let ms_src = suji_dan_to_ms(dx+1, dy+2);
                            if gen_ky.has_ms_km( ms_src, &km_src ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 北
            N  (b)=>if b {
                        // 長北
                        for i_south in 1..9{
                            if dy+i_south<DAN_10{
                                let ms_src = suji_dan_to_ms(dx, dy+i_south);
                                if gen_ky.has_ms_km( ms_src, &km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    }else{
                        // 北
                        if dy+1<DAN_10 {
                            let ms_src = suji_dan_to_ms(dx, dy+1);
                            if gen_ky.has_ms_km( ms_src, &km_src ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 北北西
            NNW   =>{
                        if SUJI_0<dx-1 && dy+2<DAN_10 {
                            let ms_src = suji_dan_to_ms(dx-1, dy+2);
                            if gen_ky.has_ms_km( ms_src, &km_src )
                            {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 北西
            NW (b)=>if b {
                        // 長北西
                        for i_se in 1..9{
                            if SUJI_0<dx-i_se && dy+i_se<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx-i_se, dy+i_se);
                                if gen_ky.has_ms_km( ms_src, &km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    }else{
                        // 北西
                        if dx-1>SUJI_0 && DAN_10>dy+1 {
                            let ms_src = suji_dan_to_ms(dx-1, dy+1);
                            if gen_ky.has_ms_km( ms_src, &km_src ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 西
            W  (b)=>if b {
                        // 長西
                        for i_east in 1..9{
                            if SUJI_0<dx-i_east{
                                // 進みたいマスから戻ったマス
                                let ms_src = suji_dan_to_ms(dx-i_east, dy);
                                if gen_ky.has_ms_km( ms_src, &km_src ) { // 指定の駒があれば、その升は移動元。続行
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) { // なんか他の駒があれば終わり
                                    break;
                                }                                
                            }
                        }
                    }else{
                        // 西
                        if SUJI_0<dx-1 {
                            let ms_src = suji_dan_to_ms(dx-1, dy);
                            if gen_ky.has_ms_km( ms_src, &km_src ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 南西
            SW (b)=>if b {
                        // 長南西
                        for i_ne in 1..9{
                            if SUJI_0<dx-i_ne && DAN_0<dy-i_ne {
                                let ms_src = suji_dan_to_ms(dx-i_ne, dy-i_ne);
                                if gen_ky.has_ms_km( ms_src, &km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    }else{
                        // 南西
                        if SUJI_0<dx-1 && DAN_0<dy-1 {
                            let ms_src = suji_dan_to_ms(dx-1, dy-1);
                            if gen_ky.has_ms_km( ms_src, &km_src ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 南南西
            SSW   =>{
                        if SUJI_0<dx-1 && DAN_0<dy-2 {
                            let ms_src = suji_dan_to_ms(dx-1, dy-2);
                            if gen_ky.has_ms_km( ms_src, &km_src ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 南
            S  (b)=>if b {
                        // 長南
                        for i_north in 1..9{
                            if DAN_0<dy-i_north {
                                let ms_src = suji_dan_to_ms(dx, dy-i_north);
                                if gen_ky.has_ms_km( ms_src, &km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    }else{
                        // 南
                        if DAN_0<dy-1 {
                            let ms_src = suji_dan_to_ms(dx, dy-1);
                            if gen_ky.has_ms_km( ms_src, &km_src ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 南南東
            SSE   =>{
                        if dx+1<SUJI_10 && DAN_0<dy-2 {
                            let ms_src = suji_dan_to_ms(dx+1, dy-2);
                            if gen_ky.has_ms_km( ms_src, &km_src ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 南東
            SE (b)=>if b {
                        // 長南東
                        for i_nw in 1..9{
                            if dx+i_nw<SUJI_10 && DAN_0<dy-i_nw {
                                let ms_src = suji_dan_to_ms(dx+i_nw, dy-i_nw);
                                if gen_ky.has_ms_km( ms_src, &km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    }else{
                        // 南東
                        if dx+1<SUJI_10 && DAN_0<dy-1 {
                            let ms_src = suji_dan_to_ms(dx+1, dy-1);
                            if gen_ky.has_ms_km( ms_src, &km_src ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            Owari =>{ break },
        }
    }
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
        use teigi::shogi_syugo::KmDir::*;
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
            use teigi::shogi_syugo::KmDir::*;
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
            use teigi::shogi_syugo::KmDir::*;
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























pub fn choice_1ss_by_hashset( ss_hashset:&HashSet<u64> ) -> Movement {

    let index = if ss_hashset.len()==0 {
        0
    } else {
        rand::thread_rng().gen_range( 0, ss_hashset.len() )
    };
    let mut i = 0;
    let mut ss_choice_hash = 0;
    for ss_hash in ss_hashset.iter() {
        if i==index {
            ss_choice_hash = *ss_hash;
            break;
        }
        i += 1;
    }
    Movement::from_hash( ss_choice_hash )
}

/**
 * 王が取られる局面を除く手を選ぶぜ☆（＾～＾）
 */
pub fn filtering_ss_except_oute(
    searcher: &Searcher,
    ss_hashset_input:&mut HashSet<u64>
) {
    // 自玉の位置
    let ms_r = searcher.cur_position.ms_r[sn_to_num(&searcher.game_record.get_teban(&Jiai::Ji))];
    // g_writeln(&format!("info string My raion {}.", ms_r ));

    // 王手の一覧を取得
    let sn1;
    {
        sn1 = searcher.game_record.get_teban(&Jiai::Ai);
    }
    let komatori_result_hashset : HashSet<u64> = lookup_banjo_catch(&searcher, &sn1, ms_r);
    if 0<komatori_result_hashset.len() {
        // 王手されていれば

        // 表示
        /*
        // g_writeln(&format!("info string My raion is {} OUTED.", komatori_result_hashset.len() ));
        for komatori_result_hash0 in komatori_result_hashset.iter() {
            let komatori_result = KomatoriResult::from_hash( *komatori_result_hash0);
            // どんな王手か、出力
            // g_writeln(&format!("info string OUTE: {}.", komatori_result ));
        }
        */

        let mut ss_hashset_pickup : HashSet<u64> = HashSet::new();

        // 指せる手から、王手が消えている手だけ、選び抜くぜ☆（＾～＾）
        'idea: for hash_ss_potential in ss_hashset_input.iter() {
            let ss_potential = Movement::from_hash( *hash_ss_potential );
            for komatori_result_hash in komatori_result_hashset.iter() {
                let komatori_result = KomatoriResult::from_hash( *komatori_result_hash);

                // assert_banjo_ms( ss_potential.destination, "(206)Ｓearch_gohoshu_hash" );
                match komatori_result.get_result(&ss_potential) {
                        KomatoriResultResult::NoneAttacker
                    | KomatoriResultResult::NoneAigoma
                    | KomatoriResultResult::NoneMoved
                    => {
                        // 駒取りが起こらないものだけが解決
                    },
                    _ => {
                        // 解決しないのが１つでもあれば、次のアイデアへ☆（＾～＾）
                        continue 'idea;
                    },
                }
            }

            // 王手を回避している指し手
            ss_hashset_pickup.insert( *hash_ss_potential );
        }

        // 振り替え
        ss_hashset_input.clear();
        for hash_ss in ss_hashset_pickup.iter() {
            ss_hashset_input.insert( *hash_ss );
        }

    } else {
        // 王手されていなければ
        // g_writeln(&format!("info string My raion is not outed."));
    }
}

/**
 * 王手されていれば、王手を解除しろだぜ☆（＾～＾）
 * 千日手には喜んで飛び込めだぜ☆（＾▽＾）ｗｗｗ
 */
pub fn filtering_ss_except_jisatusyu(
    searcher: &mut Searcher,
    ss_hashset_input:&mut HashSet<u64>
){

    // 残すのはここに退避する☆（＾～＾）
    let mut ss_hashset_pickup : HashSet<u64> = HashSet::new();

    // 自玉の位置
    let sn1 = searcher.game_record.get_teban(&Jiai::Ji);
    let ms_r = searcher.cur_position.ms_r[ sn_to_num(&sn1) ];


    // 王手回避カードを発行する
    // TODO 王手が２か所から掛かっていたら、全部回避しないといけない☆

    // 指せる手から、王手が消えている手だけ、選び抜くぜ☆（＾～＾）
    'idea: for hash_ss_potential in ss_hashset_input.iter() {
        let ss_potential = Movement::from_hash( *hash_ss_potential );

        // その手を指してみる
        makemove(searcher, ss_potential.to_hash());
        // // 現局面表示
        // let s1 = kaku_ky(&KyNums::Current);
        // g_writeln( &s1 );            

        // 狙われている方の玉の位置
        let ms_r_new = if ss_potential.source == ms_r {
                ss_potential.destination // 狙われていた方の玉が動いた先
            } else {
                ms_r // 動いていない、狙われていた方の玉の居場所
            };

        // 利きの再計算
        // 有り得る移動元が入る☆（＾～＾）
        let mut attackers : HashSet<umasu> = HashSet::new();
        let sn1 = searcher.game_record.get_teban(&Jiai::Ji); // 指定の升に駒を動かそうとしている手番

        insert_narazu_src_by_sn_ms(
            &sn1,
            ms_r_new, // 指定の升
            &mut attackers,
            &searcher.cur_position);
        insert_narumae_src_by_sn_ms(
            &sn1,
            ms_r_new, // 指定の升
            &mut attackers,
            &searcher.cur_position);


        // 玉が利きに飛び込んでいるか？
        let jisatusyu = 0<attackers.len();
        /*
        // g_writeln(&format!("info string {} evaluated => {} attackers. offence={}->{}",
            movement_to_usi(&ss_potential),
            attackers.len(),
            sn1,
            ms_r_new
        ));
        for ms_atk in attackers.iter() {
            // g_writeln(&format!("info string ms_atk={}.",ms_atk ));
        }
        */

        // 手を戻す
        unmakemove(searcher);
        // // 現局面表示
        // let s2 = kaku_ky(&KyNums::Current);
        // g_writeln( &s2 );            

        if jisatusyu {
            continue 'idea;
        }

        //g_writeln(&format!("info string SOLUTED ss={}.", movement_to_usi(&ss_potential) ));
        // 問題を全て解決していれば、入れる
        ss_hashset_pickup.insert(ss_potential.to_hash());
    }
    //g_writeln(&format!("info string {} solutions.", ss_hashset_pickup.len() ));

    // 空っぽにする
    ss_hashset_input.clear();
    // 振り替える
    for hash_ss in ss_hashset_pickup.iter() {
        ss_hashset_input.insert( *hash_ss );
    }
}

/**
 * 千日手の指し手を取り除いた集合を作るぜ☆（＾～＾）
 *
 * ただし、千日手を取り除くと手がない場合は、千日手を選ぶぜ☆（＾～＾）
 */
pub fn filtering_ss_except_sennitite(
    searcher: &mut Searcher,
    ss_hashset_input:&mut HashSet<u64>
) {
    let mut ss_hashset_pickup = HashSet::new();

    // 指せる手から、千日手が消えている手だけ選んで、集合を作るぜ☆（＾～＾）
    'idea: for hash_ss_potential in ss_hashset_input.iter() {

        let ss = Movement::from_hash( *hash_ss_potential );
            //ss_hashset.insert( *hash_ss_potential );

        // その手を指してみる
        makemove(searcher, ss.to_hash());
        
        // 現局面表示
        // let s1 = kaku_ky(&KyNums::Current);
        // g_writeln( &s1 );            

        // 千日手かどうかを判定する☆（＾～＾）
        {
            if searcher.game_record.count_same_ky() < SENNTITE_NUM {
                ss_hashset_pickup.insert( *hash_ss_potential );
            } else {
                // 千日手
            }
        }

        // 手を戻す FIXME: 打った象が戻ってない？
        unmakemove(searcher);
        // 現局面表示
        // let s2 = kaku_ky(&KyNums::Current);
        // g_writeln( &s2 );
    }

    // ただし、千日手を取り除くと手がない場合は、千日手を選ぶぜ☆（＾～＾）
    if 0==ss_hashset_pickup.len() {
        return;
    }

    // 振り替え
    ss_hashset_input.clear();
    for hash_ss in ss_hashset_pickup.iter() {
        ss_hashset_input.insert( *hash_ss );
    }    
}