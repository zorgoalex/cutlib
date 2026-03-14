using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;
using System.Threading;

class Program
{
    static void Main(string[] args)
    {
        if (args.Length == 0 || args[0] == "--help" || args[0] == "-h")
        {
            PrintUsage();
            return;
        }

        string inputFile = null;
        string outputFile = null;
        int algorithm = 3;
        int sheetLength = 0, sheetWidth = 0;
        int blade = 4, padding = 0;
        string format = "text";

        for (int i = 0; i < args.Length; i++)
        {
            switch (args[i])
            {
                case "-i": case "--input":   inputFile = args[++i]; break;
                case "-o": case "--output":  outputFile = args[++i]; break;
                case "-a": case "--algorithm":
                    var algStr = args[++i].ToLower();
                    algorithm = algStr switch
                    {
                        "length" or "l" or "1" => 1,
                        "width" or "w" or "2"  => 2,
                        "optimal" or "opt" or "3" => 3,
                        _ => 3
                    };
                    break;
                case "-s": case "--sheet":
                    var dims = args[++i].Split('x', 'X', '*');
                    sheetLength = int.Parse(dims[0]);
                    sheetWidth = int.Parse(dims[1]);
                    break;
                case "-b": case "--blade":   blade = int.Parse(args[++i]); break;
                case "-p": case "--padding": padding = int.Parse(args[++i]); break;
                case "-f": case "--format":  format = args[++i].ToLower(); break;
            }
        }

        if (inputFile == null)
        {
            Console.Error.WriteLine("Error: input file required (-i)");
            return;
        }

        var order = LoadInput(inputFile, sheetLength, sheetWidth, blade, padding, algorithm);
        if (order == null) return;

        Console.Error.WriteLine($"Sheet: {order.parameters.ListLength_mm}x{order.parameters.ListWidth_mm} mm");
        Console.Error.WriteLine($"Blade: {order.parameters.Blade} mm, Padding: {order.parameters.Padding} mm");
        Console.Error.WriteLine($"Algorithm: {AlgName(order.parameters.Algoritm)}");
        Console.Error.WriteLine($"Parts: {order.Parts.Count} types, {order.Parts.Sum(p => p.Amount)} total pieces");

        RunCutting(order);

        string result = format == "json" ? FormatResultJson(order) : FormatResultText(order);

        if (outputFile != null)
        {
            File.WriteAllText(outputFile, result);
            Console.Error.WriteLine($"Results written to {outputFile}");
        }
        else
        {
            Console.WriteLine(result);
        }
    }

