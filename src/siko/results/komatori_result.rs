/**
 * 結果：駒を取られる手
 */
use std::fmt;
use std::collections::HashSet;

use consoles::asserts::*;
use jotai::uchu::*;
use meidai::math_meidai::*;
use syazo::sasite_seisei::*;
use syazo::sasite_sentaku::*;
use teigi::conv::*;
use teigi::geometries::geo_teigi::*;
use teigi::shogi_syugo::*;
use tusin::usi::*;

/********************
 * 駒取り結果の結果 *
 ********************/
pub enum KomatoriResultResult {
    // 駒は取られる
    Done,
    // アタッカーを除去したことにより、不発
    NoneAttacker,
    // 合い駒をしたことにより、不発
    NoneAigoma,
    // 移動したことにより、不発
    NoneMoved,
    // それ以外
    #[allow(dead_code)]
    Owari,
}

/**
 * 結果：駒取り
 */
pub struct KomatoriResult{
    // 要因：王手をしてきている駒（１つ）
    km_attacker     :Koma,
    // 要因：アタッカーが居る升
    ms_attacker     :umasu,
    // 要因：狙われている駒が居る升
    ms_target       :umasu,
}
impl fmt::Display for KomatoriResult{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f,"KmTori:{}{}{}{}"
            ,self.ms_attacker
            ,self.km_attacker
            ,if km_is_nagaikiki(&self.km_attacker){ "-->" } else { "->" }
            ,self.ms_target
        )
    }
}
impl KomatoriResult{
    #[allow(dead_code)]
    pub fn get_ms_attacker(&self)->umasu{
        self.ms_attacker
    }
    pub fn to_hash(&self)->u64{
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_ms_to_hash(hash, self.ms_target);
        hash = push_ms_to_hash(hash, self.ms_attacker);
        push_km_to_hash(hash, &self.km_attacker)
    }
    pub fn from_hash(hash:u64)->KomatoriResult{
        // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
        let (hash,km_atk) = pop_km_from_hash(hash);
        let (hash,ms_atk) = pop_ms_from_hash(hash);
        let (_hash,ms_tgt) = pop_ms_from_hash(hash);
        KomatoriResult{
            km_attacker : km_atk,
            ms_attacker : ms_atk,
            ms_target   : ms_tgt,
        }
    }
    ///
    /// この結果を起こさないのに十分かどうか判断
    ///
    /// 解消十分方法：
    ///     (1) アタッカー升に駒を動かす（取ってしまえば解決）
    ///     (2-1) アタッカーがスライダーの場合
    ///         (2-1-1) 狙われている駒以外の駒（合い駒）を、間の升に置く
    ///     (2-2) アタッカーがスライダーではない場合
    ///         (2-2-1) 狙われている駒を、動かせば解決
    ///
    /// ss : 現局面での、駒の動き手の１つ
    pub fn get_result( &self, ss:&Sasite ) -> KomatoriResultResult{
        // (1)
        if self.ms_attacker == ss.dst {
            return KomatoriResultResult::NoneAttacker;
        }

        // (2-1)
        if km_is_nagaikiki( &self.km_attacker ) {
            assert_banjo_ms(ss.dst,             "(205b2)Ｇet_result");
            assert_banjo_ms(self.ms_attacker,   "(205b3)Ｇet_result");
            assert_banjo_ms(self.ms_target,     "(205b4)Ｇet_result");

            let p_dst = ms_to_p( ss.dst );
            let p_atk = ms_to_p( self.ms_attacker );
            let p_tgt = ms_to_p( self.ms_target );                

            // 合い駒判定
            if
                // これから動かす駒は、狙われている駒ではないとする
                ss.src != self.ms_target
                // あるいは打か
                || ss.src == SS_SRC_DA
            { 
                // 利きの線分上に、駒を置いたか？
                if intersect_point_on_line_segment( &p_dst, &p_atk, &p_tgt ) {
                    // 合い駒を置いて、指定の駒取りを不成功にした
                    return KomatoriResultResult::NoneAigoma;
                }
            } else {
                // 狙われている駒を動かす場合

                assert_banjo_ms(ss.src,             "(205b1)Ｇet_result");
                let p_src = ms_to_p( ss.src );

                // スライダー駒との角度
                let argangle4a = get_argangle4_p_p( &p_atk, &p_tgt );
                // これから動かす駒の、逃げた先と、いた場所との角度
                let argangle4b = get_argangle4_p_p( &p_dst, &p_src );

                // スライダーのいる筋の上で動いても、逃げたことにはならないぜ☆（＾～＾）
                match match_argangle4( &argangle4a, &argangle4b ) {
                    MatchingResult::Unmatched => {
                        g_writeln(&format!("info ss={} evaluated in slider.", ss ));
                        // スライダーから逃げても、ひよこの利きに飛び込むことはあるが……☆
                        return KomatoriResultResult::NoneMoved
                    },
                    _ => {
                        g_writeln(&format!("info ss={} in slider attack.", ss ));                    
                    },
                }
            }

        } else {
            // (3-2) 狙われている駒を、とりあえず動かす
            if self.ms_target == ss.src { return KomatoriResultResult::NoneMoved; }
        }

        // TODO 逃げた先の自殺手判定

        // 駒が取られてしまう場合
        KomatoriResultResult::Done
    }
}

/**
 * 王手という原因を作っている関係を、（確率的洗いざらい）調べるぜ☆（＾～＾）
 *
 * sn        : 駒を「動かす」方はどっちだぜ☆（＾～＾）
 * ms_target : 取りたい駒がいる升
 *
 * return u64 : KomatoriResult のハッシュ
 */
pub fn lookup_banjo_catch(uchu:&Uchu, sn:&Sengo, ms_target:umasu)->HashSet<u64> {
    assert_banjo_ms(
        ms_target,
        &format!("(119)Ｌookup_banjo_catch sn={} ms_target={}"
            ,sn ,ms_target)
    );

    let mut hash = HashSet::new();

    if ms_target==MASU_0 {return hash;}

    let mut ss_hashset = HashSet::new();

    for kms_dst in KMS_ARRAY.iter(){
        // 移動した後の相手の駒
        let km_dst = sn_kms_to_km( &sn, kms_dst );
        //let km_dst = sn_kms_to_km( &sn, rnd_kms() );
        // 指定マスに移動できるか
        // 打は除く

        ss_hashset.clear();
        insert_ss_by_ms_km_on_banjo( &uchu, ms_target, &km_dst, &mut ss_hashset );

        // g_writeln( &format!("テスト lookup_banjo_catch insert_ss_by_ms_km_on_banjo kms_dst={}.",kms_dst) );
        // use consoles::visuals::dumps::*;
        // hyoji_ss_hashset( &ss_hashset );

        let ss = choice_1ss_by_hashset( &ss_hashset );
        if ss.exists() {
            assert_banjo_ms(
                ss.src,
                &format!("(123)Ｌookup_banjo_catch ss.src /  ms_target={} km_dst={} ss={}"
                    , ms_target, km_dst, ss)
            );
            
            let oute_result = KomatoriResult{
                km_attacker : km_dst,
                ms_attacker : ss.src, // FIXME 打だと 0 になるのでは
                ms_target   : ms_target,
            };

            // 重複がいっぱい
            hash.insert( oute_result.to_hash() );
        }        
    }
    hash
}
