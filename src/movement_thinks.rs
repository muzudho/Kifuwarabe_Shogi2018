extern crate rand;
use rand::Rng;

use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use kifuwarabe_movement_picker::*;
use searcher_impl::*;
use std::collections::HashSet;
use thinks::results::komatori_result::*;

/**
 * 1. 移動先升指定  ms_dst
 * 2. 移動先駒指定  km_dst
 *
 * 盤上の駒の移動の最初の１つ。打を除く
 */
pub fn insert_ss_by_ms_km_on_banjo(searcher: &Searcher, ms_dst:umasu, km_dst:Koma, ss_hashset:&mut HashSet<u64>) {
    // assert_banjo_ms(ms_dst,"Ｉnsert_ss_by_ms_km_on_banjo");

    // 手番の先後、駒種類
    let (sn,_kms_dst) = km_to_sn_kms( km_dst );

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if searcher.cur_position.get_sn_by_ms(ms_dst) == sn {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = Movement::default();

    ss_hash_builder.destination = ms_dst;

    // 移動元の升
    let mut mv_src_hashset : HashSet<umasu> = HashSet::new();

    // +----------------+
    // | 盤上（成らず） |
    // +----------------+
    // 現局面を読取専用で取得し、ロック。
    insert_narazu_src_by_ms_km  (&searcher.cur_position, ms_dst, km_dst, &mut mv_src_hashset);
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

    insert_narumae_src_by_ms_km (&searcher.cur_position, ms_dst, km_dst, &mut mv_src_hashset );

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
pub fn insert_ss_by_ms_km_on_da(searcher: &Searcher, ms_dst:umasu, km_dst:Koma, ss_hashset:&mut HashSet<u64>) {
    // assert_banjo_ms(ms_dst,"Ｉnsert_ss_by_ms_km_on_da");

    // 手番の先後、駒種類
    let (sn,_kms_dst) = km_to_sn_kms( km_dst );

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if searcher.cur_position.get_sn_by_ms(ms_dst) == sn {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = Movement::default();

    ss_hash_builder.destination = ms_dst;

    // 移動元の升
    //let mut mv_src_hashset : HashSet<umasu> = HashSet::new();

    // +----+
    // | 打 |
    // +----+

    let mut da_kms_hashset : HashSet<usize> = HashSet::new();
    insert_da_kms_by_ms_km(&searcher.cur_position, ms_dst, km_dst, &mut da_kms_hashset);
    // 打
    for num_kms_da in &da_kms_hashset {
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
    km_dst: Koma,
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
    let kms_dst = km_to_kms(km_dst);
    let kms_num = kms_dst as usize;

    // 行先の無いところに駒を進めることの禁止☆（＾～＾）
    use kifuwarabe_position::Koma::*;
    match km_dst {
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
        if Sengo::Sen == sn {
            p_kmdir = &KM_UGOKI.back[kms_num][i_dir]
        } else {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
            p_kmdir = &_kmdir;
        };
        // 移動先を開始地点にして、駒の位置を終了地点にする
        use kifuwarabe_movement_picker::KmDir::*;
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
                    // 東
                    }else if dx+1<SUJI_10 {
                        let ms_src = suji_dan_to_ms(dx+1, dy);
                        if gen_ky.has_ms_km( ms_src, km_dst ) {
                            result.insert( ms_src);
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
                    // 北東
                    }else if dx+1<SUJI_10 && dy+1<DAN_10 {
                        let ms_src = suji_dan_to_ms(dx+1, dy+1);
                        if gen_ky.has_ms_km( ms_src, km_dst ) {
                            result.insert( ms_src);
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
                    // 北
                    }else if dy+1<DAN_10 {
                        let ms_src = suji_dan_to_ms(dx, dy+1);
                        if gen_ky.has_ms_km( ms_src, km_dst ) {
                            result.insert( ms_src);
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
                    // 北西
                    }else if dx-1>SUJI_0 && DAN_10>dy+1 {
                        let ms_src = suji_dan_to_ms(dx-1, dy+1);
                        if gen_ky.has_ms_km( ms_src, km_dst ) {
                            result.insert( ms_src);
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
                    // 西
                    }else if SUJI_0<dx-1 {
                        let ms_src = suji_dan_to_ms(dx-1, dy);
                        if gen_ky.has_ms_km( ms_src, km_dst ) {
                            result.insert( ms_src);
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
                    // 南西
                    }else if SUJI_0<dx-1 && DAN_0<dy-1 {
                        let ms_src = suji_dan_to_ms(dx-1, dy-1);
                        if gen_ky.has_ms_km( ms_src, km_dst ) {
                            result.insert( ms_src);
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
                    // 南
                    }else if DAN_0<dy-1 {
                        let ms_src = suji_dan_to_ms(dx, dy-1);
                        if gen_ky.has_ms_km( ms_src, km_dst ) {
                            result.insert( ms_src);
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
                    // 南東
                    }else if dx+1<SUJI_10 && DAN_0<dy-1 {
                        let ms_src = suji_dan_to_ms(dx+1, dy-1);
                        if gen_ky.has_ms_km(ms_src, km_dst) {
                            result.insert( ms_src);
                        }
                    },
            Num =>{ break },
        }
    }
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
    km_dst: Koma,
    result: &mut HashSet<umasu>
){
    // assert_banjo_ms(ms_dst,"Ｉnsert_narumae_src_by_ms_km");

    // +--------------------+
    // | 移動後は成り駒か？ |
    // +--------------------+
    let kms_dst = km_to_kms(km_dst);
    if !kms_is_pro( kms_dst ) {
        return; // 成り駒でないなら、成りの動きをしていない
    }

    // +--------------------+
    // | 移動前は成る前の駒 |
    // +--------------------+
    let sn = km_to_sn( km_dst );
    let kms_src = prokms_to_kms( kms_dst );
    let km_src = sn_kms_to_km( sn, kms_src );

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
    let kms_src_narumae = prokms_to_kms( kms_dst );

    use kifuwarabe_position::KmSyurui::*;
    if let Kara = kms_src_narumae { return; } // 成れない駒は、成る動きを考えなくていいぜ☆（＾～＾）
    // 成れる駒は、成る前の駒の動きも調べる

    let kms_narumae_num = kms_src_narumae as usize;

    for i_dir in 0..KM_UGOKI_LN{ // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir : &KmDir;
        if Sengo::Sen == sn {
            p_kmdir = &KM_UGOKI.back[kms_narumae_num][i_dir]
        } else {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_narumae_num][i_dir]);
            p_kmdir = &_kmdir;
        };

        // 移動先を開始地点にして、駒の位置を終了地点にする
        use kifuwarabe_movement_picker::KmDir::*;
        match *p_kmdir {
            // 東
            E  (b)=>if b {
                        // 長東
                        for i_east in 1..9{
                            if dx+i_east<SUJI_10 {
                                let ms_src = suji_dan_to_ms(dx+i_east, dy);
                                if gen_ky.has_ms_km( ms_src, km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    // 西東
                    }else if dx+1<SUJI_10 {
                        let ms_src = suji_dan_to_ms(dx+1, dy);
                        if gen_ky.has_ms_km( ms_src, km_src ) {
                            result.insert( ms_src);
                        }
                    },
            // 北東
            NE (b)=>if b {
                        // 長北東
                        for i_ne in 1..9{
                            if dx+i_ne<SUJI_10 && dy+i_ne<DAN_10 {
                                let ms_src = suji_dan_to_ms(dx+i_ne, dy+i_ne);
                                if gen_ky.has_ms_km( ms_src, km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    // 北東
                    }else if dx+1<SUJI_10 && dy+1<DAN_10 {
                        let ms_src = suji_dan_to_ms(dx+1, dy+1);
                        if gen_ky.has_ms_km( ms_src, km_src ) {
                            result.insert( ms_src);
                        }
                    },
            // 北北東
            NNE   =>{
                        if dx+1<SUJI_10 && dy+2<DAN_10 {
                            let ms_src = suji_dan_to_ms(dx+1, dy+2);
                            if gen_ky.has_ms_km( ms_src, km_src ) {
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
                                if gen_ky.has_ms_km( ms_src, km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    // 北
                    }else if dy+1<DAN_10 {
                        let ms_src = suji_dan_to_ms(dx, dy+1);
                        if gen_ky.has_ms_km( ms_src, km_src ) {
                            result.insert( ms_src);
                        }
                    },
            // 北北西
            NNW   =>{
                        if SUJI_0<dx-1 && dy+2<DAN_10 {
                            let ms_src = suji_dan_to_ms(dx-1, dy+2);
                            if gen_ky.has_ms_km( ms_src, km_src )
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
                                if gen_ky.has_ms_km( ms_src, km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    // 北西
                    }else if dx-1>SUJI_0 && DAN_10>dy+1 {
                        let ms_src = suji_dan_to_ms(dx-1, dy+1);
                        if gen_ky.has_ms_km( ms_src, km_src ) {
                            result.insert( ms_src);
                        }
                    },
            // 西
            W  (b)=>if b {
                        // 長西
                        for i_east in 1..9{
                            if SUJI_0<dx-i_east{
                                // 進みたいマスから戻ったマス
                                let ms_src = suji_dan_to_ms(dx-i_east, dy);
                                if gen_ky.has_ms_km( ms_src, km_src ) { // 指定の駒があれば、その升は移動元。続行
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) { // なんか他の駒があれば終わり
                                    break;
                                }                                
                            }
                        }
                    // 西
                    }else if SUJI_0<dx-1 {
                        let ms_src = suji_dan_to_ms(dx-1, dy);
                        if gen_ky.has_ms_km( ms_src, km_src ) {
                            result.insert( ms_src);
                        }
                    },
            // 南西
            SW (b)=>if b {
                        // 長南西
                        for i_ne in 1..9{
                            if SUJI_0<dx-i_ne && DAN_0<dy-i_ne {
                                let ms_src = suji_dan_to_ms(dx-i_ne, dy-i_ne);
                                if gen_ky.has_ms_km( ms_src, km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    // 南西
                    }else if SUJI_0<dx-1 && DAN_0<dy-1 {
                        let ms_src = suji_dan_to_ms(dx-1, dy-1);
                        if gen_ky.has_ms_km( ms_src, km_src ) {
                            result.insert( ms_src);
                        }
                    },
            // 南南西
            SSW   =>{
                        if SUJI_0<dx-1 && DAN_0<dy-2 {
                            let ms_src = suji_dan_to_ms(dx-1, dy-2);
                            if gen_ky.has_ms_km( ms_src, km_src ) {
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
                                if gen_ky.has_ms_km( ms_src, km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    // 南
                    }else if DAN_0<dy-1 {
                        let ms_src = suji_dan_to_ms(dx, dy-1);
                        if gen_ky.has_ms_km( ms_src, km_src ) {
                            result.insert( ms_src);
                        }
                    },
            // 南南東
            SSE   =>{
                        if dx+1<SUJI_10 && DAN_0<dy-2 {
                            let ms_src = suji_dan_to_ms(dx+1, dy-2);
                            if gen_ky.has_ms_km( ms_src, km_src ) {
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
                                if gen_ky.has_ms_km( ms_src, km_src ) {
                                    result.insert( ms_src);
                                } else if gen_ky.exists_km(ms_src) {
                                    break;
                                }
                            }
                        }
                    // 南東
                    }else if dx+1<SUJI_10 && DAN_0<dy-1 {
                        let ms_src = suji_dan_to_ms(dx+1, dy-1);
                        if gen_ky.has_ms_km( ms_src, km_src ) {
                            result.insert( ms_src);
                        }
                    },
            Num =>{ break },
        }
    }
}

pub fn choice_1ss_by_hashset( ss_hashset:&HashSet<u64> ) -> Movement {

    let index = if ss_hashset.is_empty() {
        0
    } else {
        rand::thread_rng().gen_range( 0, ss_hashset.len() )
    };
    let mut ss_choice_hash = 0;
    for (i, ss_hash) in ss_hashset.iter().enumerate() {
        if i==index {
            ss_choice_hash = *ss_hash;
            break;
        }
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
    let ms_r = searcher.cur_position.ms_r[searcher.game_record.get_teban(Jiai::Ji) as usize];
    // g_writeln(&format!("info string My raion {}.", ms_r ));

    // 王手の一覧を取得
    let sn1;
    {
        sn1 = searcher.game_record.get_teban(Jiai::Ai);
    }
    let komatori_result_hashset : HashSet<u64> = lookup_banjo_catch(&searcher, sn1, ms_r);
    if komatori_result_hashset.is_empty() {
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
            for komatori_result_hash in &komatori_result_hashset {
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
        for hash_ss in &ss_hashset_pickup {
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
    let sn1 = searcher.game_record.get_teban(Jiai::Ji);
    let ms_r = searcher.cur_position.ms_r[sn1 as usize];


    // 王手回避カードを発行する
    // TODO 王手が２か所から掛かっていたら、全部回避しないといけない☆

    // 指せる手から、王手が消えている手だけ、選び抜くぜ☆（＾～＾）
    'idea: for hash_ss_potential in ss_hashset_input.iter() {
        let ss_potential = Movement::from_hash( *hash_ss_potential );

        // その手を指してみる
        let mut dummy_alpha = 0;
        userdefined_makemove(searcher, ss_potential.to_hash(), &mut dummy_alpha);
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
        let sn1 = searcher.game_record.get_teban(Jiai::Ji); // 指定の升に駒を動かそうとしている手番

        insert_narazu_src_by_sn_ms(
            sn1,
            ms_r_new, // 指定の升
            &mut attackers,
            &searcher.cur_position);
        insert_narumae_src_by_sn_ms(
            sn1,
            ms_r_new, // 指定の升
            &mut attackers,
            &searcher.cur_position);


        // 玉が利きに飛び込んでいるか？
        let jisatusyu = !attackers.is_empty();
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
    for hash_ss in &ss_hashset_pickup {
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
    for hash_ss_potential in ss_hashset_input.iter() { // 'idea: 

        let ss = Movement::from_hash( *hash_ss_potential );
            //ss_hashset.insert( *hash_ss_potential );

        // その手を指してみる
        let mut dummy_alpha = 0;
        userdefined_makemove(searcher, ss.to_hash(), &mut dummy_alpha);
        
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
    if ss_hashset_pickup.is_empty() {
        return;
    }

    // 振り替え
    ss_hashset_input.clear();
    for hash_ss in &ss_hashset_pickup {
        ss_hashset_input.insert( *hash_ss );
    }    
}