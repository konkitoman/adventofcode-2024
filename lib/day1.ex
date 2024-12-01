defmodule Day1 do
  def run() do
    IO.puts("Sample:\n")
    text = File.read!("in/day1-sample.txt") 
    part1(text)
    part2(text)

    IO.puts("\nSolution:\n")
    text = File.read!("in/day1.txt") 
    part1(text)
    part2(text)
  end

  def part1(text) do
    # extract lists
    [right, left] = get_lists(text)
    IO.inspect(right, label: "Right")
    IO.inspect(left, label: "Left")

    distances = Enum.zip([right, left]) |> Enum.map(fn {a, d} -> abs(a - d) end)
    IO.inspect(distances, label: "Distances")
    IO.puts("Distance: #{distances |> Enum.sum()}")
  end

  def part2(text) do
    [left, right] = get_lists(text)
    repeated = Enum.reduce(right, %{}, fn x, map -> Map.update(map, x, 1, fn v -> v + 1 end) end)
    IO.inspect(repeated, label: "repeated")
    similarity_score = Enum.map(left, fn x -> x * Map.get(repeated, x, 0) end) |> Enum.sum
    IO.puts("Similarity Score: #{similarity_score}")
  end

  def get_lists(input) do
    String.splitter(input, "\n")
    |> Enum.filter(fn str -> String.length(str) != 0 end)
    |> Enum.map(fn row -> String.split(row) end)
    |> Enum.map(fn list ->
      Enum.map(list, fn x ->
        {result, _} = Integer.parse(x, 10)
        result
      end)
    end)
    |> Enum.reduce([[], []], fn x, list ->
      [[Enum.at(x, 0) | Enum.at(list, 0)], [Enum.at(x, 1) | Enum.at(list, 1)]]
    end)

    # sort lists
    |> Enum.map(fn x -> Enum.sort(x) end)
  end
end
