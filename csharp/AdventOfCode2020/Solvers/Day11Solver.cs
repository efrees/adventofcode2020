using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day11Solver : ISolver
    {
        private const string Name = "Day 11";
        private const string InputFile = "day11input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var rows = Input.GetLinesFromFile(InputFile).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(rows)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(rows)}");
        }

        private long GetPart1Answer(List<string> rows)
        {
            var currentLayout = rows.Select(row => row.ToCharArray()).ToArray();
            var nextLayout = rows.Select(row => row.ToCharArray()).ToArray();

            while (SimulateRoundPart1(currentLayout, nextLayout))
            {
                (currentLayout, nextLayout) = (nextLayout, currentLayout);
            }

            return currentLayout.SelectMany(row => row).Count(seat => seat == '#');
        }

        private bool SimulateRoundPart1(char[][] currentLayout, char[][] nextLayout)
        {
            var seatChanged = false;
            for (var row = 0; row < currentLayout.Length; row++)
            {
                for (var col = 0; col < currentLayout[row].Length; col++)
                {
                    var currentSeat = currentLayout[row][col];
                    if (currentSeat == '.')
                    {
                        continue;
                    }

                    var occupiedNeighbors = GetDirections()
                        .Select(dir => AddVectors(dir, (col, row)))
                        .Where(position => IsInBounds(position, currentLayout[row].Length, currentLayout.Length))
                        .Count(position => currentLayout[position.y][position.x] == '#');

                    var newState = currentSeat switch
                    {
                        'L' when occupiedNeighbors == 0 => '#',
                        '#' when occupiedNeighbors >= 4 => 'L',
                        _ => currentSeat
                    };

                    if (newState != currentSeat)
                    {
                        seatChanged = true;
                    }

                    nextLayout[row][col] = newState;
                }
            }

            return seatChanged;
        }

        private long GetPart2Answer(List<string> rows)
        {
            var currentLayout = rows.Select(row => row.ToCharArray()).ToArray();
            var nextLayout = rows.Select(row => row.ToCharArray()).ToArray();

            while (SimulateRoundPart2(currentLayout, nextLayout))
            {
                (currentLayout, nextLayout) = (nextLayout, currentLayout);
            }

            return currentLayout.SelectMany(row => row).Count(seat => seat == '#');
        }

        private bool SimulateRoundPart2(char[][] currentLayout, char[][] nextLayout)
        {
            var seatChanged = false;
            for (var row = 0; row < currentLayout.Length; row++)
            {
                for (var col = 0; col < currentLayout[row].Length; col++)
                {
                    var currentSeat = currentLayout[row][col];
                    if (currentSeat == '.')
                    {
                        continue;
                    }

                    var occupiedNeighbors = GetDirections()
                        .Count(dir =>
                        {
                            var nextInDirection = AddVectors(dir, (col, row));
                            while (IsInBounds(nextInDirection, currentLayout[row].Length, currentLayout.Length))
                            {
                                var seatState = currentLayout[nextInDirection.y][nextInDirection.x];
                                if (seatState != '.')
                                {
                                    return seatState == '#';
                                }

                                nextInDirection = AddVectors(dir, nextInDirection);
                            }

                            return false;
                        });

                    var newState = currentSeat switch
                    {
                        'L' when occupiedNeighbors == 0 => '#',
                        '#' when occupiedNeighbors >= 5 => 'L',
                        _ => currentSeat
                    };

                    if (newState != currentSeat)
                    {
                        seatChanged = true;
                    }

                    nextLayout[row][col] = newState;
                }
            }

            return seatChanged;
        }

        private IEnumerable<(int x, int y)> GetDirections()
        {
            yield return (1, 1);
            yield return (1, 0);
            yield return (1, -1);
            yield return (0, -1);
            yield return (-1, -1);
            yield return (-1, 0);
            yield return (-1, 1);
            yield return (0, 1);
        }

        private (int x, int y) AddVectors((int x, int y) point1, (int x, int y) point2)
        {
            return (point1.x + point2.x, point1.y + point2.y);
        }

        private static bool IsInBounds((int x, int y) position, int width, int height)
        {
            return position.x >= 0 && position.x < width && position.y >= 0 && position.y < height;
        }
    }
}