using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day06Solver : ISolver
    {
        private const string Name = "Day 6";
        private const string InputFile = "day06input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var groups = CollectGroups(Input.GetLinesFromFile(InputFile)).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(groups)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(groups)}");
        }

        private IEnumerable<List<string>> CollectGroups(IEnumerable<string> lines)
        {
            var buffer = new List<string>();
            foreach (var line in lines)
            {
                if (string.IsNullOrEmpty(line))
                {
                    yield return buffer;
                    buffer = new List<string>();
                }
                else
                {
                    buffer.Add(line);
                }
            }

            yield return buffer;
        }

        private int GetPart1Answer(List<List<string>> groups)
        {
            return groups
                .Select(group => string.Join("", group).ToHashSet().Count)
                .Sum();
        }

        private int GetPart2Answer(List<List<string>> groups)
        {
            return groups
                .Select(group => IntersectionOfAll(group).Count)
                .Sum();
        }

        private ISet<char> IntersectionOfAll(List<string> group)
        {
            var sets = group.Select(g => g.ToHashSet());
            return sets.Aggregate((intersection, next) =>
            {
                intersection.IntersectWith(next);
                return intersection;
            });
        }
    }
}