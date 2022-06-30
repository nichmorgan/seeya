use super::response::NearestResponse;
use crate::geo::Point;

use super::waypoint::Waypoint;

pub struct OsrmClient {
    client: reqwest::blocking::Client,
    host: String,
}

impl OsrmClient {
    pub fn new(host: &str) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            host: host.to_string(),
        }
    }

    pub fn nearest(
        &self,
        profile: &str,
        point: &Point,
    ) -> Result<Waypoint, Box<dyn std::error::Error>> {
        let q = format!(
            "http://{}/nearest/v1/{}/{:?}.json?number={}",
            self.host, profile, point, 1
        );

        let body = self.client.get(q).send()?.json()?;
        let waypoint = serde_json::from_str::<NearestResponse>(&body)?
            .waypoints
            .into_iter()
            .nth(0)
            .unwrap();

        Ok(waypoint)
    }
}
