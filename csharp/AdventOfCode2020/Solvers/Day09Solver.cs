using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day09Solver : ISolver
    {
        private const string Name = "Day 9";
        private const string InputFile = "day09input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var sequence = Input.GetLinesFromFile(InputFile).Select(long.Parse).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(sequence)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer()}");
        }

        private long GetPart1Answer(List<long> sequence)
        {
            var buffer = new LinkedList<long>();
            var set = new HashSet<long>();
            foreach (var number in sequence)
            {
                if (buffer.Count < 25)
                {
                    set.Add(number);
                    buffer.AddLast(number);
                    continue;
                }

                if (!set.Any(previous => set.Contains(number - previous)))
                {
                    return number;
                }

                set.Remove(buffer.First!.Value);
                buffer.RemoveFirst();
                set.Add(number);
                buffer.AddLast(number);
            }

            return -1;
        }

        private int GetPart2Answer()
        {
            return -1;
        }
    }
}