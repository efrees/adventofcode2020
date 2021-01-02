using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode2020.Solvers
{
    internal class Day02Solver : ISolver
    {
        private const string Name = "Day 2";
        private const string InputFile = "day02input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var rules = Input.GetLinesFromFile(InputFile).Select(ParseRules).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(rules)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(rules)}");
        }

        private (PasswordRule rule, string password) ParseRules(string line)
        {
            var match = Regex.Match(line, @"(\d+)-(\d+) (\w): (\w+)");
            var rule = new PasswordRule
            {
                Low = int.Parse(match.Groups[1].Value),
                High = int.Parse(match.Groups[2].Value),
                Required = match.Groups[3].Value.First()
            };

            return (rule, match.Groups[4].Value);
        }

        private int GetPart1Answer(List<(PasswordRule rule, string password)> rules)
        {
            return rules.Count(pair =>
            {
                var (rule, password) = pair;

                var matchCount = password.Count(c => c == rule.Required);
                return matchCount >= rule.Low && matchCount <= rule.High;
            });
        }

        private int GetPart2Answer(List<(PasswordRule rule, string password)> rules)
        {
            return rules.Count(pair =>
            {
                var (rule, password) = pair;

                return (password[rule.Low - 1] == rule.Required)
                       ^ (password[rule.High - 1] == rule.Required);
            });
        }

        private class PasswordRule
        {
            public int Low { get; set; } 
            public int High { get; set; } 
            public char Required { get; set; }
        }
    }
}
