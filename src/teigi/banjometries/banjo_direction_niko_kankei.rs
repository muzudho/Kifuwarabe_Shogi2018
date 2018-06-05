#![allow(dead_code)]
/**
 * 盤上の二項関係☆（＾～＾）
 */
use consoles::asserts::*;
use teigi::conv::*;
use teigi::geometries::geo_teigi::*;
use teigi::geometries::geo_direction_niko_kankei::*;
use teigi::shogi_syugo::*;



/**
 * 狙われている駒から見た、長い利きの駒の居る方向（８方向）
 *
 * 盤の方向は、後手から見た視点
 * 引数には、同じ升を指定しないものとする
 */
pub fn get_dir8_to_slider_from_target(
    ms_slider:umasu,
    km_slider:&Koma,
    ms_target:umasu
) -> Dir8 {
    debug_assert!( ms_slider != ms_target, "dosn't ms{}!={}",ms_slider,ms_target);

    assert_banjo_ms(ms_slider,"(205a1)get_dir8_to_slider_from_target");
    assert_banjo_ms(ms_target,"(205a2)get_dir8_to_slider_from_target");
    let p_slider = ms_to_p( ms_slider );
    let p_target = ms_to_p( ms_target );

    let (sn_slider,kms) = km_to_sn_kms( &km_slider );
    use teigi::shogi_syugo::KmSyurui::*;
    use teigi::shogi_syugo::Sengo::*;
    match kms{
        K => {
            // 筋か、段かのどちらかが同じ
            if match_argangle0_p_p( &p_slider, &p_target ) {
                if match_a_south_of_b( &p_slider, &p_target ){
                    Dir8::S
                } else {
                    Dir8::N
                }
            } else if match_argangle90_p_p( &p_slider, &p_target ) {
                if match_a_west_of_b( &p_slider, &p_target ){
                    Dir8::W
                } else {
                    Dir8::E
                }
            } else {
                    Dir8::Owari
            }
        },
        Z => {
            // 左上がり筋か、左下がり筋かのどちらかが同じ
            if match_argangle45_p_p( &p_slider, &p_target ) {
                if match_a_west_of_b( &p_slider, &p_target ){
                    Dir8::SW
                } else {
                    Dir8::NE
                }
            } else if match_argangle135_p_p( &p_slider, &p_target ) {
                if match_a_west_of_b( &p_slider, &p_target ){
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else {
                    Dir8::Owari
            }
        },
        S => {
            // 先後
            match sn_slider {
                Sen => Dir8::N,
                Go  => Dir8::S,
                _   => Dir8::Owari,
            }
        },
        PK => {
            // 筋か、段か、
            // 左上がり筋か、左下がり筋かの　いずれかが同じ
            if match_argangle0_p_p( &p_slider, &p_target ) {
                if match_a_south_of_b( &p_slider, &p_target ){
                    Dir8::S
                } else {
                    Dir8::N
                }
            } else if match_argangle45_p_p( &p_slider, &p_target ) {
                if match_a_west_of_b( &p_slider, &p_target ){
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else if match_argangle90_p_p( &p_slider, &p_target ) {
                if match_a_west_of_b( &p_slider, &p_target ){
                    Dir8::W
                } else {
                    Dir8::E
                }
            } else if match_argangle135_p_p( &p_slider, &p_target ) {
                if match_a_west_of_b( &p_slider, &p_target ){
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else {
                    Dir8::Owari
            }
        },
        PZ => {
            // 筋か、段か、
            // 左上がり筋か、左下がり筋かの　いずれかが同じ
            if match_argangle0_p_p( &p_slider, &p_target ) {
                if match_a_south_of_b( &p_slider, &p_target ){
                    Dir8::S
                } else {
                    Dir8::N
                }
            } else if match_argangle45_p_p( &p_slider, &p_target ) {
                if match_a_west_of_b( &p_slider, &p_target ){
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else if match_argangle90_p_p( &p_slider, &p_target ) {
                if match_a_west_of_b( &p_slider, &p_target ){
                    Dir8::W
                } else {
                    Dir8::E
                }
            } else if match_argangle135_p_p( &p_slider, &p_target ) {
                if match_a_west_of_b( &p_slider, &p_target ){
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else {
                    Dir8::Owari
            }
        },
        _ => Dir8::Owari
    }
}

