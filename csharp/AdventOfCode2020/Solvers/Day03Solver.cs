using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day03Solver : ISolver
    {
        private const string Name = "Day 3";
        private const string InputFile = "day03input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var lines = Input.GetLinesFromFile(InputFile).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(lines)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(lines)}");
        }

        private int GetPart1Answer(List<string> lines)
        {
            var slope = (3, 1);
            return CountOnSlope(slope, lines);
        }

        private long GetPart2Answer(List<string> lines)
        {
            var slopes = new[]
            {
                (1, 1),
                (3, 1),
                (5, 1),
                (7, 1),
                (1, 2)
            };

            return slopes.Select(slope => CountOnSlope(slope, lines))
                .Aggregate(1L, (product, next) => product * next);
        }

        private int CountOnSlope((int dx, int dy) slope, List<string> lines)
        {
            var width = lines.First().Length;

            var x = 0;
            var y = 0;
            var count = 0;
            while (y < lines.Count)
            {
                if (lines[y][x % width] == '#')
                {
                    count++;
                }

                (x, y) = (x + slope.dx, y + slope.dy);
            }

            return count;
        }
    }
}