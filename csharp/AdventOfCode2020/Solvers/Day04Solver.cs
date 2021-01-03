using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;

namespace AdventOfCode2020.Solvers
{
    internal class Day04Solver : ISolver
    {
        private const string Name = "Day 4";
        private const string InputFile = "day04input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var passports = CombinePassportLines(Input.GetLinesFromFile(InputFile))
                .Select(ParseFields)
                .ToList();

            Console.WriteLine($"Output (part 1): {GetPart1Answer(passports)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(passports)}");
        }

        private IEnumerable<string> CombinePassportLines(IEnumerable<string> lines)
        {
            var builder = new StringBuilder();
            foreach (var line in lines)
            {
                if (string.IsNullOrEmpty(line))
                {
                    yield return builder.ToString().TrimEnd();
                    builder.Clear();
                }

                builder.Append(line);
                builder.Append(" ");
            }

            if (builder.Length > 0)
            {
                yield return builder.ToString().TrimEnd();
            }
        }

        private IList<PassportField> ParseFields(string rawPassport)
        {
            var matches = Regex.Matches(rawPassport, @"\b(\w{3}):([#\w]+)\b");
            return matches.Select(match => new PassportField
            {
                FieldName = match.Groups[1].Value,
                Value = match.Groups[2].Value
            }).ToList();
        }

        private int GetPart1Answer(List<IList<PassportField>> passports)
        {
            var requiredFields = new HashSet<string> { "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" };
            return passports.Count(passport => passport.Select(f => f.FieldName).ToHashSet().IsSupersetOf(requiredFields));
        }

        private int GetPart2Answer(List<IList<PassportField>> passports)
        {
            var requiredFields = new HashSet<string> { "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" };
            return passports
                .Where(passport => passport.Select(f => f.FieldName).ToHashSet().IsSupersetOf(requiredFields))
                .Count(passport => passport.All(FieldIsValid));
        }

        private static bool FieldIsValid(PassportField field)
        {
            return field.FieldName switch
            {
                "byr" => IsYearInRange(field.Value, 1920, 2002),
                "iyr" => IsYearInRange(field.Value, 2010, 2020),
                "eyr" => IsYearInRange(field.Value, 2020, 2030),
                "hgt" => IsValidHeight(field.Value),
                "hcl" => IsHexColor(field.Value),
                "ecl" => IsEyeColor(field.Value),
                "pid" => IsValidPassportId(field.Value),
                _ => true
            };
        }

        private static bool IsYearInRange(string fieldValue, int low, int high)
        {
            return IsNumberInRange(fieldValue, low, high);
        }

        private static bool IsValidHeight(string fieldValue)
        {
            var match = Regex.Match(fieldValue, @"(\d+)(cm|in)");

            return match.Success && match.Groups[2].Value switch
            {
                "cm" => IsNumberInRange(match.Groups[1].Value, 150, 193),
                "in" => IsNumberInRange(match.Groups[1].Value, 59, 76),
                _ => false
            };
        }

        private static bool IsNumberInRange(string fieldValue, int low, int high)
        {
            return int.TryParse(fieldValue, out var numberValue) && numberValue >= low && numberValue <= high;
        }

        private static bool IsHexColor(string fieldValue)
        {
            return Regex.IsMatch(fieldValue, @"#[0-9a-f]{6}");
        }

        private static bool IsEyeColor(string fieldValue)
        {
            return fieldValue.Length == 3 && new[] { "amb", "blu", "brn", "gry", "grn", "hzl", "oth" }.Contains(fieldValue);
        }

        private static bool IsValidPassportId(string fieldValue)
        {
            return Regex.IsMatch(fieldValue, @"^\d{9}$");
        }

        private class PassportField
        {
            public string FieldName { get; set; }
            public string Value { get; set; }
        }
    }
}