mod location;

use std::env;
use std::str::FromStr;

use crate::location::Location;

fn old_main() {
    let opt_animal: Option<String> = env::args().nth(1);
    let opt_loops: Option<String> = env::args().nth(2);

    if let Some(animal) = opt_animal {
        match animal.to_lowercase().as_str() {
            "dog" => println!("Bark"),
            "duck" => println!("Quack"),
            _ => println!("All quiet out here")
        }
    }

    if let Some(loops) = opt_loops {
        let range = match i32::from_str(loops.as_str()) {
            Ok(end) => 0..=end,
            _ => 0..=0
        };

        for i in range {
            println!("{:}", i)
        }
    }
}

fn traverse(route: &[&Location]) -> f64 {
    let mut total_distance = 0.0;
    let mut previous: Option<&Location> = None;

    for waypoint in route {
        total_distance += match previous {
            None => 0.0,
            Some(location) => {
                let result = location.distance_to(waypoint);
                println!("Distance from {} to {} => {:.1}", location.name, waypoint.name, result);
                result
            }
        };

        previous = Some(waypoint);
    }

    println!("Total distance traveled => {:.1}", total_distance);
    return total_distance;
}

fn main() {
    let bff = Location::new("BFF", 41.89420, -103.48200);
    let bryto = Location::new("BRYTO", 41.74170, -85.31320);
    let coton = Location::new("COTON", 42.31990, -89.31220);
    let dbq = Location::new("DBQ", 42.40150, -90.70910);
    let fod = Location::new("FOD", 42.61110, -94.29480);
    let gij = Location::new("GIJ", 41.76860, -86.31850);
    let kcle = Location::new("KCLE", 61.4075, -81.851111);
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

    traverse(&[
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
    ]);
}
