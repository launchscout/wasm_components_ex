defmodule WasmComponentsEx.Test.FormHandler.Native do
  use Rustler, otp_app: :wasm_components_ex, crate: :form_handler, path: "test/support/form_handler"

  def instantiate(_store, _component), do: error()

  def handle_submit(_store, _instance, _form_data), do: error()

  def engine_new(_config), do: error()

  def new_component(_store, _component), do: error()

  def new_store(_options, _limits), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)

end
