using System;
using System.Collections.Generic;
using System.IO;
using System.Text;

namespace AdventOfCode2020
{
    public static class Input
    {
        internal static string GetInputFromFile(string filename)
        {
            return File.ReadAllText("../../InputFiles/" + filename);
        }

        internal static IEnumerable<string> GetLinesFromFile(string filename)
        {
            return GetInputFromFile(filename).SplitIntoLines();
        }
    }
}
