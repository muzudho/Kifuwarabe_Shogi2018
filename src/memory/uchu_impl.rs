use CUR_POSITION_WRAP;
use kifuwarabe_position::*;
use memory::uchu::*;

impl Uchu {

    /**
     * らいおんの位置
     */
    pub fn get_ms_r( &self, jiai:&Jiai ) -> umasu {
        CUR_POSITION_WRAP.try_read().unwrap().ms_r[ sn_to_num(&self.get_teban(jiai)) ]
    }
}