use yew::prelude::*;
use yew_router::prelude::*;

use super::{index::Index, not_found::NotFound};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/Todo")]
    CreateTodo,
    #[at("/Todo/:id")]
    ViewTodo { id: String },
    #[at("/*path")]
    NotFound { path: String },
}

pub fn route_switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Index/>},
        Route::CreateTodo => todo!(),
        Route::ViewTodo { id } => todo!(),
        Route::NotFound { path } => html! {<NotFound {path}/>},
    }
}
