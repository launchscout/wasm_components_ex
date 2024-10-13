use rustler::NifResult;
use rustler::ResourceArc;
use std::sync::Mutex;
use wasm_components_ex::component::ComponentResource;
use wasm_components_ex::linker::build_linker;
use wasm_components_ex::store::{ComponentStoreData, ComponentStoreResource, ExWasiOptions, ExStoreLimits};
use wasmtime::component::{bindgen, Linker};
use wasmtime::{Config, Engine, Store};

bindgen!("form-handler" in "form-handler.wit");

pub struct FormHandlerResource {
    pub inner: Mutex<FormHandler>,
}

#[rustler::resource_impl()]
impl rustler::Resource for FormHandlerResource {}

// macro_rules! define_instantiate {
//   ($component:ident) => {

//   }
// }

#[rustler::nif(name = "handle_submit")]
pub fn handle_submit(
    store_or_caller_resource: ResourceArc<ComponentStoreResource>,
    form_handler_resource: ResourceArc<FormHandlerResource>,
    form_values: Vec<FormValue>,
) -> NifResult<String> {
    let store_or_caller: &mut Store<ComponentStoreData> =
        &mut *(store_or_caller_resource.inner.lock().map_err(|e| {
            rustler::Error::Term(Box::new(format!(
                "Could not unlock store_or_caller resource as the mutex was poisoned: {e}"
            )))
        })?);

    let form_handler = &mut form_handler_resource.inner.lock().map_err(|e| {
        rustler::Error::Term(Box::new(format!(
            "Could not unlock todo_list resource as the mutex was poisoned: {e}"
        )))
    })?;

    form_handler
        .call_handle_submit(store_or_caller, &form_values)
        .map_err(|err| rustler::Error::Term(Box::new(err.to_string())))
}

rustler::init!("Elixir.WasmComponentsEx.Test.FormHandler.Native");
