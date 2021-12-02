defmodule SonarSweep do
  def run(file_name), do: find_i(File.stream!(file_name), 0, 0)

  defp find_i(file_stream, index, increment_count) do
    case {handle_data(file_stream, index), handle_data(file_stream, index + 1)} do
      {_a, nil} -> increment_count
      {a, b} -> find_i(file_stream, index + 1, increase_count(increment_count, a, b))
    end
  end

  defp handle_data(file_stream, index) do
    file_stream
    |> Enum.at(index)
    |> then(fn x ->
      if x do
        x
        |> String.trim()
        |> String.to_integer()
      else
        x
      end
    end)
  end

  defp increase_count(count, a, b) do
    if b > a do
      count + 1
    else
      count
    end
  end
end

IO.puts("Increments Count: #{SonarSweep.run("puzzle1_input.txt")}")
