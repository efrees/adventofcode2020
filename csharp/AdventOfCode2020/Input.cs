using System.Collections.Generic;
using System.Diagnostics;
using System.IO;

namespace AdventOfCode2020
{
    public static class Input
    {
        private static readonly string BinDirectory = Path.GetDirectoryName(Process.GetCurrentProcess().MainModule.FileName);

        internal static string GetInputFromFile(string filename)
        {
            var absolutePath = Path.Combine(BinDirectory, $"InputFiles/{filename}");
            return File.ReadAllText(absolutePath);
        }

        internal static IEnumerable<string> GetLinesFromFile(string filename)
        {
            return GetInputFromFile(filename).SplitIntoLines();
        }
    }
}
