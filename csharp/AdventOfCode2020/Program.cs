using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace AdventOfCode2020
{
    public class Program
    {
        static void Main()
        {
            var fileText = Input.GetInputFromFile("day09input.txt");
            TimeAction(() => new Day9Solver().Solve(fileText));

            Console.ReadKey();
        }

        private static void TimeAction(Action action)
        {
            var stopwatch = new Stopwatch();
            var times = new List<double>();
            for (var i = 0; i < 10; i++)
            {
                stopwatch.Restart();
                action();
                stopwatch.Stop();
                times.Add(stopwatch.Elapsed.TotalMilliseconds);
            }
            Console.WriteLine($@"
Hi: {times.Max():N3}ms
Lo: {times.Min():N3}ms
Av: {times.Average():N3}ms");
        }
    }
}
