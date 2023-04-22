use yew::prelude::*;
use yew_app_entities::Route;
use yew_router::prelude::use_navigator;

#[function_component(Counter)]
pub fn counter() -> Html {
    let counter = use_state(|| 0);
    let on_click_increase = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    let on_click_decrease = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };
    let navigator = use_navigator().unwrap();
    let on_back = { Callback::from(move |_| navigator.push(&Route::Home)) };

    return html! {
            <div class="card w-96 bg-primary shadow-xl mx-auto mt-40 relative">
                <div class="absolute -top-5 -left-5">
                    <button class="btn btn-circle btn-secondary" id="back" onclick={on_back}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                          <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" />
                        </svg>
                    </button>
                </div>
                <span class="card-title mx-auto pt-4">{"Counter"}</span>
                <div class="card-body">
                    <p class="text-center">{"Click the buttons to increase"}</p>
                    <p class="text-center font-bold text-6xl">{ *counter }</p>
                </div>
                <div class="card-actions justify-center p-2">
                    <button  class="btn btn-circle" onclick={on_click_decrease}>{"-1"}</button>
                    <button class="btn btn-circle" onclick={on_click_increase}>{"+1"}</button>
                </div>
            </div>
    };
}
