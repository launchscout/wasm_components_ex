use paste::paste;
use rustler::NifResult;
use rustler::ResourceArc;
use std::sync::Mutex;
use wasm_components_ex::component::ComponentResource;
use wasm_components_ex::{wrap_component, wrap_function};
use wasm_components_ex::linker::build_linker;

use wasm_components_ex::store::{ComponentStoreData, ComponentStoreResource};
use wasmtime::component::{bindgen, Linker};
use wasmtime::{Config, Engine, Store};

bindgen!("todo-list" in "todo-list.wit");

wrap_component!(TodoList);
wrap_function!(TodoList, init -> Vec<String>);

// #[rustler::nif(name = "init")]
// pub fn init(
//     store_or_caller_resource: ResourceArc<ComponentStoreResource>,
//     todo_list_resource: ResourceArc<TodoListResource>,
// ) -> NifResult<Vec<String>> {
//     let store_or_caller: &mut Store<ComponentStoreData> =
//         &mut *(store_or_caller_resource.inner.lock().map_err(|e| {
//             rustler::Error::Term(Box::new(format!(
//                 "Could not unlock store_or_caller resource as the mutex was poisoned: {e}"
//             )))
//         })?);

//     let todo_list = &mut todo_list_resource.inner.lock().map_err(|e| {
//         rustler::Error::Term(Box::new(format!(
//             "Could not unlock todo_list resource as the mutex was poisoned: {e}"
//         )))
//     })?;

//     todo_list
//         .call_init(store_or_caller)
//         .map_err(|err| rustler::Error::Term(Box::new(err.to_string())))
// }

#[rustler::nif(name = "add_todo")]
pub fn add_todo(
    store_or_caller_resource: ResourceArc<ComponentStoreResource>,
    todo_list_resource: ResourceArc<TodoListResource>,
    todo: String,
    list: Vec<String>,
) -> NifResult<Vec<String>> {
    let store_or_caller: &mut Store<ComponentStoreData> =
        &mut *(store_or_caller_resource.inner.lock().map_err(|e| {
            rustler::Error::Term(Box::new(format!(
                "Could not unlock store_or_caller resource as the mutex was poisoned: {e}"
            )))
        })?);

    let todo_list = &mut todo_list_resource.inner.lock().map_err(|e| {
        rustler::Error::Term(Box::new(format!(
            "Could not unlock todo_list resource as the mutex was poisoned: {e}"
        )))
    })?;

    todo_list
        .call_add_todo(store_or_caller, &todo, &list)
        .map_err(|err| rustler::Error::Term(Box::new(err.to_string())))
}

rustler::init!("Elixir.WasmComponentsEx.Test.TodoList.Native");
