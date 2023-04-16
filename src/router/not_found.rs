use yew::prelude::*;
use yew_router::prelude::*;

use super::router::Route;

#[derive(PartialEq, Properties)]
pub struct NotFoundProps {
    pub path: String,
}

#[function_component(NotFound)]
pub fn not_found(props: &NotFoundProps) -> Html {
    let NotFoundProps { path } = props;
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
      <div class="hero prose mx-auto">
        <div class="artboard bg-white mt-20">
          <div class="mx-4 flex flex-col my-8">
            <h1 class="text-center"> {"Url not found?"} </h1>
            <p class="mx-auto w-full items-center font-bold text-center font-sans"> {format!("Path: \"/{path}\" is not a valid url.")}</p>
            <button class="btn btn-primary" {onclick}>
              { "Go home" }
          </button>
          </div>
        </div>
      </div>
    }
}