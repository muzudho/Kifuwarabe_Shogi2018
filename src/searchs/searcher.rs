/**
 * 探索部だぜ☆（＾～＾）
 */

use mediators::med_kikisu::*;

/**
 * 探索オブジェクト。思考開始時に作成して使う。
 */
pub struct Searcher{
}

impl Searcher{
    pub fn new()->Searcher{
        Searcher{
        
        }
    }

    /**
     * 探索。
     */
    pub fn search(&mut self){

        // TODO 王手放置漏れ回避　を最優先させたいぜ☆（＾～＾）

        // +----------------------+
        // | 王手放置漏れ回避対策 |
        // +----------------------+

        // 相手の利き升調べ（自殺手防止のため）
        refresh_kikisu();
        // g_writeln( &format!("info test is_s={}", kasetu::atamakin::is_s() ) );

    }
}
