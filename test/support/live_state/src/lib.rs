use paste::paste;
use rustler::NifResult;
use rustler::ResourceArc;
use std::sync::Mutex;
use wasm_components_ex::component::ComponentResource;
use wasm_components_ex::define_instantiate;
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

macro_rules! define_function {
    ($module_name:ident, $function_name:ident -> $ret:ty) => {
        paste! {
        #[rustler::nif]
          pub fn $function_name(
              store_or_caller_resource: ResourceArc<ComponentStoreResource>,
              live_state_resource: ResourceArc<[<$module_name Resource>]>
          ) -> NifResult<$ret> {
              let store_or_caller: &mut Store<ComponentStoreData> =
                  &mut *(store_or_caller_resource.inner.lock().map_err(|e| {
                      rustler::Error::Term(Box::new(format!(
                          "Could not unlock store_or_caller resource as the mutex was poisoned: {e}"
                      )))
                  })?);

              let live_state = &mut live_state_resource.inner.lock().map_err(|e| {
                  rustler::Error::Term(Box::new(format!(
                      "Could not unlock live_state resource as the mutex was poisoned: {e}"
                  )))
              })?;

                live_state
                .[<call_ $function_name>](store_or_caller)
                .map_err(|err| rustler::Error::Term(Box::new(err.to_string())))
          }
        }
    };
    ($module_name:ident, $function_name:ident, $($argName:ident: $argType:tt),+ -> $ret:ty) => {
      paste! {
      #[rustler::nif]
        pub fn $function_name(
            store_or_caller_resource: ResourceArc<ComponentStoreResource>,
            live_state_resource: ResourceArc<[<$module_name Resource>]>,
            $($argName: $argType),+
        ) -> NifResult<$ret> {
            let store_or_caller: &mut Store<ComponentStoreData> =
                &mut *(store_or_caller_resource.inner.lock().map_err(|e| {
                    rustler::Error::Term(Box::new(format!(
                        "Could not unlock store_or_caller resource as the mutex was poisoned: {e}"
                    )))
                })?);

            let live_state = &mut live_state_resource.inner.lock().map_err(|e| {
                rustler::Error::Term(Box::new(format!(
                    "Could not unlock live_state resource as the mutex was poisoned: {e}"
                )))
            })?;

              live_state
              .[<call_ $function_name>](store_or_caller, $(&$argName),+)
              .map_err(|err| rustler::Error::Term(Box::new(err.to_string())))
        }
      }
  };

}
define_instantiate!(LiveState, LiveStateResource);
define_function!(LiveState, init -> State);
define_function!(LiveState, add_customer, customer: Customer, state: State -> State);

rustler::init!("Elixir.WasmComponentsEx.Test.LiveState.Native");
