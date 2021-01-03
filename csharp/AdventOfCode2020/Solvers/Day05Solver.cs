using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day05Solver : ISolver
    {
        private const string Name = "Day 5";
        private const string InputFile = "day05input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var lines = Input.GetLinesFromFile(InputFile).Select(ParseBinary).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(lines)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(lines)}");
        }

        private int ParseBinary(string line)
        {
            var result = 0;
            foreach (var c in line)
            {
                result *= 2;
                if (c == 'B' || c == 'R')
                {
                    result += 1;
                }
            }

            return result;
        }

        private int GetPart1Answer(List<int> lines)
        {
            return lines.Max();
        }

        private int GetPart2Answer(List<int> lines)
        {
            var set = lines.ToHashSet();
            return Enumerable.Range(lines.Min(), lines.Count).First(x => !set.Contains(x));
        }
    }
}