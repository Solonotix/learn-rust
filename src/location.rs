const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

pub struct Location<'a> {
    pub latitude: f64,
    pub longitude: f64,
    pub name: &'a str
}

impl <'a> Location<'a> {
    pub fn new(name: &'a str, latitude: f64, longitude: f64) -> Self {
        Location { name, latitude, longitude }
    }

    pub fn distance_to(&self, other: &Location) -> f64 {
        let delta_latitude = (self.latitude - other.latitude).to_radians();
        let delta_longitude = (self.longitude - other.longitude).to_radians();
        let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2) +
            self.latitude.to_radians().cos() * other.latitude.to_radians().cos() *
                f64::powi((delta_longitude / 2.0).sin(), 2);

        return EARTH_RADIUS_IN_KILOMETERS * 2.0 * inner_central_angle.sqrt().asin();
    }
}