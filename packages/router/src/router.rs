use crate::not_found::NotFound;
use yew::prelude::*;
use yew_app_counter_demo::Counter;
use yew_app_entities::{Route, TodoRoute};
use yew_app_route_index::Index;
use yew_app_todo_demo::{CreateTodo, Todos, ViewTodo};
use yew_router::Switch;

pub fn route_switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Index/>},
        Route::CounterDemo => html! {<Counter/>},
        Route::TodoDemo | Route::TodoDemoIndex => {
            html! {<Switch<TodoRoute> render={todo_route_switch}/>}
        }
        Route::NotFound { path } => html! {<NotFound {path}/>},
    }
}

pub fn todo_route_switch(routes: TodoRoute) -> Html {
    match routes {
        TodoRoute::Index => html! {<Todos/>},
        TodoRoute::CreateTodo => html! {<CreateTodo/>},
        TodoRoute::ViewTodo { id } => html! {<ViewTodo {id}/>},
    }
}
