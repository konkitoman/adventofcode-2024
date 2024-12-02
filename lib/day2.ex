defmodule Day2 do
  def run() do
    IO.puts("Sample:\n")
    text = File.read!("in/day2-sample.txt")
    part1(text)

    IO.puts("\nPart1:\n")
    text = File.read!("in/day2.txt")
    part1(text)
  end

  def part1(text) do
    raports = get_raports(text)
    dbg(raports)

    diffs =
      Enum.map(raports, fn levels ->
        Enum.chunk_every(levels, 2, 1, :discard)
        |> Enum.map(fn [left, right] -> left - right end)
      end)

    dbg(diffs)

    valids =
      Enum.map(diffs, fn diff ->
        Enum.chunk_every(diff, 2, 1, :discard)
        |> Enum.all?(fn [left, right] ->
          (0 < left == 0 < right or 0 > left == 0 > right) and abs(left) <= 3 and abs(right) <= 3 and left != 0 and right != 0
        end)
      end)

    sum =
      Enum.reduce(valids, 0, fn x, acc ->
        if x do
          acc + 1
        else
          acc
        end
      end)

    IO.puts("Safe count: #{sum}\n")
  end

  def get_raports(text) do
    raports =
      String.splitter(text, "\n")
      |> Enum.filter(fn x -> String.length(x) != 0 end)
      |> Enum.map(fn row ->
        String.split(row)
        |> Enum.map(fn x ->
          {level, _} = Integer.parse(x)
          level
        end)
      end)

    raports
  end
end
