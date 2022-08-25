defmodule MVar do
  use Rustler,
    otp_app: :accumulators,
    crate: :mvar,
    mode: :release

  def new(_val), do: nif_error()
  def get(_var), do: nif_error()
  def set(_var, _val), do: nif_error()

  defp nif_error(), do: :erlang.nif_error(:nif_not_loaded)
end
