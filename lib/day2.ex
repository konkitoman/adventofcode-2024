defmodule Day2 do
  def run() do
    IO.puts("Sample:\n")
    text = File.read!("in/day2-sample.txt")
    IO.puts("Part1:\n")
    part1(text)
    IO.puts("Part2:\n")
    part2(text)

    text = File.read!("in/day2.txt")
    IO.puts("\nResults\n")
    IO.puts("Part1:\n")
    part1(text)
    IO.puts("Part2:\n")
    part2(text)
  end

  def part1(text) do
    raports = get_raports(text)

    diffs =
      Enum.map(raports, fn levels ->
        Enum.chunk_every(levels, 2, 1, :discard)
        |> Enum.map(fn [left, right] -> left - right end)
      end)

    valids =
      Enum.map(diffs, fn diff ->
        Enum.chunk_every(diff, 2, 1, :discard)
        |> Enum.all?(fn [left, right] ->
          (0 < left == 0 < right or 0 > left == 0 > right) and abs(left) <= 3 and abs(right) <= 3 and
            left != 0 and right != 0
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

  def is_valid(original, tolerance, status \\ :undefined, levels \\ nil, i \\ 0) do
    levels =
      if levels == nil do
        original
      else
        levels
      end

    case levels do
      [left | tail] ->
        case tail do
          [right | _] ->
            diff = left - right

            {valid, status} = test(diff, status)

            if valid do
              is_valid(original, tolerance, status, tail, i + 1)
            else
              if tolerance === 0 do
                false
              else
                list = List.delete_at(original, i - 1)
                list2 = List.delete_at(original, i + 1)
                list3 = List.delete_at(original, i)

                is_valid(list, tolerance - 1) or is_valid(list2, tolerance - 1) or
                  is_valid(list3, tolerance - 1)
              end
            end

          [] ->
            true
        end

      [] ->
        true
    end
  end

  def test(diff, status) do
    s =
      cond do
        diff > 0 -> :inc
        diff < 0 -> :dec
        true -> :invalid
      end

    valid =
      case status do
        :inc -> s === :inc
        :dec -> s === :dec
        :undefined -> true
        :invalid -> false
      end and abs(diff) <= 3

    {valid, s}
  end

  def part2(text) do
    raports = get_raports(text)

    valids = Enum.map(raports, fn levels -> is_valid(levels, 1) end)

    dbg(valids)

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
