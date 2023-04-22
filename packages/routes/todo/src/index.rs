use yew::prelude::*;
use yew_app_entities::{Route, TodoRoute};
use yew_app_services::get_all_todos;
use yew_router::prelude::*;

#[function_component(Todos)]
pub fn todo_view() -> Html {
    let todos = get_all_todos();
    let navigator = use_navigator().unwrap();
    let on_create_todo = {
        let n = navigator.clone();
        Callback::from(move |_| n.push(&TodoRoute::CreateTodo))
    };
    let on_back = {
        let n = navigator.clone();
        Callback::from(move |_| n.push(&Route::Home))
    };

    let through = |done| if done { Some("text-info") } else { None };

    return html! {
    <div class="card w-96 bg-primary text-primary-content mx-auto mt-40 relative">
        <div class="absolute -top-5 -left-5">
            <button class="btn btn-circle btn-secondary" id="back" onclick={on_back}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" />
                </svg>
            </button>
        </div>
        <div class="absolute top-4 right-2">
            <button class="btn btn-circle btn-base btn-sm" id="add" onclick={on_create_todo}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                </svg>
            </button>
        </div>
        <div class="card-title justify-center p-4">
            <h1>{"Todos:"}</h1>
        </div>
        <div class="card-body bg-base-300">
            <ul class="menu">
                {todos.iter().map(|todo| {
                    html!{
                        <li key={todo.id} class={classes!(through(todo.done), Some("hover-bordered"))}><Link<TodoRoute> to={TodoRoute::ViewTodo{id: todo.id.to_string()}}>{todo.title.clone()}</Link<TodoRoute>></li>
                    }
                }).collect::<Html>()}
            </ul>
            <span>{format!("Legth: {}", todos.len())}</span>
        </div>
    </div>
    };
}
