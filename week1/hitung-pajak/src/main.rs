use std::io;
use std::process;

const PTKP_BASE: f64 = 54000000.0;
const PTKP_KAWIN_ADDITION: f64 = 4500000.0;
const PTKP_TANGGUNGAN_ADDITION: f64 = 4500000.0;
const PTKP_MAX_TANGGUNGAN: f64 = 3.0;
const BIAYA_JABATAN_RATE: f64 = 0.05;
const BIAYA_JABATAN_MAX_PER_YEAR: f64 = 6000000.0;
const BATAS_LAPISAN_1: f64 = 60000000.0;
const BATAS_LAPISAN_2: f64 = 250000000.0;
const BATAS_LAPISAN_3: f64 = 500000000.0;
const BATAS_LAPISAN_4: f64 = 5000000000.0;
const TARIF_LAPISAN_1: f64 = 0.05;
const TARIF_LAPISAN_2: f64 = 0.15;
const TARIF_LAPISAN_3: f64 = 0.25;
const TARIF_LAPISAN_4: f64 = 0.30;
const TARIF_LAPISAN_5: f64 = 0.35;

#[derive(Debug, PartialEq)]
enum StatusPerkawinan {
    Lajang,
    Kawin,
}

#[derive(Debug)]
struct DataWajibPajak {
    gaji_bulanan: f64,
    status_perkawinan: StatusPerkawinan,
    jumlah_tanggungan: f64,
}

struct HitunganTahunan {
    bruto_tahunan: i32,
    biaya_jabatan: i32,
    netto_tahunan: i32,
    ptkp_total: i32,
    pkp: i32,
    pajak_tahunan: i32,
}

fn main() {
    println!("Aplikasi Penghitung Pajak Penghasilan");

    let mut gaji_input = String::new();
    println!("Berapa gaji bulanan Anda? (Dalam rupiah)");
    io::stdin().read_line(&mut gaji_input).expect("Failed");
    let gaji_bulanan: f64 = gaji_input.trim().parse().expect("Please enter a number");

    let bruto_tahunan = gaji_bulanan * 12.0;
    if bruto_tahunan < PTKP_BASE {
        println!("Anda bukan wajib pajak");
        process::exit(1);
    }

    /* Hitung biaya jabatan */
    let biaya_jabatan_awal = BIAYA_JABATAN_RATE * bruto_tahunan;
    let mut biaya_jabatan_setahun: f64 = biaya_jabatan_awal * 12.0;
    if biaya_jabatan_setahun > BIAYA_JABATAN_MAX_PER_YEAR {
        biaya_jabatan_setahun = BIAYA_JABATAN_MAX_PER_YEAR;
    }

    /* Hitung penghasilan netto tahunan */
    let netto_tahunan = bruto_tahunan - biaya_jabatan_setahun;

    /* Hitung PTKP */

    let mut status_perkawinan_input = String::new();
    println!("Apakah status perkawinan Anda? (Kawin, Lajang)");
    io::stdin().read_line(&mut status_perkawinan_input).unwrap();
    let status_perkawinan_str = status_perkawinan_input.trim().to_lowercase();
    let status_perkawinan = match status_perkawinan_str.as_str() {
        "lajang" => StatusPerkawinan::Lajang,
        "kawin" => StatusPerkawinan::Kawin,
        _ => {
            println!("Status perkawinan tidak diketahui, menggunakan default lajang");
            StatusPerkawinan::Lajang
        }
    };

    let mut ptkp = PTKP_BASE;
    if status_perkawinan == StatusPerkawinan::Kawin {
        ptkp = ptkp + PTKP_KAWIN_ADDITION;
    }

    let mut jumlah_tanggungan_input = String::new();
    println!("Berapa jumlah tanggungan Anda? (Maksimal 3)");
    std::io::stdin()
        .read_line(&mut jumlah_tanggungan_input)
        .unwrap();

    let mut jumlah_tanggungan: f64 = jumlah_tanggungan_input.trim().parse().expect("Harus angka");
    if jumlah_tanggungan > PTKP_MAX_TANGGUNGAN {
        jumlah_tanggungan = PTKP_MAX_TANGGUNGAN;
    }
    let ptkp_total = ptkp + (jumlah_tanggungan * PTKP_TANGGUNGAN_ADDITION);

    let data_wp = DataWajibPajak {
        gaji_bulanan,
        status_perkawinan,
        jumlah_tanggungan,
    };

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

    let hasil_tahunan = HitunganTahunan {
        bruto_tahunan: bruto_tahunan as i32,
        biaya_jabatan: biaya_jabatan_setahun as i32,
        netto_tahunan: netto_tahunan as i32,
        ptkp_total: ptkp_total as i32,
        pkp: pkp as i32,
        pajak_tahunan: pph21 as i32,
    };
    println!(
        "
=== Rekap Data Pajak ===
Gaji Bulanan : {}
Status Perkawinan : {:?}
Jumlah Tanggungan : {}
Bruto Tahunan : {}
Biaya Jabatan : {}
Netto Tahunan : {}
PTKP Total  : {}
PKP : {}
Pph 21 Tahunan : {}
",
        data_wp.gaji_bulanan,
        data_wp.status_perkawinan,
        data_wp.jumlah_tanggungan,
        hasil_tahunan.bruto_tahunan,
        hasil_tahunan.biaya_jabatan,
        hasil_tahunan.netto_tahunan,
        hasil_tahunan.ptkp_total,
        hasil_tahunan.pkp,
        hasil_tahunan.pajak_tahunan
    )
}
