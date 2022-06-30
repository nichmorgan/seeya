use crate::geo::Point;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Nodes(i64, i64);

#[derive(Deserialize, Debug)]
pub struct Waypoint {
    pub hint: String,
    pub nodes: Nodes,
    pub distance: f64,
    pub name: String,
    pub location: Point,
}
