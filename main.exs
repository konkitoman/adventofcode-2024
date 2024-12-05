defmodule Aoc2024 do
  case String.trim IO.gets "Day: " do
    "1" -> Day1.run()
    "2" -> Day2.run()
    "3" -> Day3.run()
    "4" -> Day4.run()
    _ -> 
      IO.puts("Invalid Day")
  end
end
