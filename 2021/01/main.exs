defmodule Aoc202101 do
  def file_to_numbers(file) do
    File.stream!(file) |> Stream.map(&Integer.parse/1) |> Stream.map(fn {x, "\n"} -> x end) |> Enum.to_list()
  end

  def calculate_diffs(list) do
    case list do
      [head | tail] when length(tail) > 2 ->
        [Enum.at(tail, 0) - head | calculate_diffs(tail)]
      [head | tail] ->
        [Enum.at(tail, 0) - head]
    end
  end

  def filter_positives([head | tail]) when head > 0 do
    [head | filter_positives(tail)]
  end

  def filter_positives([head | tail]) when head <= 0 do
    filter_positives(tail)
  end

  def filter_positives(tail) when tail > 0 do
    [tail]
  end

  def filter_positives(tail) when tail <= 0 do
    []
  end

  def create_sliding_window(list) do
    case list do
      [head | tail] when length(tail) > 2 ->
        [head + Enum.at(tail, 0) + Enum.at(tail, 1) | create_sliding_window(tail)]
      [head | tail] when length(tail) == 2 ->
        [head + Enum.at(tail, 0) + Enum.at(tail, 1)]
    end
  end
end

IO.puts("Parte 1")
nums = Aoc202101.file_to_numbers("input.txt")
diffs = Aoc202101.calculate_diffs(nums)
res = Aoc202101.filter_positives(diffs)
IO.puts(length(res))

IO.puts("Parte 2")
window = Aoc202101.create_sliding_window(nums)
diffs = Aoc202101.calculate_diffs(window)
res = Aoc202101.filter_positives(diffs)
IO.puts(length(res))