    static void PrintUsage()
    {
        Console.WriteLine(@"SketchCut CLI - 2D sheet cutting optimizer

Usage: SketchCutCLI -i <input> [options]

Options:
  -i, --input <file>       Input CSV/JSON file with parts list
  -o, --output <file>      Output file (default: stdout)
  -s, --sheet <LxW>        Sheet size in mm, e.g. 2440x1220
  -b, --blade <mm>         Blade/kerf width in mm (default: 4)
  -p, --padding <mm>       Edge padding in mm (default: 0)
  -a, --algorithm <alg>    Algorithm: length|width|optimal (default: optimal)
  -f, --format <fmt>       Output format: text|json (default: text)

Input CSV format (separator: semicolon, comma or tab):
  length_mm;width_mm;quantity;can_rotate(0/1);name

Input JSON format:
  {
    ""sheet"": { ""length"": 2440, ""width"": 1220 },
    ""blade"": 4,
    ""padding"": 10,
    ""algorithm"": ""optimal"",
    ""parts"": [
      { ""length"": 800, ""width"": 400, ""qty"": 5, ""rotate"": true, ""name"": ""A"" }
    ]
  }");
    }

    static Order LoadInput(string file, int sheetL, int sheetW, int blade, int padding, int alg)
    {
        var order = new Order();
        order.parameters.Blade = blade;
        order.parameters.Padding = padding;
        order.parameters.Algoritm = alg;

        string ext = Path.GetExtension(file).ToLower();
        string content = File.ReadAllText(file);

        if (ext == ".json")
        {
            var doc = JsonDocument.Parse(content);
            var root = doc.RootElement;

            if (root.TryGetProperty("sheet", out var sh))
            {
                sheetL = sh.GetProperty("length").GetInt32();
                sheetW = sh.GetProperty("width").GetInt32();
            }
            if (root.TryGetProperty("blade", out var bl)) blade = bl.GetInt32();
            if (root.TryGetProperty("padding", out var pd)) padding = pd.GetInt32();
            if (root.TryGetProperty("algorithm", out var al))
            {
                var a = al.GetString()?.ToLower();
                alg = a switch
                {
                    "length" or "l" or "1" => 1,
                    "width" or "w" or "2"  => 2,
                    _ => 3
                };
            }

            order.parameters.Blade = blade;
            order.parameters.Padding = padding;
            order.parameters.Algoritm = alg;

            if (root.TryGetProperty("parts", out var parts))
            {
                int idx = 0;
                foreach (var p in parts.EnumerateArray())
                {
                    var part = new Part();
                    part.Length_mm = p.GetProperty("length").GetInt32();
                    part.Width_mm = p.GetProperty("width").GetInt32();
                    part.Sq = (long)part.Length_mm * part.Width_mm;
                    part.Amount = p.TryGetProperty("qty", out var q) ? q.GetInt32() : 1;
                    part.Turn = !p.TryGetProperty("rotate", out var r) || r.GetBoolean();
                    part.Name = p.TryGetProperty("name", out var n) ? n.GetString() ?? "" : "";
                    part.Npart = idx++;
                    for (int c = 0; c < part.Amount; c++)
                        part.Coords.Add(new Coord());
                    order.Parts.Add(part);
                }
            }
        }
        else
        {
            var lines = content.Split('\n', StringSplitOptions.RemoveEmptyEntries);
            int idx = 0;
            int lineNumber = 0;
            foreach (var line in lines)
            {
                lineNumber++;
                var trimmed = line.Trim();
                if (trimmed.StartsWith("#") || trimmed.Length == 0) continue;
                var cols = trimmed.Split(';', ',', '\t');
                if (cols.Length < 3) continue;

                if (!int.TryParse(cols[0].Trim(), out var length) ||
                    !int.TryParse(cols[1].Trim(), out var width) ||
                    !int.TryParse(cols[2].Trim(), out var qty))
                {
                    bool looksLikeHeader =
                        cols[0].Trim().Equals("length", StringComparison.OrdinalIgnoreCase) &&
                        cols[1].Trim().Equals("width", StringComparison.OrdinalIgnoreCase) &&
                        cols[2].Trim().Equals("qty", StringComparison.OrdinalIgnoreCase);

                    if (looksLikeHeader)
                        continue;

                    Console.Error.WriteLine($"Error: invalid CSV row at line {lineNumber}: {trimmed}");
                    return null;
                }

                var part = new Part();
                part.Length_mm = length;
                part.Width_mm = width;
                part.Amount = qty;
                part.Sq = (long)part.Length_mm * part.Width_mm;
                part.Turn = cols.Length > 3 && cols[3].Trim() == "1";
                part.Name = cols.Length > 4 ? cols[4].Trim() : "";
                part.Npart = idx++;
                for (int c = 0; c < part.Amount; c++)
                    part.Coords.Add(new Coord());
                order.Parts.Add(part);
            }
        }

        if (sheetL == 0 || sheetW == 0)
        {
            Console.Error.WriteLine("Error: sheet size required (-s LxW or in JSON)");
            return null;
        }

        order.parameters.ListLength_mm = sheetL;
        order.parameters.ListWidth_mm = sheetW;
        order.sheet = new Sheet { Length = sheetL, Width = sheetW };

        foreach (var p in order.Parts)
            order.PartsSq += p.Sq * p.Amount;

        return order;
    }

    static void RunCutting(Order order)
    {
        order.SheetCount = 0;
        order.PartsPlased = 0;
        order.NSnips = new List<Snip>();
        order.UsedSnipsCount = 0;
        foreach (var p in order.Parts)
        {
            p.nPlased = 0;
            for (int c = 0; c < p.Coords.Count; c++)
                p.Coords[c] = new Coord();
        }

        var parts = AlgUtils.ConvertParts_to_CParts(order.Parts);
        int alg = order.parameters.Algoritm;
        var allSheets = new List<CSheet>();

        while (true)
        {
            int ll = order.parameters.ListLength_mm * 10;
            int lw = order.parameters.ListWidth_mm * 10;
            int p = order.parameters.Padding * 10;

            if (!AlgUtils.FastFindFirst_CPart(parts, ll - p, lw - p))
                break;

            CSheet bestSheet = null;

            if (alg == 1 || alg == 3)
            {
                var results = RunParallelVariants(parts, order, 1);
                var best = PickBest(results, 1);
                if (best != null && (bestSheet == null || best.Parts_Sq > bestSheet.Parts_Sq))
                    bestSheet = best;
            }

            if (alg == 2 || alg == 3)
            {
                var results = RunParallelVariants(parts, order, 2);
                var best = PickBest(results, 2);
                if (best != null && (bestSheet == null || best.Parts_Sq > bestSheet.Parts_Sq))
                    bestSheet = best;
            }

            if (alg == 3)
            {
                var optParts = AlgUtils.Copy_CParts(parts);
                var optAlg = new Opt_Alg_Width_and_Length();
                var optSheet = optAlg.Get_Sheet_OPT_ALG_2(optParts, ll, lw,
                    order.parameters.Blade * 10, order.parameters.Padding * 10,
                    true, true, false, true, true, 3);
                if (optSheet != null && (bestSheet == null || optSheet.Parts_Sq > bestSheet.Parts_Sq))
                {
                    bestSheet = optSheet;
                    bestSheet.Alg = 3;
                }
            }

            if (bestSheet == null || bestSheet.Parts_Sq <= 0)
                break;

            MarkPlaced(parts, bestSheet);

            int sameCount = CountSameSheets(bestSheet, parts);
            allSheets.Add(bestSheet);
            for (int s = 0; s < sameCount; s++)
            {
                MarkPlaced(parts, bestSheet);
                allSheets.Add(bestSheet);
            }

            // Clean parts list
            for (int i = 0; i < parts.Count; i++)
            {
                if (parts[i].Plased >= parts[i].Qty)
                {
                    parts.RemoveAt(i);
                    i--;
                }
            }
        }

        for (int i = 0; i < allSheets.Count; i++)
            if (allSheets[i].Alg == 0)
                allSheets[i].Alg = alg == 3 ? 3 : alg;

        AlgUtils.Write_Sheets_to_Order(order, allSheets);
    }

    static List<CSheet> RunParallelVariants(List<CPart> parts, Order order, int algType)
    {
        var variants = new (bool sameMax, bool maxSq, bool optiOn, bool turnOn)[]
        {
            (true, true, true, true),
            (true, false, true, true),
            (true, false, false, true),
            (true, false, false, false),
            (false, true, true, true),
            (false, true, false, true),
            (false, false, true, true),
            (false, false, false, true),
        };

        int ll = order.parameters.ListLength_mm * 10;
        int lw = order.parameters.ListWidth_mm * 10;
        int bl = order.parameters.Blade * 10;
        int pd = order.parameters.Padding * 10;

        var results = new CSheet[variants.Length + 1];
        var threads = new Thread[variants.Length + 1];

        // Base variant
        var baseParts = AlgUtils.Copy_CParts(parts);
        threads[0] = new Thread(() =>
        {
            if (algType == 1)
            {
                var la = new Length_Alg();
                results[0] = la.GetCSheet_LENGTH_CUT(baseParts, ll, lw, bl, pd, true, true, true);
            }
            else
            {
                var wa = new Width_Alg();
                results[0] = wa.GetCSheet_WIDTH_CUT(baseParts, ll, lw, bl, pd, true, true, true);
            }
        });
        threads[0].IsBackground = true;
        threads[0].Start();

        // L16/W16 variants
        for (int v = 0; v < variants.Length; v++)
        {
            int vi = v;
            var vp = AlgUtils.Copy_CParts(parts);
            var prm = new LW16(variants[vi].sameMax, variants[vi].maxSq, variants[vi].optiOn, variants[vi].turnOn);
            threads[vi + 1] = new Thread(() =>
            {
                try
                {
                    if (algType == 1)
                    {
                        var l2 = new Length2();
                        results[vi + 1] = l2.GetCSheet_LENGTH_CUT(vp, ll, lw, bl, pd, true, prm, order.PartsSq, 0, out _);
                    }
                    else
                    {
                        var w2 = new Width2();
                        results[vi + 1] = w2.GetCSheet_WIDTH_CUT(vp, ll, lw, bl, pd, true, prm, order.PartsSq, 0, out _);
                    }
                }
                catch { /* variant failed, skip */ }
            });
            threads[vi + 1].IsBackground = true;
            threads[vi + 1].Start();
        }

        foreach (var t in threads)
            t.Join(TimeSpan.FromSeconds(30));

        return results.Where(s => s != null).ToList();
    }

    static CSheet PickBest(List<CSheet> sheets, int algType)
    {
        if (sheets.Count == 0) return null;
        var best = sheets[0];
        for (int i = 1; i < sheets.Count; i++)
        {
            if (sheets[i].Parts_Sq > best.Parts_Sq)
                best = sheets[i];
            else if (sheets[i].Parts_Sq == best.Parts_Sq)
            {
                if (algType == 1 && sheets[i].Remain.W > best.Remain.W)
                    best = sheets[i];
                else if (algType == 2 && sheets[i].Remain.L > best.Remain.L)
                    best = sheets[i];
            }
        }
        best.Alg = algType;
        return best;
    }

    static void MarkPlaced(List<CPart> parts, CSheet sheet)
    {
        foreach (var line in sheet.Lines)
            foreach (var pid in line.PartIDs)
            {
                int idx = pid < -1 ? pid * -1 - 2 : pid;
                if (idx >= 0 && idx < parts.Count)
                    parts[idx].Plased++;
            }
    }

    static int CountSameSheets(CSheet sh, List<CPart> parts)
    {
        var used = new Dictionary<int, int>();
        foreach (var line in sh.Lines)
            foreach (var pid in line.PartIDs)
            {
                int idx = pid < -1 ? pid * -1 - 2 : pid;
                used[idx] = used.GetValueOrDefault(idx) + 1;
            }
        if (used.Count == 0) return 0;

        int min = int.MaxValue;
        foreach (var kv in used)
        {
            int remaining = parts[kv.Key].Qty - parts[kv.Key].Plased;
            int possible = remaining / kv.Value;
            if (possible < min) min = possible;
        }
        return Math.Max(0, min);
    }

    static string AlgName(int alg) => alg switch { 1 => "Length", 2 => "Width", _ => "Optimal" };

    static string FormatResultText(Order order)
    {
        var sb = new System.Text.StringBuilder();
        sb.AppendLine("=== CUTTING RESULTS ===");
        sb.AppendLine($"Sheet: {order.parameters.ListLength_mm} x {order.parameters.ListWidth_mm} mm");
        sb.AppendLine($"Sheets used: {order.SheetCount}");
        sb.AppendLine($"Parts placed: {order.PartsPlased} / {order.Parts.Sum(p => p.Amount)}");

        long totalSq = (long)order.parameters.ListLength_mm * order.parameters.ListWidth_mm * order.SheetCount;
        long placedSq = order.Parts.Sum(p => p.Sq * p.nPlased);
        if (totalSq > 0)
            sb.AppendLine($"Material efficiency: {(double)placedSq / totalSq * 100:F1}%");

        sb.AppendLine();
        sb.AppendLine("--- Parts placement ---");
        foreach (var p in order.Parts)
        {
            string name = string.IsNullOrEmpty(p.Name) ? $"Part#{p.Npart + 1}" : p.Name;
            sb.AppendLine($"{name}: {p.Length_mm}x{p.Width_mm} mm, placed {p.nPlased}/{p.Amount}");
            foreach (var c in p.Coords)
                if (c.Cutted)
                {
                    string rot = c.isTurn ? " [rotated]" : "";
                    sb.AppendLine($"    Sheet {c.list}: ({c.X}, {c.Y}){rot}");
                }
        }

        if (order.NSnips.Any(s => s.Length_mm > 0 && s.Width_mm > 0))
        {
            sb.AppendLine();
            sb.AppendLine("--- Waste/offcuts ---");
            foreach (var s in order.NSnips)
                if (s.Length_mm > 0 && s.Width_mm > 0)
                    sb.AppendLine($"  Sheet {s.list}: {s.Length_mm}x{s.Width_mm} mm at ({s.X}, {s.Y})");
        }

        return sb.ToString();
    }

    static string FormatResultJson(Order order)
    {
        var sheets = new List<object>();
        for (int s = 1; s <= order.SheetCount; s++)
        {
            var partsOnSheet = new List<object>();
            foreach (var p in order.Parts)
                foreach (var c in p.Coords)
                    if (c.Cutted && c.list == s)
                        partsOnSheet.Add(new
                        {
                            name = string.IsNullOrEmpty(p.Name) ? $"Part#{p.Npart + 1}" : p.Name,
                            length = p.Length_mm, width = p.Width_mm,
                            x = c.X, y = c.Y, rotated = c.isTurn
                        });

            var waste = order.NSnips
                .Where(sn => sn.list == s && sn.Length_mm > 0 && sn.Width_mm > 0)
                .Select(sn => new { length = sn.Length_mm, width = sn.Width_mm, x = sn.X, y = sn.Y })
                .ToList<object>();

            sheets.Add(new { sheet = s, parts = partsOnSheet, offcuts = waste });
        }

        long totalSq = (long)order.parameters.ListLength_mm * order.parameters.ListWidth_mm * order.SheetCount;
        long placedSq = order.Parts.Sum(p => p.Sq * p.nPlased);

        var result = new
        {
            sheetSize = new { length = order.parameters.ListLength_mm, width = order.parameters.ListWidth_mm },
            sheetsUsed = order.SheetCount,
            partsPlaced = order.PartsPlased,
            partsTotal = order.Parts.Sum(p => p.Amount),
            efficiencyPercent = totalSq > 0 ? Math.Round((double)placedSq / totalSq * 100, 1) : 0,
            sheets
        };

        return JsonSerializer.Serialize(result, new JsonSerializerOptions { WriteIndented = true });
    }
}
