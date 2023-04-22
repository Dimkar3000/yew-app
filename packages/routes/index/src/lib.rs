use yew::prelude::*;
use yew_app_entities::Route;
use yew_router::prelude::Link;

#[function_component(Index)]
pub fn index() -> Html {
    return html! {
    <div class="flex gap-8 justify-around h-screen items-center text-primary-content">
        <div class="w-[40rem] h-[40rem] card bg-base-300">
            <div class="card-body items-center justify-center">
                <h2 class="card-title text-5xl">{"Counter"}</h2>
                <div>{"The classic Counter example to get you started with reactivity"}</div>
                <div class="card-actions p-4">
                    <Link<Route> to={Route::CounterDemo} classes="btn">{"View Demo"}</Link<Route>>
                </div>
            </div>
        </div>
        <div class="w-[40rem] h-[40rem] card bg-base-300">
            <div class="card-body items-center justify-center">
                <h2 class="card-title text-5xl">{"Todos"}</h2>
                <div>{"CRUD Operations on a todo list using local storage"}</div>
                <div class="card-actions p-4">
                    <Link<Route> to={Route::TodoDemoIndex} classes="btn">{"View Demo"}</Link<Route>>
                </div>
            </div>
        </div>

    </div>
    };
}
