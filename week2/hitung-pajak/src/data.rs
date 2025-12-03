#[derive(Debug, PartialEq)]
pub enum StatusPerkawinan {
    Lajang,
    Kawin,
}

#[derive(Debug)]
pub struct DataWajibPajak {
    pub gaji_bulanan: f64,
    pub status_perkawinan: StatusPerkawinan,
    pub jumlah_tanggungan: f64,
}

pub struct HitunganTahunan {
    pub bruto_tahunan: i32,
    pub biaya_jabatan: i32,
    pub netto_tahunan: i32,
    pub ptkp_total: f64,
    pub pkp: f64,
    pub pajak_tahunan: i32,
}
