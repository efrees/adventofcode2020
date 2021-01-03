using System;
using System.Collections.Generic;

namespace AdventOfCode2020
{
    public static class StringExtensions
    {
        public static IEnumerable<string> SplitIntoLines(this string inputText)
        {
            if (inputText == null)
                return new string[] { };

            return inputText
                .Replace("\r\n", "\n")
                .Replace("\r", "")
                .Trim()
                .Split('\n');
        }

        public static IEnumerable<string> SplitRemovingEmpty(this string inputText, params char[] separators)
        {
            if (inputText == null)
                return new string[] { };

            return inputText.Split(separators, StringSplitOptions.RemoveEmptyEntries);
        }
    }
}
