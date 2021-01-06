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

        private long GetPart1Answer(List<string> startingGrid)
        {
            var grid = InitializeGrid3d(startingGrid);
            var nextGrid = new HashSet<(int, int, int)>();

            var cycleCount = 6;
            var neighborCounts = new Dictionary<(int, int, int), int>();
            foreach (var cycle in Enumerable.Range(0, cycleCount))
            {
                foreach (var neighbor in grid.SelectMany(GetNeighbors3d))
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

                (grid, nextGrid) = (nextGrid, grid);

                neighborCounts.Clear();
                nextGrid.Clear();
            }

            return grid.Count;
        }

        private long GetPart2Answer(List<string> startingGrid)
        {
            var grid = InitializeGrid4d(startingGrid);
            var nextGrid = new HashSet<(int, int, int, int)>();

            var cycleCount = 6;
            var neighborCounts = new Dictionary<(int, int, int, int), int>();
            foreach (var cycle in Enumerable.Range(0, cycleCount))
            {
                foreach (var neighbor in grid.SelectMany(GetNeighbors4d))
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

                (grid, nextGrid) = (nextGrid, grid);

                neighborCounts.Clear();
                nextGrid.Clear();
            }

            return grid.Count;
        }

        private static HashSet<(int, int, int)> InitializeGrid3d(IReadOnlyList<string> startingGrid)
        {
            var grid = new HashSet<(int, int, int)>();
            for (var i = 0; i < startingGrid.Count; i++)
            {
                for (var j = 0; j < startingGrid[i].Length; j++)
                {
                    if (startingGrid[i][j] == '#')
                    {
                        grid.Add((i, j, 0));
                    }
                }
            }

            return grid;
        }

        private static HashSet<(int, int, int, int)> InitializeGrid4d(IReadOnlyList<string> startingGrid)
        {
            var grid = new HashSet<(int, int, int, int)>();
            for (var i = 0; i < startingGrid.Count; i++)
            {
                for (var j = 0; j < startingGrid[i].Length; j++)
                {
                    if (startingGrid[i][j] == '#')
                    {
                        grid.Add((i, j, 0, 0));
                    }
                }
            }

            return grid;
        }

        private static IEnumerable<(int, int, int)> GetNeighbors3d((int x, int y, int z) point)
        {
            for (var i = -1; i <= 1; i++)
            {
                for (var j = -1; j <= 1; j++)
                {
                    for (var k = -1; k <= 1; k++)
                    {
                        if (i == 0 && j == 0 && k == 0)
                        {
                            continue;
                        }

                        yield return (point.x + i, point.y + j, point.z + k);
                    }
                }
            }
        }

        private static IEnumerable<(int, int, int, int)> GetNeighbors4d((int x, int y, int z, int w) point)
        {
            for (var i = -1; i <= 1; i++)
            {
                for (var j = -1; j <= 1; j++)
                {
                    for (var k = -1; k <= 1; k++)
                    {
                        for (var m = -1; m <= 1; m++)
                        {
                            if (i == 0 && j == 0 && k == 0 && m == 0)
                            {
                                continue;
                            }

                            yield return (point.x + i, point.y + j, point.z + k, point.w + m);
                        }
                    }
                }
            }
        }
    }
}