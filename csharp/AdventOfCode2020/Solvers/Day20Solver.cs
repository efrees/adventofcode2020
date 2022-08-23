using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2020.Solvers
{
    internal class Day20Solver : ISolver
    {
        private const string Name = "Day 20";
        private const string InputFile = "day20input.txt";

        public void Solve()
        {
            Console.WriteLine(Name);
            var rawTiles = Input.GetInputFromFile(InputFile)
                .Trim().Replace("\r\n", "\n").Split("\n\n").Select(ParseTile)
                .ToDictionary(tile => tile.Id, tile => tile);

            Console.WriteLine($"Output (part 1): {GetPart1Answer(rawTiles)}");
            Console.WriteLine($"Output (part 2): {GetPart2Answer(rawTiles)}");
        }

        private Tile ParseTile(string tileLines)
        {
            var lines = tileLines.SplitIntoLines().ToArray();
            var id = int.Parse(lines[0].Substring("Tile ".Length).TrimEnd(':'));
            var data = lines[1..].Select(line => line.ToCharArray()).ToArray();
            return new Tile
            {
                Id = id,
                Pixels = data
            };
        }

        private long GetPart1Answer(Dictionary<int, Tile> tiles)
        {
            var tileEdges = tiles.Values.Select(tile => (tile.Id, GetAllEdges(tile))).ToDictionary(x => x.Id, x => x.Item2);
            var alignments = new List<TileAlignment>();
            var usedTiles = new HashSet<int>();
            var lookupByEdge = tileEdges.SelectMany(pair => pair.Value.Select(v => (pair.Key, edge: v))).ToLookup(x => x.edge, x => x.Key);
            AlignAllTiles(usedTiles, tileEdges, lookupByEdge, tiles, alignments);

            var imageSize = Convert.ToInt32(Math.Sqrt(tiles.Count));

            return (long)alignments[0].TileId
                * alignments[imageSize - 1].TileId
                * alignments[imageSize * (imageSize - 1)].TileId
                * alignments.Last().TileId;
        }

        private long GetPart2Answer(Dictionary<int, Tile> tiles)
        {
            return -1;
        }

        private bool AlignAllTiles(HashSet<int> usedTiles,
            Dictionary<int, int[]> tileEdges,
            ILookup<int, int> lookupByEdge,
            Dictionary<int, Tile> tileData,
            List<TileAlignment> alignments)
        {
            if (!tileData.Keys.Except(usedTiles).Any())
            {
                return true;
            }

            var imageSize = Convert.ToInt32(Math.Sqrt(tileData.Count));
            var nextTileIndex = alignments.Count;
            var leftTile = nextTileIndex % imageSize > 0
                ? alignments.Last()
                : null;
            var requiredLeftEdge = leftTile is object
                ? tileEdges[leftTile.TileId][GetRightEdgeIndex(leftTile)]
                : -1;

            var topTile = nextTileIndex > imageSize
                ? alignments[nextTileIndex - imageSize]
                : null;
            var requiredTopEdge = topTile is object
                ? tileEdges[topTile.TileId][GetBottomEdgeIndex(topTile)]
                : -1;

            var tilesToTry = (requiredLeftEdge, requiredTopEdge) switch
            {
                (-1, -1) => tileData.Keys,
                (-1, _) => lookupByEdge[requiredTopEdge],
                (_, _) => lookupByEdge[requiredLeftEdge]
            };

            var availableTileIds = tilesToTry.Except(usedTiles).ToList();
            foreach (var tileId in availableTileIds)
            {
                var edgeValues = tileEdges[tileId];
                for (var i = 0; i < edgeValues.Length; i++)
                {
                    if (requiredTopEdge == -1 || edgeValues[i] == requiredTopEdge)
                    {
                        // Edges: [ T, R, B, L, T', R', B', L' ]
                        //                     (L),(B),(R),(T)
                        var actualTopEdgeIndex = 8 - 1 - i; // We actually matched the inverse.
                        var relativeLeftIndex = (actualTopEdgeIndex + 3) % 4 + actualTopEdgeIndex / 4 * 4;
                        if (requiredLeftEdge == -1 || edgeValues[8 - 1 - relativeLeftIndex] == requiredLeftEdge)
                        {
                            alignments.Add(new TileAlignment
                            {
                                TileId = tileId,
                                Alignment = actualTopEdgeIndex
                            });
                            usedTiles.Add(tileId);

                            if (AlignAllTiles(usedTiles, tileEdges, lookupByEdge, tileData, alignments))
                            {
                                return true;
                            }

                            usedTiles.Remove(tileId);
                            alignments.RemoveAt(alignments.Count - 1);
                        }
                    }
                }
            }

            return false;
        }

        private static int GetRightEdgeIndex(TileAlignment alignment)
        {
            var flip = alignment.Alignment / 4;
            var rotation = (alignment.Alignment + 1) % 4;
            return 4 * flip + rotation;
        }

        private static int GetBottomEdgeIndex(TileAlignment alignment)
        {
            var flip = alignment.Alignment / 4;
            var rotation = (alignment.Alignment + 2) % 4;
            return 4 * flip + rotation;
        }

        private int[] GetAllEdges(Tile tile)
        {
            var top = 0;
            var right = 0;
            var bottom = 0;
            var left = 0;
            var topPrime = 0;
            var rightPrime = 0;
            var bottomPrime = 0;
            var leftPrime = 0;

            var tileSize = tile.Pixels.Length;
            for (var i = 0; i < tileSize; i++)
            {
                top = (top << 1) + (tile.Pixels[0][i] == '#'
                    ? 1
                    : 0);
                right = (right << 1) + (tile.Pixels[i][^1] == '#'
                    ? 1
                    : 0);
                bottom = (bottom << 1) + (tile.Pixels[^1][^(i + 1)] == '#'
                    ? 1
                    : 0);
                left = (left << 1) + (tile.Pixels[^(i + 1)][0] == '#'
                    ? 1
                    : 0);

                // After a flip, top and left have changed places
                topPrime = (topPrime << 1) + (tile.Pixels[i][0] == '#'
                    ? 1
                    : 0);
                rightPrime = (rightPrime << 1) + (tile.Pixels[^1][i] == '#'
                    ? 1
                    : 0);
                bottomPrime = (bottomPrime << 1) + (tile.Pixels[^(i + 1)][^1] == '#'
                    ? 1
                    : 0);
                leftPrime = (leftPrime << 1) + (tile.Pixels[0][^(i + 1)] == '#'
                    ? 1
                    : 0);
            }

            return new[] { top, right, bottom, left, topPrime, rightPrime, bottomPrime, leftPrime };
        }

        private class Tile
        {
            public int Id { get; set; }
            public char[][] Pixels { get; set; }
        }

        private class TileAlignment
        {
            public int TileId { get; set; }

            /// <summary>
            /// Which edge is at top?
            /// </summary>
            public int Alignment { get; set; }
        }
    }
}