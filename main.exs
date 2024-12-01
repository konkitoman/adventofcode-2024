defmodule Aoc2024 do
  case String.trim IO.gets "Day: " do
    "1" -> Day1.run()
    _ -> 
      IO.puts("Invalid Day")
  end
end
