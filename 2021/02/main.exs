defmodule Aoc202102 do
  def read_input(file) do
    File.stream!(file) |> Enum.to_list()
  end

  def lines_to_commands([line | tail]) do
    [parse_line(line) | lines_to_commands(tail)]
  end

  def lines_to_commands([]) do
    []
  end

  def parse_line(line) do
    case line do
      "forward "<>rest ->
        {amount, _} = Integer.parse(rest)
        {:forward, amount}
      "down "<>rest ->
        {amount, _} = Integer.parse(rest)
        {:down, amount}
      "up "<>rest ->
        {amount, _} = Integer.parse(rest)
        {:up, amount}
    end
  end

  def process_commands([{command, amount} | tail], {h_pos, d_pos, _}, :part1) do
    case command do
      :forward ->
        process_commands(tail, {h_pos+amount, d_pos, 0}, :part1)
      :down ->
        process_commands(tail, {h_pos, d_pos+amount, 0}, :part1)
      :up ->
        process_commands(tail, {h_pos, d_pos-amount, 0}, :part1)
      end
  end

  def process_commands([{command, amount} | tail], {h_pos, d_pos, aim}, :part2) do
    case command do
      :forward ->
        process_commands(tail, {h_pos+amount, d_pos+aim*amount, aim}, :part2)
      :down ->
        process_commands(tail, {h_pos, d_pos, aim+amount}, :part2)
      :up ->
        process_commands(tail, {h_pos, d_pos, aim-amount}, :part2)
      end
  end

  def process_commands([], {h_pos, d_pos, aim}, _) do
    {h_pos, d_pos, aim}
  end
end

IO.puts("Parte 1")
lines = Aoc202102.read_input("input.txt")
commands = Aoc202102.lines_to_commands(lines)
{final_h, final_depth, _} = Aoc202102.process_commands(commands, {0, 0, 0}, :part1)
IO.puts("#{final_h},#{final_depth}")
IO.puts("#{final_h*final_depth}")

IO.puts("Parte 2")
{final_h, final_depth, final_aim} = Aoc202102.process_commands(commands, {0, 0, 0}, :part2)
IO.puts("#{final_h},#{final_depth},#{final_aim}")
IO.puts("#{final_h*final_depth}")

System.halt
