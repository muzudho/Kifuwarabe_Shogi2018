/**
 * コマンド一覧
 */

use kifuwarabe_position::*;
use LOGGER;

use UCHU_WRAP;

/**
 * 利き数表示
 */
pub fn cmd_kikisu(){
    for km in &KM_ARRAY {
        LOGGER.try_write().unwrap().writeln(&format!("利き数：{}", km));
        let s = UCHU_WRAP.try_read().unwrap().kaku_number_board(Sengo::Num, *km);
        LOGGER.try_write().unwrap().writeln( &s );
    }

    for sn in &SN_ARRAY {
        LOGGER.try_write().unwrap().writeln(&format!("利き数：{}", sn));
        let s = UCHU_WRAP.try_read().unwrap().kaku_number_board(*sn, Koma::Num);
        LOGGER.try_write().unwrap().writeln( &s );        
    }
}