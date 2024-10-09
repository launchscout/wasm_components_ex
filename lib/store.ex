defmodule WasmComponentsEx.Store do

  defstruct resource: nil, reference: nil

  def __wrap_resource__(resource) do
    %__MODULE__{
      resource: resource,
      reference: make_ref()
    }
  end

  def new(native_module, options, store_limits) do
    case native_module.new_store(
           options,
           store_limits
         ) do
      {:error, err} -> {:error, err}
      resource -> {:ok, __MODULE__.__wrap_resource__(resource)}
    end
  end
end
