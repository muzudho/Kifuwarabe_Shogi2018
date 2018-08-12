/// 探索部だぜ☆（＾～＾）
use CUR_POSITION_EX_WRAP;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use misc::movement::*;
use std::collections::HashSet;
use syazo::sasite_seisei::*;
use syazo::sasite_sentaku::*;


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
    /// 
    /// Returns: ベストムーブ, 評価値。
    pub fn search(&mut self, max_depth: i16, cur_depth: i16) -> (Movement, i16) {

        if 0 == cur_depth {
            // 葉。
            // 現局面の駒割りを評価値とする。

            // 評価値は駒割り。
            let komawari;
            {
                let position_ex = CUR_POSITION_EX_WRAP.try_read().unwrap();
                komawari = position_ex.komawari;
            }

            // まだ次の手ではないので、点数は逆さにしておくと、さかさまにし直すように動く。
            return (Movement::new(), -komawari);
        }


        // 現局面の合法手を取得する。
        let mut hashset_movement : HashSet<u64> = HashSet::new();
        // 駒の動き方
        insert_potential_move(&mut hashset_movement);
        // g_writeln("テスト ポテンシャルムーブ.");
        // use consoles::visuals::dumps::*;
        // hyoji_ss_hashset( &hashset_movement );

        if max_depth == cur_depth {
            // 王手されている場合、王手回避の手に絞り込む。
            filtering_ss_except_oute(&mut hashset_movement);

            // FIXME 負けてても、千日手は除くぜ☆（＾～＾）ただし、千日手を取り除くと手がなくなる場合は取り除かないぜ☆（＾～＾）
            filtering_ss_except_sennitite(&mut hashset_movement);

            // 自殺手は省くぜ☆（＾～＾）
            filtering_ss_except_jisatusyu( &mut hashset_movement);
        }

        let mut best_movement = Movement::new();
        let mut best_evalutation = -30000;
        'idea: for hash_mv in hashset_movement.iter() {
            let movement = Movement::from_hash( *hash_mv );

            // 1手指す。
            make_movement2(&movement, |&cap: &KmSyurui|{
                // 駒割り
                let mut position_ex = CUR_POSITION_EX_WRAP.try_write().unwrap();
                position_ex.komawari += self.get_koma_score(&cap);
            });

            // 子を探索へ。
            let (_child_movement, mut child_evaluation) = self.search(max_depth, cur_depth-1);
            // 相手の評価値を逆さにする。
            child_evaluation = -child_evaluation;

            // 比較して、一番良い手を選ぶ。
            if best_evalutation < child_evaluation {
                best_evalutation = child_evaluation;
                best_movement = movement; // この手。
            }

            // 1手戻す。
            unmake_movement2(|&cap|{
                // 駒割り
                let mut position_ex = CUR_POSITION_EX_WRAP.try_write().unwrap();
                position_ex.komawari -= self.get_koma_score(&cap);
            });
        }

        // 返却。
        (best_movement, best_evalutation)
    }
}
