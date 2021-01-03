using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day10Solver : ISolver
    {
        private const string Name = "Day 10";
        private const string InputFile = "day10input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var sequence = Input.GetLinesFromFile(InputFile).Select(int.Parse).OrderBy(x => x).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(sequence)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(sequence)}");
        }

        private long GetPart1Answer(List<int> sortedAdapters)
        {
            var oneDiffs = 0;
            var threeDiffs = 1; // The last one is always a three
            var previous = 0;
            foreach (var number in sortedAdapters)
            {
                switch (number - previous)
                {
                    case 1:
                        oneDiffs++;
                        break;
                    case 3:
                        threeDiffs++;
                        break;
                }

                previous = number;
            }

            return oneDiffs * (long)threeDiffs;
        }

        private long GetPart2Answer(List<int> sortedAdapters)
        {
            var partialAnswers = new long[sortedAdapters.Count];

            for (var i = 0; i < partialAnswers.Length; i++)
            {
                var currentJolts = sortedAdapters[i];

                if (currentJolts <= 3)
                {
                    partialAnswers[i] += 1;
                }

                var j = i + 1;
                while (j < sortedAdapters.Count && sortedAdapters[j] <= currentJolts + 3)
                {
                    partialAnswers[j] += partialAnswers[i];
                    j++;
                }
            }

            return partialAnswers[^1];
        }
    }
}