use CUR_POSITION_WRAP;
use GAME_RECORD_WRAP;
use kifuwarabe_position::*;
use memory::uchu::*;

impl Uchu {

    /**
     * らいおんの位置
     */
    pub fn get_ms_r( &self, jiai:&Jiai ) -> umasu {
        let game_record = GAME_RECORD_WRAP.try_read().unwrap();
        CUR_POSITION_WRAP.try_read().unwrap().ms_r[ sn_to_num(&game_record.get_teban(jiai)) ]
    }
}