defmodule Day1 do
  def run() do
    IO.puts("Sample:\n")
    part1(File.read!("in/day1-sample.txt"))
    IO.puts("\nSolution:\n")
    text = File.read!("in/day1.txt") 
    part1(text)
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
