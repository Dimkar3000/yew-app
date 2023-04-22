use std::{error::Error, io::ErrorKind};

use gloo_storage::Storage;
use std::io;
use yew_app_entities::Todo;

pub fn toggle_done_todo(id: usize) -> Result<(), Box<dyn Error>> {
    let todos = gloo_storage::LocalStorage::get::<Vec<Todo>>("todoes")?
        .iter()
        .cloned()
        .map(|mut x| {
            if x.id == id {
                x.done = !x.done;
                x
            } else {
                x
            }
        })
        .collect::<Vec<_>>();

    gloo_storage::LocalStorage::set("todoes", todos)?;

    Ok(())
}

pub fn delete_todo(id: usize) -> Result<(), Box<dyn Error>> {
    let todos = gloo_storage::LocalStorage::get::<Vec<Todo>>("todoes")?
        .iter()
        .filter(|x| x.id != id)
        .cloned()
        .collect::<Vec<_>>();

    gloo_storage::LocalStorage::set("todoes", todos)?;

    Ok(())
}

pub fn create_todo(title: String, description: String) -> Todo {
    let todos_length = gloo_storage::LocalStorage::get::<usize>("todoIdgen").unwrap_or_default();

    let todo = Todo {
        title,
        description,
        id: todos_length,
        done: false,
        deleted: false,
    };

    gloo_storage::LocalStorage::set("todoIdgen", todos_length + 1)
        .expect("failed to store id counter");
    let mut todos = get_all_todos();
    todos.push(todo.clone());
    gloo_storage::LocalStorage::set::<Vec<Todo>>("todoes", todos).expect("failed to store todo");

    todo
}

pub fn get_todo_by_id(id: usize) -> Result<Todo, Box<dyn Error>> {
    let todos = gloo_storage::LocalStorage::get::<Vec<Todo>>("todoes")?;
    let todo = todos.iter().filter(|x| x.id == id).next();
    match todo {
        Some(t) => Ok(t.clone()),
        None => Err(Box::new(io::Error::new(ErrorKind::NotFound, "not found"))),
    }
}

pub fn get_all_todos() -> Vec<Todo> {
    gloo_storage::LocalStorage::get::<Vec<Todo>>("todoes").unwrap_or_default()
}
