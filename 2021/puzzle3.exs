defmodule SubmarineDiagnostic do
  def interpret_data(filename) do
    filename
    |> stream_file_into_numbers()
    |> compute_data()
    |> seperate_gamma_and_epsilon()
  end

  defp stream_file_into_numbers(filename) do
    File.stream!(filename)
    |> Enum.map(&String.to_charlist(String.trim(&1)))
    |> Enum.map(fn x -> Enum.map(x, &(&1 - 48)) end)
  end

  defp compute_data(numbers) do
    numbers
    |> Enum.reduce(&Enum.map(Enum.with_index(&1), fn {x, index} -> x + Enum.at(&2, index) end))
    |> Enum.map(fn x ->
      if x > length(numbers) / 2 do
        {1, 0}
      else
        {0, 1}
      end
    end)
  end

  defp seperate_gamma_and_epsilon(data) do
    data
    |>Enum.reverse()
    |> Enum.reduce({[], []}, &({[elem(&1, 0) | elem(&2, 0)], [elem(&1, 1) | elem(&2, 1)]}))
    |> then(fn {gamma, epsilon} -> {Integer.undigits(gamma, 2), Integer.undigits(epsilon, 2)} end)
  end
end

{gamma, epsilon} = SubmarineDiagnostic.interpret_data("puzzle3_input.txt")

IO.puts "Part 1"
IO.puts "Gamma: #{gamma}"
IO.puts "Epsilon: #{epsilon}"
IO.puts "Submarine Power Output: #{gamma * epsilon}\n"
