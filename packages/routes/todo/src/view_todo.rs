use yew::prelude::*;
use yew_app_entities::TodoRoute;
use yew_app_services::{delete_todo, get_todo_by_id, toggle_done_todo};
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ViewTodoProps {
    pub id: String,
}

#[function_component(ViewTodo)]
pub fn view_todo(props: &ViewTodoProps) -> Html {
    let ViewTodoProps { id } = props;

    let navigator = use_navigator().unwrap();

    let on_back = {
        let n = navigator.clone();
        Callback::from(move |_| n.push(&TodoRoute::Index))
    };

    let id = match id.parse::<usize>() {
        Ok(k) => k,
        Err(_) => {
            navigator.push(&TodoRoute::Index);
            return html! {};
        }
    };
    let todo = match get_todo_by_id(id) {
        Ok(t) => t,
        Err(_) => {
            navigator.push(&TodoRoute::Index);
            return html! {};
        }
    };

    let on_done = {
        let n = navigator.clone();
        Callback::from(move |_| {
            toggle_done_todo(todo.id).expect("tried to toggle todo that was available");
            n.push(&TodoRoute::Index)
        })
    };

    let on_delete = {
        let n = navigator.clone();
        Callback::from(move |_| {
            delete_todo(todo.id).expect("tried to toggle todo that was available");
            n.push(&TodoRoute::Index)
        })
    };

    return html! {
        <div class="container w-full prose flex flex-col justify-center align-middle mx-auto mt-10">
            <div class="relative">
                <div class="card w-96 bg-primary text-primary-content">
                    <div class="card-body">
                        <div class="card-title">{todo.title}</div>
                        <p class="text-neutral-content">{todo.description}</p>
                        <div class="card-actions justify-end">
                            <button class="btn btn-success" id="done" onclick={on_done}>{if !todo.done {"Done"} else {"Enable"}}</button>
                            <button class="btn btn-error" id="delete" onclick={on_delete}>{"Delete"}</button>
                        </div>
                    </div>
                 </div>
                <div class="absolute -top-5 -left-5">
                    <button class="btn btn-circle btn-secondary" id="back" onclick={on_back}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                          <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" />
                        </svg>
                    </button>
                </div>
            </div>
        </div>
    };
}
