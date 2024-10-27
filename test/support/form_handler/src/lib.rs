use rustler::NifResult;
use rustler::ResourceArc;
use std::sync::Mutex;
use paste::paste;
use wasm_components_ex::component::ComponentResource;
use wasm_components_ex::linker::build_linker;
use wasm_components_ex::{wrap_component, wrap_function};
use wasm_components_ex::store::{
    ComponentStoreData, ComponentStoreResource, ExStoreLimits, ExWasiOptions,
};
use wasmtime::component::{bindgen, Linker};
use wasmtime::{Config, Engine, Store};

bindgen!("form-handler" in "form-handler.wit");

wrap_component!(FormHandler);
wrap_function!(FormHandler, handle_submit, String, form_values: Vec<FormValue>);

rustler::init!("Elixir.WasmComponentsEx.Test.FormHandler.Native");
