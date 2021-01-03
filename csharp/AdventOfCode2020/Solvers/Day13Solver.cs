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
            Console.WriteLine($"Output (part 2): {GetPart2Answer(earliestTime, busSchedule)}");
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

        private long GetPart2Answer(long earliestTime, string[] busSchedule)
        {
            return -1;
        }
    }
}