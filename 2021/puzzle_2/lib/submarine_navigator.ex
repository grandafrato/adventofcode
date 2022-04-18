defmodule SubmarineNavigator do
  alias SubmarineNavigator.SubmarineDirection

  def start() do
    {:ok, horizontal} = SubmarineDirection.start_link()
    {:ok, depth} = SubmarineDirection.start_link()

    Application.app_dir(:submarine, "priv/puzzle_input.txt")
    |> File.stream!()
    |> Enum.map(fn x ->
      case String.split(x) do
        ["forward", amount] ->
          :ok = SubmarineDirection.increase(horizontal, String.to_integer(amount))

        ["down", amount] ->
          :ok = SubmarineDirection.increase(depth, String.to_integer(amount))

        ["up", amount] ->
          :ok = SubmarineDirection.decrease(depth, String.to_integer(amount))
      end
    end)

    answer = SubmarineDirection.value(horizontal) * SubmarineDirection.value(depth)

    IO.puts("The submarine depth times the horizontal position is: #{answer}")
  end

  def start2() do
    {:ok, horizontal} = SubmarineDirection.start_link()
    {:ok, depth} = SubmarineDirection.start_link()
    {:ok, aim} = SubmarineDirection.start_link()

    Application.app_dir(:submarine, "priv/puzzle_input.txt")
    |> File.stream!()
    |> Enum.map(fn x ->
      case String.split(x) do
        ["forward", amount] ->
          :ok = forward_part2(horizontal, depth, aim, String.to_integer(amount))

        ["down", amount] ->
          :ok = SubmarineDirection.increase(aim, String.to_integer(amount))

        ["up", amount] ->
          :ok = SubmarineDirection.decrease(aim, String.to_integer(amount))
      end
    end)

    answer = SubmarineDirection.value(horizontal) * SubmarineDirection.value(depth)

    IO.puts("The submarine depth times the horizontal position is: #{answer}")
  end

  defp forward_part2(horizontal, depth, aim, amount) do
    :ok = SubmarineDirection.increase(horizontal, amount)
    SubmarineDirection.increase(depth, SubmarineDirection.value(aim) * amount)
  end
end
