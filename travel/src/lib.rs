mod location;

use crate::location::Location;

pub fn main() {
    let bff = Location::new("BFF", 41.89420, -103.48200);
    let bryto = Location::new("BRYTO", 41.74170, -85.31320);
    let coton = Location::new("COTON", 42.31990, -89.31220);
    let dbq = Location::new("DBQ", 42.40150, -90.70910);
    let fod = Location::new("FOD", 42.61110, -94.29480);
    let gij = Location::new("GIJ", 41.76860, -86.31850);
    let kcle = Location::new("KCLE", 41.4075, -81.851111);
    let kslc = Location::new("KSLC", 40.7861, -111.9822);
    let leyir = Location::new("LEYIR", 41.51030, -83.88080);
    let modem = Location::new("MODEM", 41.72800, -84.89730);
    let nepts = Location::new("NEPTS", 41.96750, -87.05300);
    let obk = Location::new("OBK", 42.22140, -87.95160);
    let ocs = Location::new("OCS", 41.59020, -109.01500);
    let onl = Location::new("ONL", 42.47050, -98.68690);
    let pions = Location::new("PIONS", 41.65390, -84.48190);
    let pudvy = Location::new("PUDVY", 41.54270, -109.34200);
    let sewto = Location::new("SEWTO", 41.74780, -85.51130);
    let thorr = Location::new("THORR", 42.12330, -87.60030);
    let viggr = Location::new("VIGGR", 42.55520, -93.12410);
    let wegem = Location::new("WEGEM", 41.44560, -109.99000);
    let zoser = Location::new("ZOSER", 41.72390, -84.78130);

    Location::traverse(&[
        &kcle,
        &leyir,
        &pions,
        &zoser,
        &modem,
        &bryto,
        &sewto,
        &gij,
        &nepts,
        &thorr,
        &obk,
        &coton,
        &dbq,
        &viggr,
        &fod,
        &onl,
        &bff,
        &ocs,
        &pudvy,
        &wegem,
        &kslc
    ], true);
}
