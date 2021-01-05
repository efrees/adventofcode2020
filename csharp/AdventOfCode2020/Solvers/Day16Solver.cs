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
                .Where(field => !SomeRangeContains(allRanges, field))
                .Sum();
        }

        private long GetPart2Answer(List<string> groups)
        {
            var rules = groups[0].SplitIntoLines().Select(ParseRule).ToList();
            var myTicket = ParseTicket(groups[1].SplitIntoLines().Last());
            var nearbyTickets = groups[2].SplitIntoLines().Skip(1).Select(ParseTicket).ToList();
            var allRanges = rules.SelectMany(rule => rule.Ranges).ToList();

            var nearbyValidTickets = nearbyTickets
                .Where(fields => fields.All(field => SomeRangeContains(allRanges, field)));

            var validTicketFieldValues = nearbyValidTickets
                .SelectMany(values => values.Select((value, index) => (value, index)))
                .ToLookup(x => x.index, x => x.value);
            var possibleMatchesForField = Enumerable.Range(0, myTicket.Count)
                .ToDictionary(fieldIndex => fieldIndex,
                    fieldIndex => rules
                        .Where(rule => validTicketFieldValues[fieldIndex].All(value => SomeRangeContains(rule.Ranges, value)))
                        .Select(r => r.Name).ToHashSet());
            var fieldMatches = new Dictionary<int, string>();

            while (possibleMatchesForField.Any())
            {
                var (fieldPosition, fieldNames) = possibleMatchesForField.First(pair => pair.Value.Count == 1);
                var fieldName = fieldNames.First();
                fieldMatches[fieldPosition] = fieldName;

                possibleMatchesForField.Remove(fieldPosition);
                foreach (var possibilities in possibleMatchesForField.Values)
                {
                    possibilities.Remove(fieldName);
                }
            }

            var fieldPositions = fieldMatches.Where(pair => pair.Value.StartsWith("departure")).Select(pair => pair.Key);

            return fieldPositions.Select(index => myTicket[index]).Aggregate(1L, (product, next) => product * (long)next);
        }

        private static bool SomeRangeContains(List<(int low, int high)> ranges, int field)
        {
            return ranges.Any(range => field >= range.low && field <= range.high);
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