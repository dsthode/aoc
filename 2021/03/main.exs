defmodule Aoc202103 do
  use Bitwise

  def read_input(file) do
    File.stream!(file)
      |> Stream.map(fn x -> String.trim(x, "\n") end)
      |> Stream.map(fn x -> {String.to_integer(x, 2), x} end)
      |> Enum.to_list()
  end

  def calc_gamma_rate(values) do
    IO.puts("list length = #{length(values)}")
    times = String.length(elem(Enum.at(values, 0), 1))
    Enum.to_list(0..(times-1))
    |> Enum.map(fn pos -> calc_position(pos, values) end)
    |> sum_lists
  end

  def calc_position(pos, values) do
    Enum.map(values, fn {int_val, _} -> calc_position_from_value(pos, int_val) end)
  end

  def calc_position_from_value(pos, value) do
    ((1 <<< pos) &&& value) >>> pos
  end

  def sum_lists([list | tail]) do
    [sum_list(list) | sum_lists(tail)]
  end

  def sum_lists([]) do
    []
  end

  def sum_list(list) do
    Enum.reduce(list, fn x, acc -> acc + x end)
  end
end

IO.puts("Parte 1")
data = Aoc202103.read_input("input.txt")
data2 = Aoc202103.calc_gamma_rate(data)
IO.inspect(data2)

System.halt
