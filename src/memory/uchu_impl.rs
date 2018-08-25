use GAME_RECORD_WRAP;
use kifuwarabe_position::*;
use memory::uchu::*;

impl Uchu {

    /**
     * らいおんの位置
     */
    pub fn get_ms_r(&self, jiai:&Jiai, position1: &Position) -> umasu {
        let game_record = GAME_RECORD_WRAP.try_read().unwrap();
        position1.ms_r[ sn_to_num(&game_record.get_teban(jiai)) ]
    }
}