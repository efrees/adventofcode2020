using System;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day13Solver : ISolver
    {
        private const string Name = "Day 13";
        private const string InputFile = "day13input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var input = Input.GetLinesFromFile(InputFile).ToList();
            var earliestTime = long.Parse(input[0]);
            var busSchedule = input[1].Split(",");

            Console.WriteLine($"Output (part 1): {GetPart1Answer(earliestTime, busSchedule)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(busSchedule)}");
        }

        private long GetPart1Answer(long earliestTime, string[] busSchedule)
        {
            var waitTimes = busSchedule
                .Where(bus => bus != "x")
                .Select(long.Parse)
                .Select(period => (busId: period, waitTime: period - earliestTime % period));
            var (busId, waitTime) = waitTimes.OrderBy(result => result.waitTime).First();
            return busId * waitTime;
        }

        private long GetPart2Answer(string[] busSchedule)
        {
            var desiredOffsets = busSchedule
                .Select((busId, offset) => (busId, offset))
                .Where(busAndOffset => busAndOffset.busId != null && busAndOffset.busId != "x")
                .Select(busAndOffset => (busId: long.Parse(busAndOffset.busId), busAndOffset.offset));

            var timestamp = 0L;
            var timeUntilSolutionRepeats = 1L;

            foreach (var (busId, targetOffset) in desiredOffsets)
            {
                // The bus could run multiple times before the desired offset.
                // We want to search for the one closest to the timestamp.
                var firstOffset = targetOffset % busId;

                // Note special case for a valid solution having timestamp evenly divide by the bus id/period
                while (busId - firstOffset != timestamp % busId
                       && (0 != firstOffset || 0 != timestamp % busId))
                {
                    timestamp += timeUntilSolutionRepeats;
                }

                timeUntilSolutionRepeats *= busId;
            }

            return timestamp;
        }
    }
}