use serde::{Deserialize, Serialize};
use yew_router::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub id: usize,
    pub done: bool,
    pub deleted: bool,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/Counter")]
    CounterDemo,
    #[at("/Todo")]
    TodoDemoIndex,
    #[at("/Todo/*")]
    TodoDemo,
    #[at("/*path")]
    NotFound { path: String },
}

#[derive(Clone, Routable, PartialEq)]
pub enum TodoRoute {
    #[at("/Todo")]
    Index,
    #[at("/Todo/Create")]
    CreateTodo,
    #[at("/Todo/:id")]
    ViewTodo { id: String },
}
