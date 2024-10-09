use wasm_components_ex::component::ComponentResource;
use wasm_components_ex::store::{
    ComponentStoreData, ComponentStoreResource,
};
use rustler::NifResult;
use rustler::ResourceArc;
use std::sync::Mutex;
use wasmtime::component::{bindgen, Linker};
use wasmtime::{Config, Engine, Store};

bindgen!("todo-list" in "todo-list.wit");

pub struct TodoListResource {
    pub inner: Mutex<TodoList>,
}

#[rustler::resource_impl()]
impl rustler::Resource for TodoListResource {}

#[rustler::nif(name = "instantiate")]
pub fn instantiate(
    component_store_resource: ResourceArc<ComponentStoreResource>,
    component_resource: ResourceArc<ComponentResource>,
) -> NifResult<ResourceArc<TodoListResource>> {
    let component_store: &mut Store<ComponentStoreData> =
        &mut *(component_store_resource.inner.lock().map_err(|e| {
            rustler::Error::Term(Box::new(format!(
                "Could not unlock component_store resource as the mutex was poisoned: {e}"
            )))
        })?);

    let component = &mut component_resource.inner.lock().map_err(|e| {
        rustler::Error::Term(Box::new(format!(
            "Could not unlock component resource as the mutex was poisoned: {e}"
        )))
    })?;


    let mut linker = Linker::new(component_store.engine());
    wasmtime_wasi::add_to_linker_sync(&mut linker);
    wasmtime_wasi_http::add_only_http_to_linker_sync(&mut linker);
    let todo_instance = TodoList::instantiate(component_store, &component, &linker)
        .map_err(|err| rustler::Error::Term(Box::new(err.to_string())))?;

    Ok(ResourceArc::new(TodoListResource {
        inner: Mutex::new(todo_instance),
    }))
}

#[rustler::nif(name = "init")]
pub fn init(
    store_or_caller_resource: ResourceArc<ComponentStoreResource>,
    todo_list_resource: ResourceArc<TodoListResource>,
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
        .call_init(store_or_caller)
        .map_err(|err| rustler::Error::Term(Box::new(err.to_string())))
}

#[rustler::nif(name = "add_todo")]
pub fn add_todo(
    store_or_caller_resource: ResourceArc<ComponentStoreResource>,
    todo_list_resource: ResourceArc<TodoListResource>,
    todo: String,
    list: Vec<String>
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
