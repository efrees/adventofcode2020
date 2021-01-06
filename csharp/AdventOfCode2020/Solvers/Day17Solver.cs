using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day17Solver : ISolver
    {
        private const string Name = "Day 17";
        private const string InputFile = "day17input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var startingGrid = Input.GetLinesFromFile(InputFile).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(startingGrid)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(startingGrid)}");
        }

        private long GetPart1Answer(List<string> sequence)
        {
            var grid = InitializeGrid(sequence);
            var nextGrid = new HashSet<(int, int, int)>();

            var cycleCount = 6;
            var neighborCounts = new Dictionary<(int, int, int), int>();
            foreach (var cycle in Enumerable.Range(0, cycleCount))
            {
                foreach (var neighbor in grid.SelectMany(GetNeighbors))
                {
                    neighborCounts[neighbor] = neighborCounts.GetValueOrDefault(neighbor, 0) + 1;
                }

                foreach (var (point, neighborCount) in neighborCounts)
                {
                    if (neighborCount == 3 || (neighborCount == 2 && grid.Contains(point)))
                    {
                        nextGrid.Add(point);
                    }
                }

                //foreach (var activePoint in grid)
                //{
                //    var neighborCount = neighborCounts.GetValueOrDefault(activePoint, -1);
                //    if (neighborCount == 2)
                //    {
                //        nextGrid.Add(activePoint);
                //    }
                //}
                (grid, nextGrid) = (nextGrid, grid);

                neighborCounts.Clear();
                nextGrid.Clear();
            }

            return grid.Count;
        }

        private long GetPart2Answer(List<string> sequence)
        {
            return -1;
        }

        private static HashSet<(int, int, int)> InitializeGrid(IReadOnlyList<string> sequence)
        {
            var grid = new HashSet<(int, int, int)>();
            for (var i = 0; i < sequence.Count; i++)
            {
                for (var j = 0; j < sequence[i].Length; j++)
                {
                    if (sequence[i][j] == '#')
                    {
                        grid.Add((i, j, 0));
                    }
                }
            }

            return grid;
        }

        private static IEnumerable<(int, int, int)> GetNeighbors((int x, int y, int z) point)
        {
            for (var i = -1; i <= 1; i++)
            {
                for (var j = -1; j <= 1; j++)
                {
                    for (var k = -1; k <= 1; k++)
                    {
                        if (i == 0 && j == 0 && k == 0) continue;
                        yield return (point.x + i, point.y + j, point.z + k);
                    }
                }
            }
        }
    }
}