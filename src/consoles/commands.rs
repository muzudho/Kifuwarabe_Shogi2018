/**
 * コマンド一覧
 */

use memory::ky::*;
use memory::uchu::*;

use UCHU_WRAP;

/**
 * 利き数表示
 */
pub fn cmd_kikisu(){
    for km in KM_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", km));
        let s = UCHU_WRAP.try_read().unwrap().kaku_number_board( &Sengo::Owari, &km );
        g_writeln( &s );
    }

    for sn in SN_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", sn));
        let s = UCHU_WRAP.try_read().unwrap().kaku_number_board( &sn, &Koma::Owari );
        g_writeln( &s );        
    }
}