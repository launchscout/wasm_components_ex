defmodule WasmComponentsEx.Test.TodoList do
  alias WasmComponentsEx.Test.TodoList.Native

  defstruct instance: nil, store: nil, reference: nil

  def init(%__MODULE__{instance: instance, store: store}), do: Native.init(store, instance)

  def add_todo(%__MODULE__{instance: instance, store: store}, item, list), do: Native.add_todo(store, instance, item, list)

  def new(component_bytes, options \\ []) do
    store = Native.new_store(%WasmComponentsEx.WasiOptions{}, %WasmComponentsEx.StoreLimits{})
    component = Native.new_component(store, component_bytes)
    instance = Native.instantiate(store, component)
    {:ok, %__MODULE__{instance: instance, store: store}}
  end

end
