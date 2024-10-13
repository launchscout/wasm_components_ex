use wasmtime::{component::Linker, Store};

use crate::store::ComponentStoreData;

pub fn build_linker(store: &mut Store<ComponentStoreData>) -> Linker<ComponentStoreData> {
  let mut linker = Linker::new(store.engine());
  wasmtime_wasi::add_to_linker_sync(&mut linker);
  wasmtime_wasi_http::add_only_http_to_linker_sync(&mut linker);
  linker
}
