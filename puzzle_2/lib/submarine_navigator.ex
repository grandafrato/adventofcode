defmodule SubmarineNavigator do
  alias SubmarineNavigator.SubmarineDirection

  def start() do
    {:ok, x_plane} = SubmarineDirection.start_link()
    {:ok, y_plane} = SubmarineDirection.start_link()

    Application.app_dir(:submarine, "priv/puzzle_input.txt")
    |> File.stream!()
    |> Enum.map(fn x ->
      case String.split(x) do
        ["forward", amount] ->
          :ok = SubmarineDirection.increase(x_plane, String.to_integer(amount))

        ["down", amount] ->
          :ok = SubmarineDirection.increase(y_plane, String.to_integer(amount))

        ["up", amount] ->
          :ok = SubmarineDirection.decrease(y_plane, String.to_integer(amount))
      end
    end)

    answer = SubmarineDirection.value(x_plane) * SubmarineDirection.value(y_plane)

    IO.puts("The submarine depth times the horizontal position is: #{answer}")
  end
end
