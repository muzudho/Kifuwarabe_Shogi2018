/**
 * 探索部だぜ☆（＾～＾）
 */

use CUR_POSITION_WRAP;
use mediators::med_kikisu::*;
use memory::ky::*;
use UCHU_WRAP;

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

        let mut uchu = UCHU_WRAP.try_write().unwrap();

        // ゼロ・リセット
        // 駒別に用意した盤を使った、利き数。
        for km in KM_ARRAY.iter() {
            uchu.kiki_su_by_km[km_to_num(km)].clear();
        }

        // 先後別に用意した盤を使った、利き数。
        for sn in SN_ARRAY.iter() {
            uchu.kiki_su_by_sn[sn_to_num(sn)].clear();
        }

        // 相手の利き升調べ（自殺手防止のため）
        let (local_kiki_su_by_sn, local_kiki_su_by_km) = refresh_kikisu(
            // 現局面を読取専用で取得し、ロック。
            &CUR_POSITION_WRAP.try_read().unwrap()
            );
        // g_writeln( &format!("info test is_s={}", kasetu::atamakin::is_s() ) );

        // 駒別
        uchu.set_kiki_su_by_km( local_kiki_su_by_km);

        // 先後別
        uchu.set_kiki_su_by_sn( local_kiki_su_by_sn);
    }
}
