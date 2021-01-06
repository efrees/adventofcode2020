using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day18Solver : ISolver
    {
        private const string Name = "Day 18";
        private const string InputFile = "day18input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var expressions = Input.GetLinesFromFile(InputFile).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(expressions)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(expressions)}");
        }

        private long GetPart1Answer(List<string> expressions)
        {
            return expressions.Select(EvaluateForPart1).Sum();
        }

        private long EvaluateForPart1(string expression)
        {
            var tokens = expression.Replace("(", "( ").Replace(")", " )").Split(" ").ToList();
            return EvaluateForPart1Recursively(tokens);
        }

        private long EvaluateForPart1Recursively(List<string> tokens)
        {
            var result = 0L;
            var pendingOperator = "";

            while (tokens.Any())
            {
                var token = tokens[0];
                tokens.RemoveAt(0);

                if (token == "*" || token == "+")
                {
                    pendingOperator = token;
                    continue;
                }

                if (token == ")")
                {
                    return result;
                }

                var nextValue = token == "("
                    ? EvaluateForPart1Recursively(tokens)
                    : long.Parse(token);

                if (pendingOperator == string.Empty)
                {
                    result = nextValue;
                }
                else
                {
                    result = pendingOperator switch
                    {
                        "*" => result * nextValue,
                        "+" => result + nextValue,
                        _ => result
                    };
                }
            }

            return result;
        }

        private long GetPart2Answer(List<string> expressions)
        {
            return expressions.Select(EvaluateForPart2).Sum();
        }

        private long EvaluateForPart2(string expression)
        {
            var tokens = expression.Replace("(", "( ").Replace(")", " )").Split(" ").ToList();
            return EvaluateForPart2Recursively(tokens);
        }

        private long EvaluateForPart2Recursively(List<string> tokens)
        {
            var pendingOperands = new Stack<long>();
            var pendingAddition = false;

            while (tokens.Any())
            {
                var token = tokens[0];
                tokens.RemoveAt(0);

                if (token == "*")
                {
                    //We'll calculate at the end
                    continue;
                }

                if (token == "+")
                {
                    pendingAddition = true;
                    continue;
                }

                if (token == ")")
                {
                    break;
                }

                var nextValue = token == "("
                    ? EvaluateForPart2Recursively(tokens)
                    : long.Parse(token);

                if (pendingAddition)
                {
                    pendingAddition = false;
                    pendingOperands.Push(pendingOperands.Pop() + nextValue);
                }
                else
                {
                    pendingOperands.Push(nextValue);
                }
            }

            return pendingOperands.Aggregate((product, next) => product * next);
        }
    }
}