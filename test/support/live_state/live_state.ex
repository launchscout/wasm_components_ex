defmodule WasmComponentsEx.Test.LiveState do
  alias WasmComponentsEx.Test.LiveState.Native

  defstruct instance: nil, store: nil, reference: nil

  def init(%__MODULE__{instance: instance, store: store}), do: Native.init(store, instance)

  def add_customer(%__MODULE__{instance: instance, store: store}, customer, state), do: Native.add_todo(store, instance, customer, state)

  def new(component_bytes, options \\ []) do
    store = Native.new_store(%WasmComponentsEx.WasiOptions{}, %WasmComponentsEx.StoreLimits{})
    component = Native.new_component(store, component_bytes)
    instance = Native.instantiate(store, component)
    {:ok, %__MODULE__{instance: instance, store: store}}
  end

end
