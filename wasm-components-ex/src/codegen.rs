#[macro_export]
macro_rules! define_instantiate {
  ($component:ident, $resource:ident) => {
      #[rustler::nif(name = "instantiate")]
      pub fn instantiate(
          component_store_resource: ResourceArc<ComponentStoreResource>,
          component_resource: ResourceArc<ComponentResource>,
      ) -> NifResult<ResourceArc<$resource>> {
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

          Ok(ResourceArc::new($resource {
              inner: Mutex::new(form_handler_instance),
          }))
      }
  };
}