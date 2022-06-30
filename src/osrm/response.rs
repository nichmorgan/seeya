use crate::osrm::waypoint::Waypoint;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NearestResponse {
    pub code: String,
    pub waypoints: Vec<Waypoint>,
}
