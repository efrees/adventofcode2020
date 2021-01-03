using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day12Solver : ISolver
    {
        private const string Name = "Day 12";
        private const string InputFile = "day12input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var sequence = Input.GetLinesFromFile(InputFile).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(sequence)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(sequence)}");
        }

        private long GetPart1Answer(List<string> directions)
        {
            var currentPosition = (x: 0, y: 0);
            var currentDirection = (x: 1, y: 0);

            foreach (var direction in directions)
            {
                var amount = int.Parse(direction.Substring(1));
                currentPosition = direction[0] switch
                {
                    'N' => AddVectors(MultiplyScalar((0, 1), amount), currentPosition),
                    'S' => AddVectors(MultiplyScalar((0, -1), amount), currentPosition),
                    'E' => AddVectors(MultiplyScalar((1, 0), amount), currentPosition),
                    'W' => AddVectors(MultiplyScalar((-1, 0), amount), currentPosition),
                    'F' => AddVectors(MultiplyScalar(currentDirection, amount), currentPosition),
                    _ => currentPosition
                };
                currentDirection = direction[0] switch
                {
                    'L' => RotateLeft(currentDirection, amount),
                    'R' => RotateLeft(currentDirection, 360 - amount),
                    _ => currentDirection
                };
            }

            return Math.Abs(currentPosition.x) + Math.Abs(currentPosition.y);
        }

        private long GetPart2Answer(List<string> directions)
        {
            return -1;
        }

        private (int x, int y) AddVectors((int x, int y) point1, (int x, int y) point2)
        {
            return (point1.x + point2.x, point1.y + point2.y);
        }

        private (int x, int y) MultiplyScalar((int x, int y) point, int magnitude)
        {
            return (point.x * magnitude, point.y * magnitude);
        }

        private (int x, int y) RotateLeft((int x, int y) currentDirection, int amount)
        {
            var cosTheta = Convert.ToInt32(Math.Cos(amount * Math.PI / 180));
            var sinTheta = Convert.ToInt32(Math.Sin(amount * Math.PI / 180));
            return (currentDirection.x * cosTheta - currentDirection.y * sinTheta,
                currentDirection.x * sinTheta + currentDirection.y * cosTheta);
        }
    }
}