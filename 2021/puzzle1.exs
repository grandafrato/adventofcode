defmodule SonarSweep do
  def run_increment(file_name), do: find_i(File.stream!(file_name), 0, 0)

  def run_sliding_window(file_name) do
    file_name
    |> extract_sliding_numbers()
    |> combine_sliding_numbers()
    |> find_i(0, 0, &Enum.at/2)
  end

  defp extract_sliding_numbers(file_name) do
    file_name
    |> File.stream!()
    |> Enum.map(&String.to_integer(String.trim(&1)))
    |> then(fn a ->
      [_ | b] = a
      [_ | c] = b
      [_ | d] = c

      {
        handle_sliding_numbers(a),
        handle_sliding_numbers(b),
        handle_sliding_numbers(c),
        handle_sliding_numbers(d)
      }
    end)
  end

  defp handle_sliding_numbers(number_list) do
    number_list
    |> Enum.chunk_every(3, 4)
    |> Enum.map(fn x ->
      Enum.reduce(x, &Kernel.+/2)
    end)
  end

  defp combine_sliding_numbers({a, b, c, d}) do
    a
    |> Enum.with_index()
    |> Enum.map(fn {x, index} ->
      [[x | [Enum.at(b, index)]] | [Enum.at(c, index) | [Enum.at(d, index)]]]
    end)
    |> List.flatten()
  end

  defp find_i(file_stream, index, increment_count, handler \\ &handle_data/2) do
    case {handler.(file_stream, index), handler.(file_stream, index + 1)} do
      {_a, nil} -> increment_count
      {a, b} -> find_i(file_stream, index + 1, increase_count(increment_count, a, b), handler)
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
      # IO.puts("#{b}: Increment")
      count + 1
    else
      # IO.puts("#{b}: Decrement")
      count
    end
  end
end

increments_count = SonarSweep.run_increment("puzzle1_input.txt")
increments_sliding_window = SonarSweep.run_sliding_window("puzzle1_input.txt")

IO.puts("Increments Count: #{increments_count}")
IO.puts("Increments Sliding Window: #{increments_sliding_window}")
