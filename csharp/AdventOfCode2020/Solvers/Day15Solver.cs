using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day15Solver : ISolver
    {
        private const string Name = "Day 15";
        private const string InputFile = "1,20,11,6,12,0";

        public void Solve()
        {
            Console.WriteLine(Name);
            var sequence = InputFile.Split(",").Select(int.Parse).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(sequence)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(sequence)}");
        }

        private long GetPart1Answer(List<int> sequence)
        {
            return PlayTurns(sequence, 2020);
        }

        private long GetPart2Answer(List<int> sequence)
        {
            return PlayTurns(sequence, 30_000_000);
        }

        private static long PlayTurns(List<int> sequence, int numberOfTurns)
        {
            var turnCount = sequence.Count - 1;
            var previousNumbers = sequence.Select((number, index) => (number, turn: index))
                .Take(turnCount)
                .ToDictionary(tuple => tuple.number, tuple => tuple.turn);

            var lastNumber = sequence[^1];
            while (turnCount < numberOfTurns - 1)
            {
                var next = previousNumbers.ContainsKey(lastNumber)
                    ? turnCount - previousNumbers[lastNumber]
                    : 0;

                previousNumbers[lastNumber] = turnCount;
                lastNumber = next;
                turnCount++;
            }

            return lastNumber;
        }
    }
}