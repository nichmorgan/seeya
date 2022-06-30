use leaflet;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct Point(pub f64, pub f64);

impl From<Point> for leaflet::LatLng {
    fn from(p: Point) -> Self {
        leaflet::LatLng::new(p.0, p.1)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}
