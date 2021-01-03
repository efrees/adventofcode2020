using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
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
            var memory = new Dictionary<long, long>();
            var currentMaskSet = new List<string>();

            foreach (var command in commands)
            {
                if (command.StartsWith("mask = "))
                {
                    var rawMask = command.Substring("mask = ".Length);
                    currentMaskSet = GenerateAllMasks(rawMask);
                }
                else
                {
                    var match = Regex.Match(command, @"mem\[(\d+)\] = (\d+)");
                    var address = int.Parse(match.Groups[1].Value);
                    var value = long.Parse(match.Groups[2].Value);

                    foreach (var mask in currentMaskSet)
                    {
                        memory[ApplyMask(address, mask)] = value;
                    }
                }
            }

            return memory.Values.Sum();
        }

        private List<string> GenerateAllMasks(string rawMask)
        {
            var floatingBitCount = rawMask.Count(bit => bit == 'X');
            var numberOfBitCombinations = 1 << floatingBitCount;
            var result = new List<string>(numberOfBitCombinations);
            var maskBuilder = new StringBuilder(rawMask.Length);
            foreach (var floatingBits in Enumerable.Range(0, numberOfBitCombinations))
            {
                var nextFloatingBitIndex = 0;
                foreach (var bit in rawMask)
                {
                    if (bit == 'X')
                    {
                        maskBuilder.Append((floatingBits >> nextFloatingBitIndex) & 1);
                        nextFloatingBitIndex++;
                    }
                    else
                    {
                        // Change '0' to 'X' to allow reuse of ApplyMask function
                        maskBuilder.Append(bit switch { '0' => 'X', var one => one });
                    }
                }

                result.Add(maskBuilder.ToString());
                maskBuilder.Clear();
            }

            return result;
        }
    }
}