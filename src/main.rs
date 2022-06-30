use crate::geo::Point;
use crate::map_component::MapComponent;
use yew::{html, Component, Context, Html};

mod geo;
mod map_component;
mod osrm;

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Model
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let lat = Point(51.505, -0.09);

        html! {
            <MapComponent {lat} />
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
