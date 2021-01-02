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
            Console.WriteLine($"Output (part 2): {GetPart2Answer()}");
        }

        private BagNode ParseBag(string line)
        {
            var halves = line.Split(" bags contain ");
            var name = halves[0];
            var contents = Regex.Matches(halves[1], @"(\d+) (.+?) bag");
            return new BagNode
            {
                Color = name,
                Contents = contents.Select(match => match.Groups[2].Value).ToList()
            };
        }

        private int GetPart1Answer(List<BagNode> bagNodes)
        {
            var containingColors = bagNodes.SelectMany(node => node.Contents.Select(child => (child, parent: node.Color)))
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

        private int GetPart2Answer()
        {
            return -1;
        }

        private class BagNode
        {
            public string Color { get; set; }
            public List<string> Contents { get; set; }
        }
    }
}