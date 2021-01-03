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

            var part1Answer = GetPart1Answer(sequence);
            Console.WriteLine($"Output (part 1): {part1Answer}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(sequence, part1Answer)}");
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

        private long GetPart2Answer(List<long> sequence, long target)
        {
            var buffer = new LinkedList<long>();
            var sum = 0L;
            foreach (var number in sequence)
            {
                buffer.AddLast(number);
                sum += number;

                while (sum > target && buffer.Any())
                {
                    sum -= buffer.First!.Value;
                    buffer.RemoveFirst();
                }

                if (sum == target)
                {
                    return buffer.Max() + buffer.Min();
                }
            }

            return -1;
        }
    }
}