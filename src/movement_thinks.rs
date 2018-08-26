use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use movement_picker::*;
use searcher_impl::*;
use std::collections::HashSet;
use teigi::shogi_syugo::*;
use teigi::conv::*;

/**
 * 1. 移動先升指定  ms_dst
 * 2. 移動先駒指定  km_dst
 *
 * 盤上の駒の移動の最初の１つ。打を除く
 */
pub fn insert_ss_by_ms_km_on_banjo(searcher: &Searcher, ms_dst:umasu, km_dst:&Koma, ss_hashset:&mut HashSet<u64>) {
    // assert_banjo_ms(ms_dst,"Ｉnsert_ss_by_ms_km_on_banjo");

    // 手番の先後、駒種類
    let (sn,_kms_dst) = km_to_sn_kms( &km_dst );

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if match_sn(&searcher.cur_position.get_sn_by_ms(ms_dst), &sn) {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = Movement::new();

    ss_hash_builder.destination = ms_dst;

    // 移動元の升
    let mut mv_src_hashset : HashSet<umasu> = HashSet::new();

    // +----------------+
    // | 盤上（成らず） |
    // +----------------+
    // 現局面を読取専用で取得し、ロック。
    insert_narazu_src_by_ms_km  (&searcher.cur_position, ms_dst, &km_dst, &mut mv_src_hashset);
    for ms_src in &mv_src_hashset{
        // assert_banjo_ms(*ms_src, "Ｉnsert_ss_by_ms_km_on_banjo ms_src(成らず)");

        ss_hash_builder.source = *ms_src;
        // 成らず
        ss_hash_builder.promotion = false;
        ss_hash_builder.drop = KmSyurui::Kara;
        ss_hashset.insert( ss_hash_builder.to_hash() );
    }

    // +--------------+
    // | 盤上（成り） |
    // +--------------+
    mv_src_hashset.clear();

    insert_narumae_src_by_ms_km (&searcher.cur_position, ms_dst, &km_dst, &mut mv_src_hashset );

    for ms_src in &mv_src_hashset{
        // assert_banjo_ms(*ms_src, "Ｉnsert_ss_by_ms_km_on_banjo ms_src(成り)");

        ss_hash_builder.source = *ms_src;
        // 成り
        ss_hash_builder.promotion = true;
        ss_hash_builder.drop = KmSyurui::Kara;
        ss_hashset.insert( ss_hash_builder.to_hash() );
    }
}

/**
 * 打
*
 * 1. 移動先升指定  ms_dst
 * 2. 移動先駒指定  km_dst
 */
pub fn insert_ss_by_ms_km_on_da(searcher: &Searcher, ms_dst:umasu, km_dst:&Koma, ss_hashset:&mut HashSet<u64>) {
    // assert_banjo_ms(ms_dst,"Ｉnsert_ss_by_ms_km_on_da");

    // 手番の先後、駒種類
    let (sn,_kms_dst) = km_to_sn_kms( &km_dst );

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if match_sn( &searcher.cur_position.get_sn_by_ms(ms_dst), &sn ) {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = Movement::new();

    ss_hash_builder.destination = ms_dst;

    // 移動元の升
    //let mut mv_src_hashset : HashSet<umasu> = HashSet::new();

    // +----+
    // | 打 |
    // +----+

    let mut da_kms_hashset : HashSet<usize> = HashSet::new();
    insert_da_kms_by_ms_km(&searcher, ms_dst, &km_dst, &mut da_kms_hashset);
    // 打
    for num_kms_da in da_kms_hashset.iter() {
        let kms_da = num_to_kms( *num_kms_da );
        
        let hash_ss = Movement{
            source: SS_SRC_DA,
            destination: ms_dst,
            promotion: false,
            drop: kms_da,
        }.to_hash();
        ss_hashset.insert( hash_ss );
    }
}

/**
 * 成る前を含めない、移動元升生成
 *
 * 1. 移動先を指定          ms_dst
 * 2. 移動先にある駒を指定  km_dst
 *
 * その願いが叶う移動元の一覧を返す。
 * 最大２０升。合法手生成の逆の動き☆（＾～＾）
 *
 * 「成る前」を調べるのは別関数でやるぜ☆（＾～＾）
 *
 * TODO 先手１段目の香車とか、必ず成らないといけないぜ☆（＾～＾）
 */
pub fn insert_narazu_src_by_ms_km(
    gen_ky: &Position,
    ms_dst: umasu,
    km_dst: &Koma,
    result: &mut HashSet<umasu>
) {
    // assert_banjo_ms(ms_dst,"ｉnsert_narazu_src_by_ms_km");

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
    let sn = km_to_sn( km_dst );
    let kms_dst = km_to_kms(&km_dst);
    let kms_num = kms_to_num(&kms_dst);

    // 行先の無いところに駒を進めることの禁止☆（＾～＾）
    use kifuwarabe_position::Koma::*;
    match *km_dst {
        U0      => {
            // ▼うさぎ　は１、２段目には進めない
            if dy < DAN_3 {return;}
        },
        S0 | H0 => {
            // ▼しし、▼ひよこ　は１段目には進めない
            if dy < DAN_2 {return;}
        },
        U1      => {
            // △うさぎ　は８、９段目には進めない
            if DAN_7 < dy {return;}
        },
        S1 | H1 => {
            // △しし、△ひよこ　は９段目には進めない
            if DAN_8 < dy {return;}
        },
        _ => {}
    }

    for i_dir in 0..KM_UGOKI_LN{ // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir : &KmDir;
        if match_sn( &Sengo::Sen, &sn ) {
            p_kmdir = &KM_UGOKI.back[kms_num][i_dir]
        } else {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
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
                                if gen_ky.has_ms_km( ms_src, km_dst ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    }else{
                        // 東
                        if dx+1<SUJI_10 {
                            let ms_src = suji_dan_to_ms(dx+1, dy);
                            if gen_ky.has_ms_km( ms_src, km_dst ) {
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
                                if gen_ky.has_ms_km( ms_src, km_dst ) {
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
                            if gen_ky.has_ms_km( ms_src, km_dst ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 北北東
            NNE   =>{
                        if dx+1<SUJI_10 && dy+2<DAN_10 {
                            let ms_src = suji_dan_to_ms(dx+1, dy+2);
                            if gen_ky.has_ms_km( ms_src, km_dst ) {
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
                                if gen_ky.has_ms_km( ms_src, km_dst ) {
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
                            if gen_ky.has_ms_km( ms_src, km_dst ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 北北西
            NNW   =>{
                        if SUJI_0<dx-1 && dy+2<DAN_10 {
                            let ms_src = suji_dan_to_ms(dx-1, dy+2);
                            if gen_ky.has_ms_km( ms_src, km_dst )
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
                                if gen_ky.has_ms_km( ms_src, km_dst ) {
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
                            if gen_ky.has_ms_km( ms_src, km_dst ) {
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
                                if gen_ky.has_ms_km( ms_src, km_dst ) { // 指定の駒があれば、その升は移動元。続行
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
                            if gen_ky.has_ms_km( ms_src, km_dst ) {
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
                                if gen_ky.has_ms_km( ms_src, km_dst ) {
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
                            if gen_ky.has_ms_km( ms_src, km_dst ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 南南西
            SSW   =>{
                        if SUJI_0<dx-1 && DAN_0<dy-2 {
                            let ms_src = suji_dan_to_ms(dx-1, dy-2);
                            if gen_ky.has_ms_km( ms_src, km_dst ) {
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
                                if gen_ky.has_ms_km( ms_src, km_dst ) {
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
                            if gen_ky.has_ms_km( ms_src, km_dst ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            // 南南東
            SSE   =>{
                        if dx+1<SUJI_10 && DAN_0<dy-2 {
                            let ms_src = suji_dan_to_ms(dx+1, dy-2);
                            if gen_ky.has_ms_km( ms_src, km_dst ) {
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
                                if gen_ky.has_ms_km( ms_src, km_dst ) {
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
                            if gen_ky.has_ms_km( ms_src, km_dst ) {
                                result.insert( ms_src);
                            }
                        }
                    },
            Owari =>{ break },
        }
    }
}
