using System;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day01Solver : ISolver
    {
        private const string Name = "Day 1";
        private const string InputFile = "day01input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var lines = Input.GetLinesFromFile(InputFile);
            var sortedNumbers = lines.Select(long.Parse).OrderBy(x => x).ToArray();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(sortedNumbers)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(sortedNumbers)}");
        }

        private static long GetPart1Answer(long[] sortedNumbers)
        {
            var (i, j) = FindIndexOfPairWithSum(sortedNumbers, 2020);
            return sortedNumbers[i] * sortedNumbers[j];
        }

        private static (int i, int j) FindIndexOfPairWithSum(long[] sortedNumbers, long target)
        {
            var i = 0;
            var j = sortedNumbers.Length - 1;
            while (i < j)
            {
                var sum = sortedNumbers[i] + sortedNumbers[j];
                if (sum == target)
                {
                    break;
                }

                if (sum < target)
                {
                    i++;
                }

                j--;
            }

            return i < j
                ? (i, j)
                : (-1, -1);
        }

        private static long GetPart2Answer(long[] sortedNumbers)
        {
            for (var i = 0; i < sortedNumbers.Length - 2; i++)
            {
                var target = 2020 - sortedNumbers[i];

                var (j, k) = FindIndexOfPairWithSum(sortedNumbers[(i+1)..], target);
                if (j > -1)
                {
                    return sortedNumbers[i] * sortedNumbers[i + j + 1] * sortedNumbers[i + k + 1];
                }
            }

            throw new InvalidOperationException("Answer not found");
        }
    }
}