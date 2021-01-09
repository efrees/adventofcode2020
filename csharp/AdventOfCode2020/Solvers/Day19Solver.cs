using System;
using System.Collections.Generic;
using System.Dynamic;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day19Solver : ISolver
    {
        private const string Name = "Day 19";
        private const string InputFile = "day19input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var groups = Input.GetInputFromFile(InputFile).Split("\n\n");
            var rules = groups[0].SplitIntoLines().Select(ParseRule).ToList();
            var strings = groups[1].SplitIntoLines().ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(rules, strings)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(rules, strings)}");
        }

        private GrammarRule ParseRule(string line)
        {
            var parts = line.Split(": ");
            var ruleId = int.Parse(parts[0]);
            var productions = parts[1];
            if (productions.StartsWith("\""))
            {
                return new GrammarRule
                {
                    RuleId = ruleId,
                    Terminal = productions.Trim('"').First()
                };
            }

            return new GrammarRule
            {
                RuleId = ruleId,
                SubRuleSequences = productions
                    .Split(" | ")
                    .Select(seq => seq.Split(' ').Select(int.Parse).ToList())
                    .ToList()
            };
        }

        private long GetPart1Answer(List<GrammarRule> rules, List<string> strings)
        {
            var ruleLookup = rules.ToDictionary(r => r.RuleId, r => r);
            var rulesToMatch = new Stack<int>();
            return strings.Count(s =>
            {
                rulesToMatch.Clear();
                rulesToMatch.Push(0);
                return StringMatches(s.ToCharArray(), rulesToMatch, ruleLookup);
            });
        }

        private bool StringMatches(char[] word, Stack<int> rulesToMatch, Dictionary<int, GrammarRule> rules)
        {
            if (rulesToMatch.Count > word.Length)
            {
                return false;
            }

            if (rulesToMatch.Count == 0)
            {
                return word.Length == 0;
            }

            var nextRuleId = rulesToMatch.Pop();
            var nextRule = rules[nextRuleId];

            if (nextRule.Terminal != 0)
            {
                if (word[0] == nextRule.Terminal && StringMatches(word[1..], rulesToMatch, rules))
                {
                    return true;
                }
            }
            else
            {
                foreach (var ruleSequence in nextRule.SubRuleSequences)
                {
                    for (var i = ruleSequence.Count - 1; i >= 0; i--)
                    {
                        rulesToMatch.Push(ruleSequence[i]);
                    }

                    if (StringMatches(word, rulesToMatch, rules))
                    {
                        return true;
                    }

                    ruleSequence.ForEach(_ => rulesToMatch.Pop());
                }
            }

            // Put it back for the recursive level above
            rulesToMatch.Push(nextRuleId);
            return false;
        }

        private long GetPart2Answer(List<GrammarRule> rules, List<string> strings)
        {
            return -1;
        }

        private class GrammarRule
        {
            public int RuleId { get; set; }
            public IList<List<int>> SubRuleSequences { get; set; }
            public char Terminal { get; set; }
        }
    }
}