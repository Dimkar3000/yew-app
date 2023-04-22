use yew::prelude::*;
use yew_app_entities::Route;
use yew_app_router::route_switch;
use yew_router::prelude::*;

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={route_switch} />
        </BrowserRouter>
    }
}

fn main() {
    // yew::set_event_bubbling(false);
    yew::Renderer::<Main>::new().render();
}
