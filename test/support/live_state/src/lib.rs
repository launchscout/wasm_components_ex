use paste::paste;
use rustler::NifResult;
use rustler::ResourceArc;
use std::sync::Mutex;
use wasm_components_ex::component::ComponentResource;
use wasm_components_ex::linker::build_linker;
use wasm_components_ex::{wrap_component, wrap_function};

use wasm_components_ex::store::{ComponentStoreData, ComponentStoreResource};
use wasmtime::component::{bindgen, Linker};
use wasmtime::{Config, Engine, Store};

bindgen!({world: "live-state", path: "live-state.wit", additional_derives: [rustler::NifMap]});

wrap_component!(LiveState);
wrap_function!(LiveState, init -> State);
wrap_function!(LiveState, add_customer, State, customer: Customer, state: State);

rustler::init!("Elixir.WasmComponentsEx.Test.LiveState.Native");
