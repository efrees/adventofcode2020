using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode2020.Solvers
{
    internal class Day07Solver : ISolver
    {
        private const string Name = "Day 7";
        private const string InputFile = "day07input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var nodes = Input.GetLinesFromFile(InputFile).Select(ParseBag).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(nodes)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(nodes)}");
        }

        private BagNode ParseBag(string line)
        {
            var halves = line.Split(" bags contain ");
            var name = halves[0];
            var contents = Regex.Matches(halves[1], @"(\d+) (.+?) bag");
            return new BagNode
            {
                Color = name,
                Contents = contents.Select(match => new CountedBags
                {
                    Count = int.Parse(match.Groups[1].Value),
                    Color = match.Groups[2].Value
                }).ToList()
            };
        }

        private int GetPart1Answer(List<BagNode> bagNodes)
        {
            var containingColors = bagNodes
                .SelectMany(node =>
                    node.Contents.Select(child => (child: child.Color, parent: node.Color))
                )
                .ToLookup(x => x.child, x => x.parent);
            var visited = new HashSet<string>();

            int DepthFirstCount(string color)
            {
                if (visited.Contains(color))
                {
                    return 0;
                }

                visited.Add(color);
                return 1 + containingColors[color].Select(DepthFirstCount).Sum();
            }

            return DepthFirstCount("shiny gold") - 1;
        }

        private int GetPart2Answer(List<BagNode> bagNodes)
        {
            var contents = bagNodes
                .ToDictionary(x => x.Color, x => x.Contents);
            var memos = new Dictionary<string, int>();

            int DepthFirstCount(string color)
            {
                if (memos.ContainsKey(color))
                {
                    return memos[color];
                }

                var totalCount = 1 + contents[color].Select(c => c.Count * DepthFirstCount(c.Color)).Sum();
                memos.Add(color, totalCount);
                return totalCount;
            }

            return DepthFirstCount("shiny gold") - 1;
        }

        private class BagNode
        {
            public string Color { get; set; }
            public List<CountedBags> Contents { get; set; }
        }

        private class CountedBags
        {
            public int Count { get; set; }
            public string Color { get; set; }
        }
    }
}