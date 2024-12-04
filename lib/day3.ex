defmodule Day3 do
  def run() do
    IO.puts("Sample:\n")
    text = File.read!("in/day3-sample.txt")
    IO.puts("Part1:\n")
    part1(text)
    IO.puts("Part2:\n")
    part2(text)

    text = File.read!("in/day3.txt")
    IO.puts("\nResults\n")
    IO.puts("Part1:\n")
    part1(text)
    IO.puts("Part2:\n")
    part2(text)
  end

  defguardp is_digit(c) when c in ?0..?9

  def part1(text) do
    dbg(text)
    IO.puts("Result: #{step1(text)}")
  end

  defp step1(text), do: step1(:none, text)

  defp step1(:none, <<"mul(", tail::binary>>), do: step1(:mul, tail, 0)
  defp step1(:none, <<_::8, tail::binary>>), do: step1(:none, tail)
  defp step1(:none, <<>>), do: 0

  defp step1(:mul, <<num::8, tail::binary>>, last) when is_digit(num) do
    step1(:mul, tail, last * 10 + (num - ?0))
  end

  defp step1(:mul, <<",", tail::binary>>, last), do: step1(:mul, tail, last, 0)
  defp step1(:mul, <<_::8, tail::binary>>, _), do: step1(:none, tail)

  defp step1(:mul, <<num::8, tail::binary>>, left, last) when is_digit(num) do
    step1(:mul, tail, left, last * 10 + (num - ?0))
  end

  defp step1(:mul, <<")", tail::binary>>, left, right) do
    left * right + step1(:none, tail)
  end

  defp step1(:mul, <<_::8, tail::binary>>, _, _), do: step1(:none, tail)
  defp step1(:mul, <<>>, _, _), do: 0

  def part2(text) do
    IO.puts("Results: #{step2(text)}")
  end

  defp step2(text), do: step2(:do, text)

  defp step2(:do, <<"mul(", tail::binary>>), do: step2(:mul, tail, 0)
  defp step2(:do, <<"don't()", tail::binary>>), do: step2(:dont, tail)
  defp step2(:do, <<_::8, tail::binary>>), do: step2(:do, tail)
  defp step2(:do, <<>>), do: 0

  defp step2(:mul, <<num::8, tail::binary>>, last) when is_digit(num) do
    step2(:mul, tail, last * 10 + (num - ?0))
  end

  defp step2(:mul, <<",", tail::binary>>, left), do: step2(:mul, tail, left, 0)
  defp step2(:mul, <<_::8, tail::binary>>, _), do: step2(:do, tail)
  defp step2(:mul, <<>>, _), do: 0

  defp step2(:mul, <<num::8, tail::binary>>, left, last) when is_digit(num) do
    step2(:mul, tail, left, last * 10 + (num - ?0))
  end

  defp step2(:mul, <<")", tail::binary>>, left, right), do: left * right + step2(:do, tail)
  defp step2(:mul, <<_::8, tail::binary>>, _, _), do: step2(:do, tail)
  defp step2(:mul, <<>>, _, _), do: 0

  defp step2(:dont, <<"do()", tail::binary>>), do: step2(:do, tail)
  defp step2(:dont, <<_::8, tail::binary>>), do: step2(:dont, tail)
  defp step2(:dont, <<>>), do: 0
end
