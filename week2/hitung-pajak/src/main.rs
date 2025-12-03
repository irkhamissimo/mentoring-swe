mod data;
mod hitung;
mod konstanta;
use data::*;
use hitung::hitung_pph;
use num_format::{Locale, ToFormattedString};
use std::io;

fn main() {
    println!("Kalkulator Pajak Penghasilan");

    let mut gaji_input = String::new();
    println!("Berapa gaji bulanan Anda? (Dalam rupiah)");
    io::stdin().read_line(&mut gaji_input).expect("Failed");
    let gaji_bulanan: f64 = gaji_input.trim().parse().expect("Please enter a number");

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

    let mut jumlah_tanggungan_input = String::new();
    println!("Berapa jumlah tanggungan Anda? (Maksimal 3)");
    std::io::stdin()
        .read_line(&mut jumlah_tanggungan_input)
        .unwrap();

    let jumlah_tanggungan: f64 = jumlah_tanggungan_input.trim().parse().expect("Harus angka");

    let data_wp = DataWajibPajak {
        gaji_bulanan,
        status_perkawinan,
        jumlah_tanggungan,
    };

    let hasil = hitung_pph(&data_wp);

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
        (data_wp.gaji_bulanan.round() as i64).to_formatted_string(&Locale::id),
        data_wp.status_perkawinan,
        data_wp.jumlah_tanggungan,
        (hasil.bruto_tahunan).to_formatted_string(&Locale::id),
        (hasil.biaya_jabatan).to_formatted_string(&Locale::id),
        (hasil.netto_tahunan).to_formatted_string(&Locale::id),
        (hasil.ptkp_total.round() as i64).to_formatted_string(&Locale::id),
        (hasil.pkp.round() as i64).to_formatted_string(&Locale::id),
        (hasil.pajak_tahunan).to_formatted_string(&Locale::id)
    )
}
