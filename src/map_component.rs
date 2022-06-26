use leaflet::{LatLng, Map, TileLayer};
use yew::{html, Component, Html, Properties};

const ZOOM: f64 = 11.0;

pub enum Msg {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(pub f64, pub f64);

#[derive(Properties, PartialEq)]
pub struct Props {
    pub lat: Point,
}

pub struct MapComponent {
    map: Option<Map>,
    lat: Point,
}

impl Component for MapComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            map: None,
            lat: ctx.props().lat,
        }
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            let mut map = Map::new("map");
            map.set_view(LatLng::new(self.lat.0, self.lat.1), ZOOM);
            add_tile_layer(&map);
            self.map = Some(map);
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        if self.lat != ctx.props().lat {
            self.lat = ctx.props().lat;
            self.map
                .as_mut()
                .unwrap()
                .set_view(LatLng::new(self.lat.0, self.lat.1), ZOOM);
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

fn add_tile_layer(map: &Map) {
    TileLayer::new_with_options(
        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png".to_string(),
        serde_json::Value::Null,
    )
    .add_to(map);
}
