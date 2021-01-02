using AdventOfCode2020.Solvers;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace AdventOfCode2020
{
    public static class Program
    {
        private static readonly IReadOnlyList<ISolver> Solvers = new ISolver[]
        {
            new Day01Solver(),
            new Day02Solver(),
            new Day03Solver(),
            new Day04Solver(),
            new Day05Solver(),
            new Day06Solver(),
        };

        static void Main()
        {
            ReportTime(SolveAll, "\nTotal time:");
        }

        private static void SolveAll()
        {
            foreach(var solver in Solvers)
            {
                ReportTime(solver.Solve);
            }
        }

        private static void ReportTime(Action action, string label = "Solved in")
        {
            var timeInMillis = TimeAction(action);
            Console.WriteLine($"{label} {timeInMillis/1000:F9}s\n");
        }

        private static void ReportAverageTime(Action action)
        {
            var times = new List<double>();
            for (var i = 0; i < 10; i++)
            {
                times.Add(TimeAction(action));
            }

            Console.WriteLine($@"
Hi: {times.Max():N3}ms
Lo: {times.Min():N3}ms
Av: {times.Average():N3}ms");
        }

        private static double TimeAction(Action action)
        {
            var stopwatch = new Stopwatch();
            stopwatch.Restart();
            action();
            stopwatch.Stop();
            return stopwatch.Elapsed.TotalMilliseconds;
        }
    }
}