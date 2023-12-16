// Defines a struct for location, lat and lon, to be used in my haversine function
#[derive(Copy, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}
// Defines the units (I will use kilometers) for my haversine function
pub enum Units {
    Km,
}
const EARTH_RADIUS_KM: f64 = 6371.0;
// Uses the haversine formula to calculate distance using lon and lat
// Used typically globally (great circle distance)
pub fn haversine(start: Location, end: Location, units: Units) -> f64 {
    let lat_rad_1 = start.latitude.to_radians();
    let lat_rad_2 = end.latitude.to_radians();
    let delta_lat_rad = (end.latitude - start.latitude).to_radians();
    let delta_lon_rad = (end.longitude - start.longitude).to_radians();

    let a = (delta_lat_rad / 2.0).sin().powi(2) + lat_rad_1.cos() * lat_rad_2.cos() * (delta_lon_rad / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    match units {
        Units::Km => EARTH_RADIUS_KM * c,
    }
}