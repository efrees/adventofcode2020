using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode2020.Solvers
{
    internal class Day16Solver : ISolver
    {
        private const string Name = "Day 16";
        private const string InputFile = "day16input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var groups = Input.GetInputFromFile(InputFile).Split("\n\n").ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(groups)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(groups)}");
        }

        private long GetPart1Answer(List<string> groups)
        {
            var rules = groups[0].SplitIntoLines().Select(ParseRule).ToList();
            var nearbyTickets = groups[2].SplitIntoLines().Skip(1).Select(ParseTicket).ToList();

            var allRanges = rules.SelectMany(rule => rule.Ranges).ToList();
            return nearbyTickets.SelectMany(ticket => ticket)
                    .Where(field => !allRanges.Any(range => field >= range.low && field <= range.high))
                    .Sum();
        }

        private long GetPart2Answer(List<string> groups)
        {
            return -1;
        }

        private FieldRule ParseRule(string line)
        {
            var name = line.Substring(0, line.IndexOf(':'));
            var rangeMatches = Regex.Matches(line, @"(\d+)-(\d+)");
            var ranges = rangeMatches.Select(match => (int.Parse(match.Groups[1].Value), int.Parse(match.Groups[2].Value)))
                .ToList();
            return new FieldRule
            {
                Name = name,
                Ranges = ranges
            };
        }

        private List<int> ParseTicket(string line)
        {
            return line.Split(",").Select(int.Parse).ToList();
        }

        private class FieldRule
        {
            public string Name { get; set; }
            public List<(int low, int high)> Ranges { get; set; }
        }
    }
}