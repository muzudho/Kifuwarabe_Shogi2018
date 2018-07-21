/**
 * 深い考えだぜ☆（＾～＾）
 */

extern crate rand;
use rand::Rng;
use std::collections::HashSet;

use memory::uchu::*;
use searchs::searcher::*;
use syazo::sasite_seisei::*;
use syazo::sasite_sentaku::*;
use teigi::conv::*;
use teigi::shogi_syugo::*;
use thinks::visions::vision_tree::*;
use tusin::usi::*;

use UCHU_WRAP;

/**
 * 現局面での最善手を返すぜ☆（*＾～＾*）
 */
pub fn think()->Sasite{

    // 探索を開始する。
    let mut searcher = Searcher::new();

    searcher.search();

    // let を 先に記述した変数の方が、後に記述した変数より　寿命が長いので注意☆（＾～＾）

    // 現局面の合法手を取得する。
    let mut ss_hashset : HashSet<u64> = HashSet::new();

    insert_potential_move(&mut ss_hashset );
    // g_writeln("テスト ポテンシャルムーブ.");
    // use consoles::visuals::dumps::*;
    // hyoji_ss_hashset( &ss_hashset );

    filtering_ss_except_oute(&mut ss_hashset);

    // 現局面を見て、ビジョンを作り直せだぜ☆（＾～＾）
    &UCHU_WRAP.write().unwrap().remake_visions();

    g_writeln( &format!("think(1)") );

    /*
    // 楽観筋
    for sn in SN_ARRAY.iter() {

        g_writeln( &format!("think(2)") );

        let ai_sn = hanten_sn( sn );

        g_writeln( &format!("think(3)") );

        // 相手の　らいおん　の位置を覚える
        let ai_ms_r = UCHU_WRAP.read().unwrap().ky.ms_r[sn_to_num(&ai_sn)];

        g_writeln( &format!("think(4)") );

        insert_rakkansuji(&sn, &mut UCHU_WRAP.write().unwrap().vision_tree_by_sn[sn_to_num(sn)], ai_ms_r);

        g_writeln( &format!("think(7)") );
    }
    // TODO 楽観筋はまだ使ってない☆（＾～＾）
    */

    g_writeln( &format!("think(8)") );

    // 楽観王手の一覧はできているはず。

    // FIXME 負けてても、千日手は除くぜ☆（＾～＾）ただし、千日手を取り除くと手がなくなる場合は取り除かないぜ☆（＾～＾）
    filtering_ss_except_sennitite(
        &mut ss_hashset
    );

    g_writeln( &format!("think(9)") );

    // 自殺手は省くぜ☆（＾～＾）
    filtering_ss_except_jisatusyu( &mut ss_hashset);

    g_writeln( &format!("think(10)") );


    if ss_hashset.len()==0 {
        // 投了

        g_writeln( &format!("think(11)") );

        return Sasite::new();
    } else {

        g_writeln( &format!("think(12)") );

        let index = rand::thread_rng().gen_range(0,ss_hashset.len());

        g_writeln( &format!("think(13)") );

        let mut i = 0;
        for ss_hash in ss_hashset {

            g_writeln( &format!("think(14)") );

            if i==index {

                g_writeln( &format!("think(15)") );

                //let result : Sasite = ss.clone();
                let ss = Sasite::from_hash(ss_hash);
                g_writeln(&format!("info solution:{}.", ss ));
                return ss;
            }

            g_writeln( &format!("think(16)") );

            i+=1;
        }

        g_writeln( &format!("think(17)") );

        // 投了
        return Sasite::new();
    }
}
