use yew::prelude::*;
use yew_router::prelude::*;

mod routes;
use routes::{route_switch, Route};

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={route_switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
  yew::set_event_bubbling(false);
  yew::Renderer::<Main>::new().render();
}
