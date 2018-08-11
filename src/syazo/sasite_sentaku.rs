/**
 * 指し手選択
 */

extern crate rand;
use rand::Rng;

use CUR_POSITION_WRAP;
use consoles::asserts::*;
use GAME_RECORD_WRAP;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use memory::uchu::*;
use std::collections::HashSet;
use syazo::sasite_element::*;
use thinks::results::komatori_result::*;
use tusin::us_conv::*;
use UCHU_WRAP;

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
    ss_hashset_input:&mut HashSet<u64>
) {
    // 自玉の位置
    let ms_r = UCHU_WRAP.try_read().unwrap().get_ms_r(&Jiai::Ji);
    g_writeln(&format!("info string My raion {}.", ms_r ));

    // 王手の一覧を取得
    let sn1;
    {
        sn1 = GAME_RECORD_WRAP.try_read().unwrap().get_teban(&Jiai::Ai);
    }
    let komatori_result_hashset : HashSet<u64> = lookup_banjo_catch(&sn1, ms_r);
    if 0<komatori_result_hashset.len() {
        // 王手されていれば

        // 表示
        g_writeln(&format!("info string My raion is {} OUTED.", komatori_result_hashset.len() ));
        for komatori_result_hash0 in komatori_result_hashset.iter() {
            let komatori_result = KomatoriResult::from_hash( *komatori_result_hash0);
            // どんな王手か、出力
            g_writeln(&format!("info OUTE: {}.", komatori_result ));
        }

        let mut ss_hashset_pickup : HashSet<u64> = HashSet::new();

        // 指せる手から、王手が消えている手だけ、選び抜くぜ☆（＾～＾）
        'idea: for hash_ss_potential in ss_hashset_input.iter() {
            let ss_potential = Movement::from_hash( *hash_ss_potential );
            for komatori_result_hash in komatori_result_hashset.iter() {
                let komatori_result = KomatoriResult::from_hash( *komatori_result_hash);

                assert_banjo_ms( ss_potential.destination, "(206)Ｓearch_gohoshu_hash" );
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
        g_writeln(&format!("info string My raion is not outed."));
    }
}

/**
 * 王手されていれば、王手を解除しろだぜ☆（＾～＾）
 * 千日手には喜んで飛び込めだぜ☆（＾▽＾）ｗｗｗ
 */
pub fn filtering_ss_except_jisatusyu(
    ss_hashset_input:&mut HashSet<u64>
){

    // 残すのはここに退避する☆（＾～＾）
    let mut ss_hashset_pickup : HashSet<u64> = HashSet::new();

    // 自玉の位置
    let sn1;
    {
        sn1 = GAME_RECORD_WRAP.try_read().unwrap().get_teban(&Jiai::Ji);
    }
    let ms_r = CUR_POSITION_WRAP.try_read().unwrap().ms_r[ sn_to_num(&sn1) ];


    // 王手回避カードを発行する
    // TODO 王手が２か所から掛かっていたら、全部回避しないといけない☆

    // 指せる手から、王手が消えている手だけ、選び抜くぜ☆（＾～＾）
    'idea: for hash_ss_potential in ss_hashset_input.iter() {
        let ss_potential = Movement::from_hash( *hash_ss_potential );

        // その手を指してみる
        {
            let mut uchu_w = UCHU_WRAP.try_write().unwrap();
            uchu_w.make_movement2(&ss_potential);
        }
        // // 現局面表示
        // let s1 = &UCHU_WRAP.try_read().unwrap().kaku_ky( &KyNums::Current );
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
        let sn1;
        {
            sn1 = GAME_RECORD_WRAP.try_read().unwrap().get_teban(&Jiai::Ji); // 指定の升に駒を動かそうとしている手番
        }
        insert_narazu_src_by_sn_ms(
            &sn1,
            ms_r_new, // 指定の升
            &mut attackers );
        insert_narumae_src_by_sn_ms(
            &sn1,
            ms_r_new, // 指定の升
            &mut attackers );


        // 玉が利きに飛び込んでいるか？
        let jisatusyu = 0<attackers.len();
        g_writeln(&format!("info {} evaluated => {} attackers. offence={}->{}",
            movement_to_usi(&ss_potential),
            attackers.len(),
            sn1,
            ms_r_new
        ));
        for ms_atk in attackers.iter() {
            g_writeln(&format!("info ms_atk={}.",ms_atk ));
        }

        // 手を戻す
        UCHU_WRAP.try_write().unwrap().unmake_movement2();
        // // 現局面表示
        // let s2 = &UCHU_WRAP.try_read().unwrap().kaku_ky( &KyNums::Current );
        // g_writeln( &s2 );            

        if jisatusyu {
            continue 'idea;
        }

        g_writeln(&format!("info SOLUTED ss={}.", movement_to_usi(&ss_potential) ));
        // 問題を全て解決していれば、入れる
        ss_hashset_pickup.insert( ss_potential.to_hash() );
    }
    g_writeln(&format!("info {} solutions.", ss_hashset_pickup.len() ));

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
    ss_hashset_input:&mut HashSet<u64>
) {
    let mut ss_hashset_pickup = HashSet::new();

    // 指せる手から、千日手が消えている手だけ選んで、集合を作るぜ☆（＾～＾）
    'idea: for hash_ss_potential in ss_hashset_input.iter() {

        let ss = Movement::from_hash( *hash_ss_potential );
            //ss_hashset.insert( *hash_ss_potential );

        // その手を指してみる
        {
            let mut uchu_w = UCHU_WRAP.try_write().unwrap();
            uchu_w.make_movement2(&ss);
        }
        // 現局面表示
        // let s1 = &UCHU_WRAP.try_read().unwrap().kaku_ky( &KyNums::Current );
        // g_writeln( &s1 );            

        // 千日手かどうかを判定する☆（＾～＾）
        if UCHU_WRAP.try_read().unwrap().count_same_ky() < SENNTITE_NUM {
            ss_hashset_pickup.insert( *hash_ss_potential );
        } else {
            // 千日手
        }

        // 手を戻す FIXME: 打った象が戻ってない？
        UCHU_WRAP.try_write().unwrap().unmake_movement2();
        // 現局面表示
        // let s2 = &UCHU_WRAP.try_read().unwrap().kaku_ky( &KyNums::Current );
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