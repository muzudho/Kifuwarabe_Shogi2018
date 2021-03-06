/**
 * ユニットテストだぜ☆（＾～＾）
 *
 * test コマンドで実行しろだぜ☆（＾～＾）
 */
use consoles::visuals::dumps::*;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use LOGGER;
use meidai::math_meidai::*;
use kifuwarabe_movement_picker::*;
use movement_thinks::*;
use searcher_impl::*;
use std::collections::HashSet;
use thinks::randommove;
use teigi::geometries::geo_teigi::*;
use tusin::us_conv::*;

/**
 * test 2
 * といったコマンドに対応☆（＾～＾）
 */
pub fn test(searcher: &Searcher, line:&str, starts:&mut usize, len:usize) {

    // いろいろな動作テスト
    LOGGER.try_write().unwrap().writeln( &format!("test starts={} len={}", *starts, len));

    // 現局面を読取専用で取得し、ロック。

    if 4<(len-*starts) && &line[*starts..*starts+5] == "mvsrc" {
        *starts += 5;
        LOGGER.try_write().unwrap().writeln("4<len mvsrc");
        // 駒の移動元升
        LOGGER.try_write().unwrap().writeln( "駒の移動元升");
        let kms = randommove::rnd_kms();
        let km;
        {
            km = sn_kms_to_km( searcher.game_record.get_teban(Jiai::Ji), *kms );
        }
        let ms_dst = randommove::rnd_ms();
        LOGGER.try_write().unwrap().writeln( &format!("kms={} km={} ms_dst={}",kms,km,ms_dst) );
        let mut mv_src_hashset : HashSet<umasu> = HashSet::new();
        let mut da_kms_hashset : HashSet<usize> = HashSet::new();
        insert_narazu_src_by_ms_km  (&searcher.cur_position, ms_dst, km, &mut mv_src_hashset);
        insert_narumae_src_by_ms_km (&searcher.cur_position, ms_dst, km, &mut mv_src_hashset);
        insert_da_kms_by_ms_km      (&searcher.cur_position, ms_dst, km, &mut da_kms_hashset);
        hyoji_ms_hashset    ( &mv_src_hashset);
        hyoji_kms_hashset   ( &da_kms_hashset);

    }else if 3<(len-*starts) && &line[*starts..*starts+4] == "mvkm" {
        *starts += 4;
        // 移動後の駒
        let kms = randommove::rnd_kms();
        let km;
        {
            km = sn_kms_to_km( searcher.game_record.get_teban(Jiai::Ji), *kms );
        }
        // 移動先の升、および　不成駒／成駒
        let ms_dst = randommove::rnd_ms();
        let pro_dst = randommove::rnd_bool();
        let mut ss = Movement::default();
        // 移動可能な元升
        let mut mv_src_hashset : HashSet<umasu> = HashSet::new();
        //let mut da_kms_hashset : HashSet<usize> = HashSet::new();
        insert_narazu_src_by_ms_km  (&searcher.cur_position, ms_dst, km, &mut mv_src_hashset);
        insert_narumae_src_by_ms_km (&searcher.cur_position, ms_dst, km, &mut mv_src_hashset);
        //insert_da_kms_by_ms_km      ( ms_dst, &km, &mut da_kms_hashset );
        #[allow(never_loop)]
        for ms_src in mv_src_hashset {
            ss.source = ms_src;
            LOGGER.try_write().unwrap().writeln( &format!( "移動可能な駒がある升={}", ms_src) );
            ss.destination = ms_dst;
            ss.promotion = pro_dst;
            ss.drop = KmSyurui::Kara;
            break; // Clippyが「this loop never actually loops」とエラーにする。
        }
        LOGGER.try_write().unwrap().writeln( &format!( "指し手にすると={}", movement_to_usi(&ss) ) );

    } else if 0<(len-*starts) && &line[*starts..*starts+1] == "1" {
        *starts += 1;
        // 駒の移動元升
        {
            LOGGER.try_write().unwrap().writeln( "利きテスト1");
            let kms = KmSyurui::PH; // ぱわーあっぷひよこ
            let km = sn_kms_to_km( Sengo::Go, kms );// △ph
            let ms_dst = 79;
            LOGGER.try_write().unwrap().writeln( &format!("kms={} km={} ms_dst={}",kms,km,ms_dst) );
            let mut mv_src_hashset : HashSet<umasu> = HashSet::new();
            let mut da_kms_hashset : HashSet<usize> = HashSet::new();
            insert_narazu_src_by_ms_km  (&searcher.cur_position, ms_dst, km, &mut mv_src_hashset);
            insert_narumae_src_by_ms_km (&searcher.cur_position, ms_dst, km, &mut mv_src_hashset);
            insert_da_kms_by_ms_km      (&searcher.cur_position, ms_dst, km, &mut da_kms_hashset);
            hyoji_ms_hashset    ( &mv_src_hashset);
            hyoji_kms_hashset   ( &da_kms_hashset);
        }
        {
            LOGGER.try_write().unwrap().writeln( "利きテスト2");
            let kms = KmSyurui::PH; // ぱわーあっぷひよこ
            let km = sn_kms_to_km( Sengo::Go, kms );// △ph
            let ms_dst = 68;
            LOGGER.try_write().unwrap().writeln( &format!("kms={} km={} ms_dst={}",kms,km,ms_dst) );
            let mut mv_src_hashset : HashSet<umasu> = HashSet::new();
            let mut da_kms_hashset : HashSet<usize> = HashSet::new();
            insert_narazu_src_by_ms_km  (&searcher.cur_position, ms_dst, km, &mut mv_src_hashset);
            insert_narumae_src_by_ms_km (&searcher.cur_position, ms_dst, km, &mut mv_src_hashset);
            insert_da_kms_by_ms_km      (&searcher.cur_position, ms_dst, km, &mut da_kms_hashset);
            hyoji_ms_hashset    ( &mv_src_hashset);
            hyoji_kms_hashset   ( &da_kms_hashset);
        }
        {
            LOGGER.try_write().unwrap().writeln( "利きテスト3");
            let kms = KmSyurui::PH; // ぱわーあっぷひよこ
            let km = sn_kms_to_km( Sengo::Go, kms );// △ph
            let ms_dst = 77;
            LOGGER.try_write().unwrap().writeln( &format!("kms={} km={} ms_dst={}",kms,km,ms_dst) );
            let mut mv_src_hashset : HashSet<umasu> = HashSet::new();
            let mut da_kms_hashset : HashSet<usize> = HashSet::new();
            insert_narazu_src_by_ms_km  (&searcher.cur_position, ms_dst, km, &mut mv_src_hashset);
            insert_narumae_src_by_ms_km (&searcher.cur_position, ms_dst, km, &mut mv_src_hashset);
            insert_da_kms_by_ms_km      (&searcher.cur_position, ms_dst, km, &mut da_kms_hashset);
            hyoji_ms_hashset    ( &mv_src_hashset);
            hyoji_kms_hashset   ( &da_kms_hashset);
        }
        {
            LOGGER.try_write().unwrap().writeln( "利きテスト2");
            let kms = KmSyurui::R; // らいおん
            let km = sn_kms_to_km( Sengo::Sen, kms );// ▼ら
            let ms_dst = 58;
            LOGGER.try_write().unwrap().writeln( &format!("kms={} km={} ms_dst={}",kms,km,ms_dst) );
            let mut mv_src_hashset : HashSet<umasu> = HashSet::new();
            let mut da_kms_hashset : HashSet<usize> = HashSet::new();
            insert_narazu_src_by_ms_km  (&searcher.cur_position, ms_dst, km, &mut mv_src_hashset);
            insert_narumae_src_by_ms_km (&searcher.cur_position, ms_dst, km, &mut mv_src_hashset);
            insert_da_kms_by_ms_km      (&searcher.cur_position, ms_dst, km, &mut da_kms_hashset);
            hyoji_ms_hashset    ( &mv_src_hashset);
            hyoji_kms_hashset   ( &da_kms_hashset);
        }
    } else if 0<(len-*starts) && &line[*starts..*starts+1] == "2" {
        *starts += 1;
        LOGGER.try_write().unwrap().writeln( "順番テスト");
        LOGGER.try_write().unwrap().writeln( &format!( "0・0・0 = {}", reflexive_ordered3_i8(0,0,0) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "0・0・1 = {}", reflexive_ordered3_i8(0,0,1) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "0・0・2 = {}", reflexive_ordered3_i8(0,0,2) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "0・1・0 = {}", reflexive_ordered3_i8(0,1,0) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "0・1・1 = {}", reflexive_ordered3_i8(0,1,1) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "0・1・2 = {}", reflexive_ordered3_i8(0,1,2) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "0・2・0 = {}", reflexive_ordered3_i8(0,2,0) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "0・2・1 = {}", reflexive_ordered3_i8(0,2,1) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "0・2・2 = {}", reflexive_ordered3_i8(0,2,2) ) );

        LOGGER.try_write().unwrap().writeln( &format!( "1・0・0 = {}", reflexive_ordered3_i8(1,0,0) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "1・0・1 = {}", reflexive_ordered3_i8(1,0,1) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "1・0・2 = {}", reflexive_ordered3_i8(1,0,2) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "1・1・0 = {}", reflexive_ordered3_i8(1,1,0) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "1・1・1 = {}", reflexive_ordered3_i8(1,1,1) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "1・1・2 = {}", reflexive_ordered3_i8(1,1,2) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "1・2・0 = {}", reflexive_ordered3_i8(1,2,0) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "1・2・1 = {}", reflexive_ordered3_i8(1,2,1) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "1・2・2 = {}", reflexive_ordered3_i8(1,2,2) ) );

        LOGGER.try_write().unwrap().writeln( &format!( "2・0・0 = {}", reflexive_ordered3_i8(2,0,0) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "2・0・1 = {}", reflexive_ordered3_i8(2,0,1) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "2・0・2 = {}", reflexive_ordered3_i8(2,0,2) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "2・1・0 = {}", reflexive_ordered3_i8(2,1,0) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "2・1・1 = {}", reflexive_ordered3_i8(2,1,1) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "2・1・2 = {}", reflexive_ordered3_i8(2,1,2) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "2・2・0 = {}", reflexive_ordered3_i8(2,2,0) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "2・2・1 = {}", reflexive_ordered3_i8(2,2,1) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "2・2・2 = {}", reflexive_ordered3_i8(2,2,2) ) );

    } else if 0<(len-*starts) && &line[*starts..*starts+1] == "3" {
        *starts += 1;
        LOGGER.try_write().unwrap().writeln( "升Pは、点ABで作る平面上にあるか？");
        LOGGER.try_write().unwrap().writeln( "P・A・B" );
        LOGGER.try_write().unwrap().writeln( "a{0,0} b{1,1} c{2,2}" );
        let a = Point{ x:0, y: 0 };
        let b = Point{ x:1, y: 1 };
        let c = Point{ x:2, y: 2 };

        LOGGER.try_write().unwrap().writeln( &format!( "a・a・a = {}", intersect_point_on_plane(&a,&a,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・a・b = {}", intersect_point_on_plane(&a,&a,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・a・c = {}", intersect_point_on_plane(&a,&a,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・b・a = {}", intersect_point_on_plane(&a,&b,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・b・b = {}", intersect_point_on_plane(&a,&b,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・b・c = {}", intersect_point_on_plane(&a,&b,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・c・a = {}", intersect_point_on_plane(&a,&c,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・c・b = {}", intersect_point_on_plane(&a,&c,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・c・c = {}", intersect_point_on_plane(&a,&c,&c) ) );

        LOGGER.try_write().unwrap().writeln( &format!( "b・a・a = {}", intersect_point_on_plane(&b,&a,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・a・b = {}", intersect_point_on_plane(&b,&a,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・a・c = {}", intersect_point_on_plane(&b,&a,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・b・a = {}", intersect_point_on_plane(&b,&b,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・b・b = {}", intersect_point_on_plane(&b,&b,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・b・c = {}", intersect_point_on_plane(&b,&b,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・c・a = {}", intersect_point_on_plane(&b,&c,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・c・b = {}", intersect_point_on_plane(&b,&c,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・c・c = {}", intersect_point_on_plane(&b,&c,&c) ) );

        LOGGER.try_write().unwrap().writeln( &format!( "c・a・a = {}", intersect_point_on_plane(&c,&a,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・a・b = {}", intersect_point_on_plane(&c,&a,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・a・c = {}", intersect_point_on_plane(&c,&a,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・b・a = {}", intersect_point_on_plane(&c,&b,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・b・b = {}", intersect_point_on_plane(&c,&b,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・b・c = {}", intersect_point_on_plane(&c,&b,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・c・a = {}", intersect_point_on_plane(&c,&c,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・c・b = {}", intersect_point_on_plane(&c,&c,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・c・c = {}", intersect_point_on_plane(&c,&c,&c) ) );
            
    } else if 0<(len-*starts) && &line[*starts..*starts+1] == "4" {
        *starts += 1;
        LOGGER.try_write().unwrap().writeln( "点ABは、同じ段にあるか？");
        LOGGER.try_write().unwrap().writeln( "A・B" );
        LOGGER.try_write().unwrap().writeln( "a{0,0} b{1,1} c{2,2} d{2,0}" );
        let a = Point{ x:0, y: 0 };
        let b = Point{ x:1, y: 1 };
        let c = Point{ x:2, y: 2 };
        let d = Point{ x:2, y: 0 };
        LOGGER.try_write().unwrap().writeln( &format!( "a・a = {}", match_argangle0_p_p(&a,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・b = {}", match_argangle0_p_p(&a,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・c = {}", match_argangle0_p_p(&a,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・d = {}", match_argangle0_p_p(&a,&d) ) );

        LOGGER.try_write().unwrap().writeln( &format!( "b・a = {}", match_argangle0_p_p(&b,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・b = {}", match_argangle0_p_p(&b,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・c = {}", match_argangle0_p_p(&b,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・d = {}", match_argangle0_p_p(&b,&d) ) );

        LOGGER.try_write().unwrap().writeln( &format!( "c・a = {}", match_argangle0_p_p(&c,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・b = {}", match_argangle0_p_p(&c,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・c = {}", match_argangle0_p_p(&c,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・d = {}", match_argangle0_p_p(&c,&d) ) );

        LOGGER.try_write().unwrap().writeln( &format!( "d・a = {}", match_argangle0_p_p(&d,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "d・b = {}", match_argangle0_p_p(&d,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "d・c = {}", match_argangle0_p_p(&d,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "d・d = {}", match_argangle0_p_p(&d,&d) ) );

    } else if 0<(len-*starts) && &line[*starts..*starts+1] == "5" {
        *starts += 1;
        LOGGER.try_write().unwrap().writeln( "点ABは、４つの角度の直線上にあるか？");
        LOGGER.try_write().unwrap().writeln( "A・B" );
        LOGGER.try_write().unwrap().writeln( "a{0,0} b{1,1} c{2,2} d{2,0}" );
        let a = Point{ x:0, y: 0 };
        let b = Point{ x:1, y: 1 };
        let c = Point{ x:2, y: 2 };
        let d = Point{ x:2, y: 0 };
        LOGGER.try_write().unwrap().writeln( &format!( "a・a = {}", get_argangle4_p_p(&a,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・b = {}", get_argangle4_p_p(&a,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・c = {}", get_argangle4_p_p(&a,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a・d = {}", get_argangle4_p_p(&a,&d) ) );

        LOGGER.try_write().unwrap().writeln( &format!( "b・a = {}", get_argangle4_p_p(&b,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・b = {}", get_argangle4_p_p(&b,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・c = {}", get_argangle4_p_p(&b,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b・d = {}", get_argangle4_p_p(&b,&d) ) );

        LOGGER.try_write().unwrap().writeln( &format!( "c・a = {}", get_argangle4_p_p(&c,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・b = {}", get_argangle4_p_p(&c,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・c = {}", get_argangle4_p_p(&c,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c・d = {}", get_argangle4_p_p(&c,&d) ) );

        LOGGER.try_write().unwrap().writeln( &format!( "d・a = {}", get_argangle4_p_p(&d,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "d・b = {}", get_argangle4_p_p(&d,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "d・c = {}", get_argangle4_p_p(&d,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "d・d = {}", get_argangle4_p_p(&d,&d) ) );

    } else if 0<(len-*starts) && &line[*starts..*starts+1] == "6" {
        *starts += 1;
        LOGGER.try_write().unwrap().writeln( "升Pは、線分AB上にあるか？");
        LOGGER.try_write().unwrap().writeln( "P・A・B" );
        LOGGER.try_write().unwrap().writeln( "a{0,0} b{1,1} c{2,2} d{2,0}" );
        let a = Point{ x:0, y: 0 };
        let b = Point{ x:1, y: 1 };
        let c = Point{ x:2, y: 2 };
        let d = Point{ x:2, y: 0 };

        LOGGER.try_write().unwrap().writeln( &format!( "a　　a・a = {}", intersect_point_on_line_segment(&a,&a,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　a・b = {}", intersect_point_on_line_segment(&a,&a,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　a・c = {}", intersect_point_on_line_segment(&a,&a,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　a・d = {}", intersect_point_on_line_segment(&a,&a,&d) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　b・a = {}", intersect_point_on_line_segment(&a,&b,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　b・b = {}", intersect_point_on_line_segment(&a,&b,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　b・c = {}", intersect_point_on_line_segment(&a,&b,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　b・d = {}", intersect_point_on_line_segment(&a,&b,&d) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　c・a = {}", intersect_point_on_line_segment(&a,&c,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　c・b = {}", intersect_point_on_line_segment(&a,&c,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　c・c = {}", intersect_point_on_line_segment(&a,&c,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　c・d = {}", intersect_point_on_line_segment(&a,&c,&d) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　d・a = {}", intersect_point_on_line_segment(&a,&d,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　d・b = {}", intersect_point_on_line_segment(&a,&d,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　d・c = {}", intersect_point_on_line_segment(&a,&d,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "a　　d・d = {}", intersect_point_on_line_segment(&a,&d,&d) ) );

        LOGGER.try_write().unwrap().writeln( &format!( "b　　a・a = {}", intersect_point_on_line_segment(&b,&a,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　a・b = {}", intersect_point_on_line_segment(&b,&a,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　a・c = {}", intersect_point_on_line_segment(&b,&a,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　a・d = {}", intersect_point_on_line_segment(&b,&a,&d) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　b・a = {}", intersect_point_on_line_segment(&b,&b,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　b・b = {}", intersect_point_on_line_segment(&b,&b,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　b・c = {}", intersect_point_on_line_segment(&b,&b,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　b・d = {}", intersect_point_on_line_segment(&b,&b,&d) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　c・a = {}", intersect_point_on_line_segment(&b,&c,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　c・b = {}", intersect_point_on_line_segment(&b,&c,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　c・c = {}", intersect_point_on_line_segment(&b,&c,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　c・d = {}", intersect_point_on_line_segment(&b,&c,&d) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　d・a = {}", intersect_point_on_line_segment(&b,&d,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　d・b = {}", intersect_point_on_line_segment(&b,&d,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　d・c = {}", intersect_point_on_line_segment(&b,&d,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "b　　d・d = {}", intersect_point_on_line_segment(&b,&d,&d) ) );

        LOGGER.try_write().unwrap().writeln( &format!( "c　　a・a = {}", intersect_point_on_line_segment(&c,&a,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　a・b = {}", intersect_point_on_line_segment(&c,&a,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　a・c = {}", intersect_point_on_line_segment(&c,&a,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　a・d = {}", intersect_point_on_line_segment(&c,&a,&d) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　b・a = {}", intersect_point_on_line_segment(&c,&b,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　b・b = {}", intersect_point_on_line_segment(&c,&b,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　b・c = {}", intersect_point_on_line_segment(&c,&b,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　b・d = {}", intersect_point_on_line_segment(&c,&b,&d) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　c・a = {}", intersect_point_on_line_segment(&c,&c,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　c・b = {}", intersect_point_on_line_segment(&c,&c,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　c・c = {}", intersect_point_on_line_segment(&c,&c,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　c・d = {}", intersect_point_on_line_segment(&c,&c,&d) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　d・a = {}", intersect_point_on_line_segment(&c,&d,&a) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　d・b = {}", intersect_point_on_line_segment(&c,&d,&b) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　d・c = {}", intersect_point_on_line_segment(&c,&d,&c) ) );
        LOGGER.try_write().unwrap().writeln( &format!( "c　　d・d = {}", intersect_point_on_line_segment(&c,&d,&d) ) );

    } else {
        //LOGGER.try_write().unwrap().writeln( &format!( "未定義のテスト「{}」", &line[*starts..len-1] ) );
        //UCHU_WRAP.try_write().unwrap().push_command( &"position startpos moves 6i5h 8c8d 9i9h 8d8e 3g3f 8e8f 5h4h 8f8g+ 1i1h 8g9h 2g2f 9h8h 9g9f 8h7i 2i3g 8b8i+ 2f2e 7i7h".to_string() );
        //UCHU_WRAP.try_write().unwrap().push_command( &"ky".to_string() );
        //LOGGER.try_write().unwrap().writeln( &UCHU_WRAP.try_write().unwrap().pop_command() );
    }
}