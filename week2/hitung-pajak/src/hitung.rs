use crate::data::{DataWajibPajak, HitunganTahunan};
use crate::konstanta::*;
use std::process;

pub fn hitung_pph(wp: &DataWajibPajak) -> HitunganTahunan {
    let bruto_tahunan = wp.gaji_bulanan * 12.0;
    if bruto_tahunan < PTKP_BASE {
        println!("Anda bukan wajib pajak");
        process::exit(1);
    }

    /* Hitung biaya jabatan */
    let biaya_jabatan_awal = BIAYA_JABATAN_RATE * bruto_tahunan;
    let mut biaya_jabatan_setahun: f64 = biaya_jabatan_awal;
    if biaya_jabatan_setahun > BIAYA_JABATAN_MAX_PER_YEAR {
        biaya_jabatan_setahun = BIAYA_JABATAN_MAX_PER_YEAR;
    }

    /* Hitung penghasilan netto tahunan */
    let netto_tahunan = bruto_tahunan - biaya_jabatan_setahun;

    let mut ptkp = PTKP_BASE;
    if wp.status_perkawinan == crate::data::StatusPerkawinan::Kawin {
        ptkp = ptkp + PTKP_KAWIN_ADDITION;
    }

    let jumlah_tanggungan = if wp.jumlah_tanggungan > PTKP_MAX_TANGGUNGAN {
        PTKP_MAX_TANGGUNGAN
    } else {
        wp.jumlah_tanggungan
    };
    let ptkp_total = ptkp + (jumlah_tanggungan * PTKP_TANGGUNGAN_ADDITION);
    /* Hitung PKP */
    let pkp_awal = netto_tahunan - ptkp_total;
    let mut pkp = 0.0;
    if pkp_awal > 0.0 {
        pkp = pkp_awal;
    }

    /* Hitung PPh 21 */
    let pph21 = if pkp <= BATAS_LAPISAN_1 {
        pkp * TARIF_LAPISAN_1
    } else if pkp <= BATAS_LAPISAN_2 {
        (BATAS_LAPISAN_1 * TARIF_LAPISAN_1) + ((pkp - BATAS_LAPISAN_1) * TARIF_LAPISAN_2)
    } else if pkp > BATAS_LAPISAN_2 && pkp <= BATAS_LAPISAN_3 {
        (BATAS_LAPISAN_1 * TARIF_LAPISAN_1)
            + ((BATAS_LAPISAN_2 - BATAS_LAPISAN_1) * TARIF_LAPISAN_2)
            + ((pkp - BATAS_LAPISAN_2) * TARIF_LAPISAN_3)
    } else if pkp <= BATAS_LAPISAN_4 {
        (BATAS_LAPISAN_1 * TARIF_LAPISAN_1)
            + ((BATAS_LAPISAN_2 - BATAS_LAPISAN_1) * TARIF_LAPISAN_2)
            + ((BATAS_LAPISAN_3 - BATAS_LAPISAN_2) * TARIF_LAPISAN_3)
            + ((pkp - BATAS_LAPISAN_3) * TARIF_LAPISAN_4)
    } else {
        (BATAS_LAPISAN_1 * TARIF_LAPISAN_1)
            + ((BATAS_LAPISAN_2 - BATAS_LAPISAN_1) * TARIF_LAPISAN_2)
            + ((BATAS_LAPISAN_3 - BATAS_LAPISAN_2) * TARIF_LAPISAN_3)
            + ((BATAS_LAPISAN_4 - BATAS_LAPISAN_3) * TARIF_LAPISAN_4)
            + ((pkp - BATAS_LAPISAN_4) * TARIF_LAPISAN_5)
    };

    HitunganTahunan {
        bruto_tahunan: bruto_tahunan as i32,
        biaya_jabatan: biaya_jabatan_setahun as i32,
        netto_tahunan: netto_tahunan as i32,
        ptkp_total: ptkp_total,
        pkp: pkp,
        pajak_tahunan: pph21 as i32,
    }
}
