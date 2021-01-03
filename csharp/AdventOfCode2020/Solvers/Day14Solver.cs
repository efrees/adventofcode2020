using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode2020.Solvers
{
    internal class Day14Solver : ISolver
    {
        private const string Name = "Day 14";
        private const string InputFile = "day14input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var commands = Input.GetLinesFromFile(InputFile).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(commands)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(commands)}");
        }

        private long GetPart1Answer(List<string> commands)
        {
            var memory = new Dictionary<int, long>();
            var currentMask = "";

            foreach (var command in commands)
            {
                if (command.StartsWith("mask = "))
                {
                    currentMask = command.Substring("mask = ".Length);
                }
                else
                {
                    var match = Regex.Match(command, @"mem\[(\d+)\] = (\d+)");
                    var address = int.Parse(match.Groups[1].Value);
                    var value = long.Parse(match.Groups[2].Value);

                    memory[address] = ApplyMask(value, currentMask);
                }
            }

            return memory.Values.Sum();
        }

        private long ApplyMask(long value, string currentMask)
        {
            for (var i = 0; i < currentMask.Length; i++)
            {
                var bit = currentMask[^(i + 1)];
                if (bit == '1')
                {
                    value |= 1L << i;
                }
                else if (bit == '0')
                {
                    value &= ~(1L << i);
                }
            }

            return value;
        }

        private long GetPart2Answer(List<string> commands)
        {
            return -1;
        }
    }
}