use crate::geo::Point;
use crate::osrm::client::OsrmClient;
use leaflet;
use serde_json::json;
use wasm_bindgen::prelude::*;
use yew::{html, Component, Html, Properties};
const ZOOM: f64 = 18.0;

pub enum Msg {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub lat: Point,
}

pub struct MapComponent {
    osrm: OsrmClient,
    map: Option<leaflet::Map>,
    lat: Point,
}

impl MapComponent {
    pub fn draw_line(&self, points: Vec<Point>) -> leaflet::Polyline {
        let points = points
            .into_iter()
            .map(|v| json!([v.0, v.1]))
            .map(|v| JsValue::from_serde(&v).unwrap())
            .collect::<Vec<JsValue>>();

        let options = JsValue::from_serde(&json!({"color": "red"})).unwrap();
        let polyline = leaflet::Polyline::new_with_options(points, &options);
        polyline.addTo(self.map.as_ref().as_mut().unwrap());

        polyline
    }
}

impl Component for MapComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            osrm: OsrmClient::new("router.project-osrm.org"),
            map: None,
            lat: ctx.props().lat,
        }
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            let map = leaflet::Map::new("map", &JsValue::NULL);
            map.setView(&leaflet::LatLng::new(self.lat.0, self.lat.1), ZOOM);
            add_tile_layer(&map);

            let points = vec![
                Point(self.lat.0, self.lat.1),
                Point(self.lat.0, self.lat.1 + 0.003),
            ];

            let waypoints = points
                .into_iter()
                .map(|p| self.osrm.nearest("driving", &p).unwrap())
                .map(|w| w.location)
                .collect::<Vec<Point>>();

            self.map = Some(map);
            let polyine = self.draw_line(waypoints);
            self.map.as_ref().unwrap().fitBounds(&polyine.getBounds());
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        if self.lat != ctx.props().lat {
            self.lat = ctx.props().lat;
            self.map
                .as_mut()
                .unwrap()
                .setView(&leaflet::LatLng::new(self.lat.0, self.lat.1), ZOOM);
            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
            <div id="map"></div>
        }
    }
}

fn add_tile_layer(map: &leaflet::Map) {
    leaflet::TileLayer::new(
        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
        &JsValue::NULL,
    )
    .addTo(map);
}
