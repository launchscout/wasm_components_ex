defmodule WasmComponentsEx.WasiOptions do
  @moduledoc ~S"""
  Configures WASI support for a Wasmex.Store.

  ## Options

    * `:args` - A list of command line arguments
    * `:env` - A map of environment variables

  ## Example

      iex> Wasmex.Store.new_wasi(%WasiOptions{
      ...>   args: ["first param", "second param"],
      ...>   env: %{"env_key" => "env_value"}
      ...> })
  """

  defstruct [args: [], env: %{}]

  @type t :: %__MODULE__{
          args: [String.t()],
          env: %{String.t() => String.t()}
        }
end
