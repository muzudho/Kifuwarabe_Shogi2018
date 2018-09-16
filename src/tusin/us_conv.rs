#![allow(dead_code)]
/**
 * USIプロトコル Rustフレームワーク
 */
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use kifuwarabe_position::Koma::*;
use kifuwarabe_usi::*;



pub fn kms_to_pt (kms: KmSyurui) -> PieceType { // kms: &KmSyurui
    use kifuwarabe_position::KmSyurui::*;
    use kifuwarabe_usi::PieceType;
    match kms{
        R => PieceType::K,
        K => PieceType::R,
        Z => PieceType::B,
        I => PieceType::G,
        N => PieceType::S,
        U => PieceType::N,
        S => PieceType::L,
        H => PieceType::P,
        PK => PieceType::PR,
        PZ => PieceType::PB,
        PN => PieceType::PS,
        PU => PieceType::PN,
        PS => PieceType::PL,
        PH => PieceType::PP,
        Kara => PieceType::Space,
        Num => PieceType::Num,
    }
}

pub fn pt_to_kms (pt: PieceType) -> KmSyurui { // pt: &PieceType
    use kifuwarabe_usi::PieceType;
    use kifuwarabe_position::KmSyurui;
    match pt{
        PieceType::K => KmSyurui::R,
        PieceType::R => KmSyurui::K,
        PieceType::B => KmSyurui::Z,
        PieceType::G => KmSyurui::I,
        PieceType::S => KmSyurui::N,
        PieceType::N => KmSyurui::U,
        PieceType::L => KmSyurui::S,
        PieceType::P => KmSyurui::H,
        PieceType::PR => KmSyurui::PK,
        PieceType::PB => KmSyurui::PZ,
        PieceType::PS => KmSyurui::PN,
        PieceType::PN => KmSyurui::PU,
        PieceType::PL => KmSyurui::PS,
        PieceType::PP => KmSyurui::PH,
        PieceType::Space => KmSyurui::Kara,
        PieceType::Num => KmSyurui::Num,
    }
}



pub fn km_to_pc (km: Koma) -> Piece { // km: &Koma
    use kifuwarabe_usi::Piece;
    match km{
        R0 => Piece::K0,
        K0 => Piece::R0,
        Z0 => Piece::B0,
        I0 => Piece::G0,
        N0 => Piece::S0,
        U0 => Piece::N0,
        S0 => Piece::L0,
        H0 => Piece::P0,
        PK0 => Piece::PR0,
        PZ0 => Piece::PB0,
        PN0 => Piece::PS0,
        PU0 => Piece::PN0,
        PS0 => Piece::PL0,
        PH0 => Piece::PP0,
        R1 => Piece::K1,
        K1 => Piece::R1,
        Z1 => Piece::B1,
        I1 => Piece::G1,
        N1 => Piece::S1,
        U1 => Piece::N1,
        S1 => Piece::L1,
        H1 => Piece::P1,
        PK1 => Piece::PR1,
        PZ1 => Piece::PB1,
        PN1 => Piece::PS1,
        PU1 => Piece::PN1,
        PS1 => Piece::PL1,
        PH1 => Piece::PP1,
        Kara => Piece::Space,
        Num => Piece::Num,
    }
}

pub fn pc_to_km (pc: Piece) -> Koma { // pc: &Piece
    use kifuwarabe_usi::Piece::*;
    use kifuwarabe_position::Koma;
    match pc{
        K0 => Koma::R0,
        R0 => Koma::K0,
        B0 => Koma::Z0,
        G0 => Koma::I0,
        S0 => Koma::N0,
        N0 => Koma::U0,
        L0 => Koma::S0,
        P0 => Koma::H0,
        PR0 => Koma::PK0,
        PB0 => Koma::PZ0,
        PS0 => Koma::PN0,
        PN0 => Koma::PU0,
        PL0 => Koma::PS0,
        PP0 => Koma::PH0,
        K1 => Koma::R1,
        R1 => Koma::K1,
        B1 => Koma::Z1,
        G1 => Koma::I1,
        S1 => Koma::N1,
        N1 => Koma::U1,
        L1 => Koma::S1,
        P1 => Koma::H1,
        PR1 => Koma::PK1,
        PB1 => Koma::PZ1,
        PS1 => Koma::PN1,
        PN1 => Koma::PU1,
        PL1 => Koma::PS1,
        PP1 => Koma::PH1,
        Space => Koma::Kara,
        Num => Koma::Num,
    }
}


/// USIの指し手表記を、きふわらべの指し手に変換する。
pub fn usi_to_movement(successful: bool, mv: UsiMovement) -> Movement { // mv: &UsiMovement
    if successful {
        let source2 : umasu = match mv.drop {
            PieceType::Space => suji_dan_to_ms(mv.source_file, mv.source_rank),
            _ => 0,
        };

        let drop2 : KmSyurui = match mv.drop {
            PieceType::Space => KmSyurui::Kara,
            _ => pt_to_kms(mv.drop),
        };

        Movement {
            source : source2,
            destination : suji_dan_to_ms(mv.destination_file, mv.destination_rank),
            promotion : mv.promotion,
            drop : drop2,
        }
    } else {
        // 投了。
        Movement::default()
    }
}

pub fn movement_to_usi(mv: &Movement) -> UsiMovement {
    // 先に投了判定を行う。
    if mv.destination==0 {
        return UsiMovement::default();
    }
    
    let (dst_file, dst_rank) = ms_to_suji_dan(mv.destination);

    let (src_file, src_rank, drop2) = match mv.drop {
        KmSyurui::Kara => {
            let (src_file, src_rank) = ms_to_suji_dan(mv.source);
            (src_file, src_rank, PieceType::Space)
        },
        // 打なら
        _ => (SUJI_0, DAN_0, kms_to_pt(mv.drop)),
    };

    UsiMovement{
        source_file : src_file,
        source_rank : src_rank,
        drop : drop2,
        destination_file : dst_file,
        destination_rank : dst_rank,
        promotion : mv.promotion,
    }
}
