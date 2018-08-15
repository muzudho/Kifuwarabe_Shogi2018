/// 探索部だぜ☆（＾～＾）
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use misc::movement::*;
use std::collections::HashSet;

fn empty_leaf_callback() -> (Movement, i16) {
    (Movement::new(), 0)
}

fn empty_makemove_callback(_cap: &KmSyurui) {
}

fn empty_unmakemove_callback(_cap: &KmSyurui) {
}

fn empty_pick_movements_callback(_max_depth: i16, _cur_depth: i16) -> HashSet<u64> {
    HashSet::new()
}

fn empty_compare_best_callback(_best_movement: &mut Movement, _best_evaluation: &mut i16, _movement: Movement, _child_evaluation: i16) {
}

/// 探索オブジェクト。思考開始時に作成して使う。
pub struct Searcher{
    pub leaf_callback: fn() -> (Movement, i16),
    pub makemove_callback: fn(&KmSyurui),
    pub unmakemove_callback: fn(&KmSyurui),
    pub pick_movements_callback: fn(max_depth: i16, cur_depth: i16) -> HashSet<u64>,
    pub compare_best_callback: fn(&mut Movement, &mut i16, Movement, i16),
}

impl Searcher{
    pub fn new()->Searcher{
        Searcher{
            leaf_callback: empty_leaf_callback,
            makemove_callback: empty_makemove_callback,
            unmakemove_callback: empty_unmakemove_callback,
            pick_movements_callback: empty_pick_movements_callback,
            compare_best_callback: empty_compare_best_callback,
        }
    }


    /// 探索。
    /// 
    /// Returns: ベストムーブ, 評価値。
    pub fn search(&mut self, max_depth: i16, cur_depth: i16) -> (Movement, i16) {

        if 0 == cur_depth {
            // 葉。
            return (self.leaf_callback)();
        }


        // 現局面の合法手を取得する。
        let hashset_movement = (self.pick_movements_callback)(max_depth, cur_depth);


        let mut best_movement = Movement::new();
        let mut best_evaluation = -30000;
        'idea: for hash_mv in hashset_movement.iter() {
            let movement = Movement::from_hash( *hash_mv );

            // 1手指す。
            make_movement2(&movement, self.makemove_callback);

            // 子を探索へ。
            let (_child_movement, mut child_evaluation) = self.search(max_depth, cur_depth-1);
            // 相手の評価値を逆さにする。
            child_evaluation = -child_evaluation;

            // 比較して、一番良い手を選ぶ。
            (self.compare_best_callback)(&mut best_movement, &mut best_evaluation, movement, child_evaluation);

            // 1手戻す。
            unmake_movement2(self.unmakemove_callback);
        }

        // 返却。
        (best_movement, best_evaluation)
    }
}
