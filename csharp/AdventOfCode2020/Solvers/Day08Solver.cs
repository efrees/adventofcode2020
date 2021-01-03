using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode2020.Solvers
{
    internal class Day08Solver : ISolver
    {
        private const string Name = "Day 8";
        private const string InputFile = "day08input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var program = Input.GetLinesFromFile(InputFile).Select(ParseInstructions).ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(program)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(program)}");
        }

        private Instruction ParseInstructions(string line)
        {
            var parts = line.Split(" ");
            return new Instruction
            {
                OpCode = parts[0],
                Arg = int.Parse(parts[1])
            };
        }

        private int GetPart1Answer(List<Instruction> instructions)
        {
            var (accumulator, _) = RunToExit(instructions);
            return accumulator;
        }

        private int GetPart2Answer(List<Instruction> instructions)
        {
            foreach (var instructionToFlip in instructions)
            {
                if (instructionToFlip.OpCode == "acc")
                {
                    continue;
                }

                instructionToFlip.OpCode = instructionToFlip.OpCode switch { "nop" => "jmp", "jmp" => "nop" };
                var (accumulator, exitedNormally) = RunToExit(instructions);
                instructionToFlip.OpCode = instructionToFlip.OpCode switch { "nop" => "jmp", "jmp" => "nop" };

                if (exitedNormally)
                {
                    return accumulator;
                }
            }

            return -1;
        }

        private static (int accumulator, bool exitedNormally) RunToExit(List<Instruction> instructions)
        {
            var visitedInstructions = new HashSet<int>();
            var accumulator = 0;
            var instruction = 0;

            while (!visitedInstructions.Contains(instruction) && instruction < instructions.Count)
            {
                visitedInstructions.Add(instruction);

                var instr = instructions[instruction];
                switch (instr.OpCode)
                {
                    case "nop": break;
                    case "acc":
                        accumulator += instr.Arg;
                        break;
                    case "jmp":
                        instruction += instr.Arg - 1;
                        break;
                }

                instruction++;
            }

            return (accumulator, exitedNormally: instruction >= instructions.Count );
        }

        private class Instruction
        {
            public string OpCode { get; set; }
            public int Arg { get; set; }
        }
    }
}