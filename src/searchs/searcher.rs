/// 探索部だぜ☆（＾～＾）
extern crate rand;
use rand::Rng;

use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use memory::uchu::*;
use misc::movement::*;
use std::collections::HashSet;
use syazo::sasite_seisei::*;
use syazo::sasite_sentaku::*;
use tusin::us_conv::*;


/// 探索オブジェクト。思考開始時に作成して使う。
pub struct Searcher{
}

impl Searcher{
    pub fn new()->Searcher{
        Searcher{
        
        }
    }

    /// 駒割り。
    pub fn get_koma_score(&self, km: &KmSyurui) -> i16 {
        use kifuwarabe_position::KmSyurui;
        match *km {
            KmSyurui::R => {15000},
            KmSyurui::Z => {  800},
            KmSyurui::K => { 1100},
            KmSyurui::I => {  600},
            KmSyurui::N => {  500},
            KmSyurui::U => {  300},
            KmSyurui::S => {  200},
            KmSyurui::H => {  100},
            KmSyurui::PZ => {  800},
            KmSyurui::PK => { 1100},
            KmSyurui::PN => {  500},
            KmSyurui::PU => {  300},
            KmSyurui::PS => {  200},
            KmSyurui::PH => {  100},
            _ => { 0},
        }
    }

    /// 探索。
    pub fn search(&mut self) -> Movement {
        // TODO 王手放置漏れ回避　を最優先させたいぜ☆（＾～＾）

        // +----------------------+
        // | 王手放置漏れ回避対策 |
        // +----------------------+

        // let を 先に記述した変数の方が、後に記述した変数より　寿命が長いので注意☆（＾～＾）

        // 現局面の合法手を取得する。
        let mut hashset_movement1 : HashSet<u64> = HashSet::new();

        // 駒の動き方
        insert_potential_move(&mut hashset_movement1 );
        // g_writeln("テスト ポテンシャルムーブ.");
        // use consoles::visuals::dumps::*;
        // hyoji_ss_hashset( &hashset_movement1 );


        // 指せる手から
        let mut komawari = 0;
        let mut max_komawari = -30000;
        let mut hashset_movement2 : HashSet<u64> = HashSet::new();
        'idea: for hash_mv in hashset_movement1.iter() {
            let movement = Movement::from_hash( *hash_mv );

            // 1手指す。
            make_movement2(&movement, |&cap: &KmSyurui|{
                // 駒割り
                komawari += self.get_koma_score(&cap);
            });

            if max_komawari < komawari {
                max_komawari = komawari;
                hashset_movement2.clear();
                hashset_movement2.insert(*hash_mv);
            } else if max_komawari == komawari {
                hashset_movement2.insert(*hash_mv);
            }

            // 1手戻す。
            unmake_movement2(|&cap|{
                // 駒割り
                komawari -= self.get_koma_score(&cap);
            });
        }




        filtering_ss_except_oute(&mut hashset_movement2);



        /*
        // 現局面を見て、ビジョンを作り直せだぜ☆（＾～＾）
        &UCHU_WRAP.try_write().unwrap().remake_visions();
        */

        /*
        // 楽観筋
        for sn in SN_ARRAY.iter() {
            let ai_sn = hanten_sn( sn );
            // 相手の　らいおん　の位置を覚える
            let ai_ms_r = CUR_POSITION_WRAP.try_read().unwrap().ms_r[sn_to_num(&ai_sn)];
            insert_rakkansuji(&sn, &mut UCHU_WRAP.try_write().unwrap().vision_tree_by_sn[sn_to_num(sn)], ai_ms_r);
        }
        // TODO 楽観筋はまだ使ってない☆（＾～＾）
        */

        // 楽観王手の一覧はできているはず。

        // FIXME 負けてても、千日手は除くぜ☆（＾～＾）ただし、千日手を取り除くと手がなくなる場合は取り除かないぜ☆（＾～＾）
        filtering_ss_except_sennitite(
            &mut hashset_movement2
        );

        // 自殺手は省くぜ☆（＾～＾）
        filtering_ss_except_jisatusyu( &mut hashset_movement2);





        if hashset_movement2.len()==0 {
            // 投了
            return Movement::new();
        } else {
            // 複数の手があれば、ランダムに1つに絞り込むぜ☆（＾～＾）
            let index = rand::thread_rng().gen_range(0, hashset_movement2.len());
            let mut i = 0;
            for ss_hash in hashset_movement2 {
                if i==index {
                    let ss = Movement::from_hash(ss_hash);
                    g_writeln(&format!("info solution:{}.", movement_to_usi(&ss) ));
                    return ss;
                }
                i+=1;
            }
            // 投了
            return Movement::new();
        }
    }
}
