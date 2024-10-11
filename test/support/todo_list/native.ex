defmodule WasmComponentsEx.Test.TodoList.Native do
  use Rustler, otp_app: :wasm_components_ex, crate: :todo_list, path: "test/support/todo_list"

  def instantiate(_store, _component), do: error()

  def init(_store, _instance), do: error()

  def add_todo(_store, _instance, _item, _list), do: error()

  def engine_new(_config), do: error()

  def new_component(_store, _component), do: error()

  def new_store(_options, _limits), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)

end
