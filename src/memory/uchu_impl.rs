use CUR_POSITION_WRAP;
use memory::ky::*;
use memory::uchu::*;
use teigi::conv::*;
use teigi::shogi_syugo::*;

impl Uchu {

    /**
     * らいおんの位置
     */
    pub fn get_ms_r( &self, jiai:&Jiai ) -> umasu {
        CUR_POSITION_WRAP.try_read().unwrap().ms_r[ sn_to_num(&self.get_teban(jiai)) ]
    }
}