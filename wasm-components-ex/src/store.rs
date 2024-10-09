use crate::{
    engine::{unwrap_engine, EngineResource},
};
use rustler::{Error, NifStruct, ResourceArc};
use wasmtime_wasi::{WasiCtx, WasiView, ResourceTable};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};
use std::{collections::HashMap, sync::Mutex};
use wasi_common::{sync::WasiCtxBuilder};
use wasmtime::{
    AsContext, AsContextMut, Config, Engine, Store, StoreContext, StoreContextMut, StoreLimits, StoreLimitsBuilder
};

#[derive(NifStruct)]
#[module = "WasmComponentsEx.WasiOptions"]
pub struct ExWasiOptions {
    args: Vec<String>,
    env: HashMap<String, String>,
}

#[derive(NifStruct)]
#[module = "WasmComponentsEx.StoreLimits"]
pub struct ExStoreLimits {
    memory_size: Option<usize>,
    table_elements: Option<u32>,
    instances: Option<usize>,
    tables: Option<usize>,
    memories: Option<usize>,
}

impl ExStoreLimits {
    pub fn to_wasmtime(&self) -> StoreLimits {
        let limits = StoreLimitsBuilder::new();

        let limits = if let Some(memory_size) = self.memory_size {
            limits.memory_size(memory_size)
        } else {
            limits
        };

        let limits = if let Some(table_elements) = self.table_elements {
            limits.table_elements(table_elements)
        } else {
            limits
        };

        let limits = if let Some(instances) = self.instances {
            limits.instances(instances)
        } else {
            limits
        };

        let limits = if let Some(tables) = self.tables {
            limits.tables(tables)
        } else {
            limits
        };

        let limits = if let Some(memories) = self.memories {
            limits.memories(memories)
        } else {
            limits
        };

        limits.build()
    }
}

pub struct ComponentStoreData {
    pub(crate) ctx: WasiCtx,
    pub(crate) http: WasiHttpCtx,
    pub(crate) limits: StoreLimits,
    pub(crate) table: ResourceTable,
}

impl WasiHttpView for ComponentStoreData {
  fn ctx(&mut self) -> &mut WasiHttpCtx { &mut self.http }
  fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}

impl WasiView for ComponentStoreData {
  fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
  fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}

pub struct ComponentStoreResource {
  pub inner: Mutex<Store<ComponentStoreData>>,
}

#[rustler::resource_impl()]
impl rustler::Resource for ComponentStoreResource {}

#[rustler::nif(name = "new_store")]
pub fn new_store(
    options: ExWasiOptions,
    limits: Option<ExStoreLimits>
) -> Result<ResourceArc<ComponentStoreResource>, rustler::Error> {
    let wasi_env = &options
        .env
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<Vec<_>>();
    let mut builder = wasmtime_wasi::WasiCtxBuilder::new();
    let wasi_ctx = builder
        .args(&options.args)
        .envs(wasi_env)
        .inherit_stdin()
        .inherit_stdout()
        .inherit_stderr()
        .inherit_network()
        .allow_ip_name_lookup(true)
        .build();
    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();

    let limits = if let Some(limits) = limits {
        limits.to_wasmtime()
    } else {
        StoreLimits::default()
    };
    let mut store = Store::new(
        &engine,
        ComponentStoreData {
            ctx: wasi_ctx,
            limits,
            http: WasiHttpCtx::new(),
            table: wasmtime_wasi::ResourceTable::new(),
        },
    );
    store.limiter(|state| &mut state.limits);
    let resource: ResourceArc<ComponentStoreResource> = ResourceArc::new(ComponentStoreResource {
        inner: Mutex::new(store),
    });
    Ok(resource)
}
