use paste::paste;
use rustler::NifResult;
use rustler::ResourceArc;
use std::sync::Mutex;
use wasm_components_ex::component::ComponentResource;
use wasm_components_ex::{define_instantiate, define_function};
use wasm_components_ex::linker::build_linker;

use wasm_components_ex::store::{ComponentStoreData, ComponentStoreResource};
use wasmtime::component::{bindgen, Linker};
use wasmtime::{Config, Engine, Store};

bindgen!({world: "live-state", path: "live-state.wit", additional_derives: [rustler::NifMap]});

pub struct LiveStateResource {
    pub inner: Mutex<LiveState>,
}

#[rustler::resource_impl()]
impl rustler::Resource for LiveStateResource {}

define_instantiate!(LiveState, LiveStateResource);
define_function!(LiveState, init -> State);
define_function!(LiveState, add_customer, customer: Customer, state: State -> State);

rustler::init!("Elixir.WasmComponentsEx.Test.LiveState.Native");
