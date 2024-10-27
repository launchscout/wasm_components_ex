#[macro_export]
macro_rules! wrap_component {
    ($component:ident) => {
        paste! {
          pub struct [<$component Resource>] {
              pub inner: Mutex<$component>,
          }

          #[rustler::resource_impl()]
          impl rustler::Resource for [<$component Resource>] {}

          #[rustler::nif(name = "instantiate")]
          pub fn instantiate(
              component_store_resource: ResourceArc<ComponentStoreResource>,
              component_resource: ResourceArc<ComponentResource>,
          ) -> NifResult<ResourceArc<[<$component Resource>]>> {
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

              let linker = build_linker(component_store);
              let form_handler_instance =
                  $component::instantiate(component_store, &component, &linker)
                      .map_err(|err| rustler::Error::Term(Box::new(err.to_string())))?;

              Ok(ResourceArc::new([<$component Resource>] {
                  inner: Mutex::new(form_handler_instance),
              }))
          }
        }
    };
}

#[macro_export]
macro_rules! wrap_function {
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
  ($module_name:ident, $function_name:ident, $ret:ty, $($argName:ident: $argType:ty),+) => {
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
