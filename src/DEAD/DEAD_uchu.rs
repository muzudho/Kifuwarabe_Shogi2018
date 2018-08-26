    /**
    * 自陣
    */
    #[allow(dead_code)]
    pub fn get_ji_jin(&self, game_record: &GameRecord)->Vec<umasu>{
        if let Sengo::Sen = game_record.get_teban(&Jiai::Ji) {
            SenteJin::to_elm()
        } else {
            GoteJin::to_elm()
        }
    }
    /**
    * 相手陣
    */
    #[allow(dead_code)]
    pub fn get_aite_jin(&self, game_record: &GameRecord)->Vec<umasu>{
        if let Sengo::Sen = game_record.get_teban(&Jiai::Ji) {
            GoteJin::to_elm()
        } else {
            SenteJin::to_elm()
        }
    }
