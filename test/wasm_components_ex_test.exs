defmodule WasmComponentsExTest do
  alias WasmComponentsEx.Test.TodoList
  use ExUnit.Case

  test "todo list" do
    component_bytes = File.read!("test/support/todo_list/todo-list.wasm")
    {:ok, todo_list} = TodoList.new(component_bytes)
    assert [first, second] = TodoList.init(todo_list)
    assert ["foo", "bar"] = TodoList.add_todo(todo_list, "foo", ["bar"])
  end
end
