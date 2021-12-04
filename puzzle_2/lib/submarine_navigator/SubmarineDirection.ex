defmodule SubmarineNavigator.SubmarineDirection do
  use Agent

  def start_link(), do: Agent.start_link(fn -> 0 end)

  def value(pid), do: Agent.get(pid, & &1)

  def increase(pid, amount), do: Agent.update(pid, &(&1 + amount))

  def decrease(pid, amount), do: Agent.update(pid, &(&1 - amount))
end
