defmodule WasmComponentsExTest do
  alias WasmComponentsEx.Test.TodoList
  alias WasmComponentsEx.Test.FormHandler
  alias WasmComponentsEx.Test.LiveState
  use ExUnit.Case

  test "todo list" do
    component_bytes = File.read!("test/support/todo_list/todo-list.wasm")
    {:ok, todo_list} = TodoList.new(component_bytes)
    assert [first, second] = TodoList.init(todo_list)
    assert ["foo", "bar"] = TodoList.add_todo(todo_list, "foo", ["bar"])
  end

  test "form handler" do
    component_bytes = File.read!("test/support/form_handler/form-handler.wasm")
    {:ok, form_handler} = FormHandler.new(component_bytes)
    assert result = FormHandler.handle_submit(form_handler, [{"foo", ["bar"]}])
    assert result =~ "message"
  end

  test "live state" do
    component_bytes = File.read!("test/support/live_state/live-state.wasm")
    {:ok, live_state} = LiveState.new(component_bytes)
    assert %{customers: customers} = LiveState.init(live_state)
    IO.inspect(customers)
  end

end
