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
    insert_rakkansuji();
    // TODO 楽観筋はまだ使ってない☆（＾～＾）

    // 楽観王手の一覧はできているはず。

    // FIXME 負けてても、千日手は除くぜ☆（＾～＾）ただし、千日手を取り除くと手がなくなる場合は取り除かないぜ☆（＾～＾）
    filtering_ss_except_sennitite(
        &mut ss_hashset
    );

    // 自殺手は省くぜ☆（＾～＾）
    filtering_ss_except_jisatusyu( &mut ss_hashset);



    if ss_hashset.len()==0 {
        // 投了
        return Sasite::new();
    } else {
        let index = rand::thread_rng().gen_range(0,ss_hashset.len());
        let mut i = 0;
        for ss_hash in ss_hashset {
            if i==index {
                //let result : Sasite = ss.clone();
                let ss = Sasite::from_hash(ss_hash);
                g_writeln(&format!("info solution:{}.", ss ));
                return ss;
            }
            i+=1;
        }

        // 投了
        return Sasite::new();
    }
}