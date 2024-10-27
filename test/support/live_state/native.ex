defmodule WasmComponentsEx.Test.LiveState.Native do
  use Rustler, otp_app: :wasm_components_ex, crate: :live_state, path: "test/support/live_state"

  def instantiate(_store, _component), do: error()

  def init(_store, _instance), do: error()

  def add_customer(_store, _instance, _customer, _state), do: error()

  def engine_new(_config), do: error()

  def new_component(_store, _component), do: error()

  def new_store(_options, _limits), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)

end
