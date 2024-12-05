defmodule Day4 do
  def run() do
    IO.puts("Sample:\n")
    text = File.read!("in/day4-sample.txt")
    IO.puts("Part1:\n")
    part1(text)

    text = File.read!("in/day4.txt")
    IO.puts("\nResults\n")
    IO.puts("Part1:\n")
    part1(text)
  end

  def extract(text) do
    lines = text |> String.split("\n") |> Enum.filter(fn x -> x != "" end) |> Enum.to_list()

    width = String.length(Enum.at(lines, 0)) - 1
    height = Enum.count(lines) - 1

    rows =
      Enum.map(0..width, fn i ->
        Enum.reduce(lines, "", fn line, acc -> acc <> String.at(line, i) end)
      end)

    diagonal_tb =
      Enum.concat([
        Enum.map(0..height, fn y ->
          Enum.reduce(0..(width - y), "", fn yy, acc ->
            acc <> String.at(Enum.at(lines, y + yy), yy)
          end)
        end),
        Enum.map(1..width, fn x ->
          Enum.reduce(0..(height - x), "", fn xx, acc ->
            acc <> String.at(Enum.at(rows, x + xx), xx)
          end)
        end)
      ])

    diagonal_bt =
      Enum.concat([
        Enum.map(0..height, fn y ->
          Enum.reduce(0..(width - y), "", fn yy, acc ->
            acc <> String.at(Enum.at(lines, y + yy), width - yy)
          end)
        end),
        Enum.map(1..width, fn x ->
          Enum.reduce(0..(height - x), "", fn xx, acc ->
            acc <> String.at(Enum.at(rows, width - (x + xx)), xx)
          end)
        end)
      ])

    Enum.concat([
      lines,
      Enum.map(lines, fn x -> String.reverse(x) end),
      rows,
      Enum.map(rows, fn x -> String.reverse(x) end),
      diagonal_tb,
      Enum.map(diagonal_tb, fn x -> String.reverse(x) end),
      diagonal_bt,
      Enum.map(diagonal_bt, fn x -> String.reverse(x) end)
    ])
  end

  def part1(text) do
    data = extract(text)
    dbg(Enum.map(data, fn line -> find1(line) end) |> Enum.sum())
  end

  def find1(<<"XMAS", tail::binary>>), do: 1 + find1(tail)
  def find1(<<_::8, tail::binary>>), do: find1(tail)
  def find1(<<>>), do: 0
end
