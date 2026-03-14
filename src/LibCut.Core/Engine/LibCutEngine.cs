using System.Linq;
using System.Threading;

public sealed class LibCutEngine
{
    public LibCutResult Optimize(LibCutRequest request)
    {
        LibCutRequestValidator.Validate(request);

        var order = OrderFactory.Create(request);
        RunCutting(order);
        return ResultMapper.Map(order);
    }

    private static void RunCutting(Order order)
    {
        order.SheetCount = 0;
        order.PartsPlased = 0;
        order.NSnips = new List<Snip>();
        order.UsedSnipsCount = 0;

        foreach (var part in order.Parts)
        {
            part.nPlased = 0;
            for (int i = 0; i < part.Coords.Count; i++)
                part.Coords[i] = new Coord();
        }

        var parts = AlgUtils.ConvertParts_to_CParts(order.Parts);
        int algorithm = order.parameters.Algoritm;
        var allSheets = new List<CSheet>();

        while (true)
        {
            int listLength = order.parameters.ListLength_mm * 10;
            int listWidth = order.parameters.ListWidth_mm * 10;
            int padding = order.parameters.Padding * 10;

            if (!AlgUtils.FastFindFirst_CPart(parts, listLength - padding, listWidth - padding))
                break;

            CSheet? bestSheet = null;

            if (algorithm == 1 || algorithm == 3)
            {
                var results = RunParallelVariants(parts, order, 1);
                var best = PickBest(results, 1);
                if (best != null && (bestSheet == null || best.Parts_Sq > bestSheet.Parts_Sq))
                    bestSheet = best;
            }

            if (algorithm == 2 || algorithm == 3)
            {
                var results = RunParallelVariants(parts, order, 2);
                var best = PickBest(results, 2);
                if (best != null && (bestSheet == null || best.Parts_Sq > bestSheet.Parts_Sq))
                    bestSheet = best;
            }

            if (algorithm == 3)
            {
                var optimizedParts = AlgUtils.Copy_CParts(parts);
                var optimizedAlgorithm = new Opt_Alg_Width_and_Length();
                var optimizedSheet = optimizedAlgorithm.Get_Sheet_OPT_ALG_2(
                    optimizedParts,
                    listLength,
                    listWidth,
                    order.parameters.Blade * 10,
                    order.parameters.Padding * 10,
                    true,
                    true,
                    false,
                    true,
                    true,
                    3);

                if (optimizedSheet != null && (bestSheet == null || optimizedSheet.Parts_Sq > bestSheet.Parts_Sq))
                {
                    bestSheet = optimizedSheet;
                    bestSheet.Alg = 3;
                }
            }

            if (bestSheet == null || bestSheet.Parts_Sq <= 0)
                break;

            MarkPlaced(parts, bestSheet);

            int sameCount = CountSameSheets(bestSheet, parts);
            allSheets.Add(bestSheet);
            for (int sheetIndex = 0; sheetIndex < sameCount; sheetIndex++)
            {
                MarkPlaced(parts, bestSheet);
                allSheets.Add(bestSheet);
            }

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
        {
            if (allSheets[i].Alg == 0)
                allSheets[i].Alg = algorithm == 3 ? 3 : algorithm;
        }

        AlgUtils.Write_Sheets_to_Order(order, allSheets);
    }

    private static List<CSheet> RunParallelVariants(List<CPart> parts, Order order, int algorithmType)
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

        int listLength = order.parameters.ListLength_mm * 10;
        int listWidth = order.parameters.ListWidth_mm * 10;
        int blade = order.parameters.Blade * 10;
        int padding = order.parameters.Padding * 10;

        var results = new CSheet?[variants.Length + 1];
        var threads = new Thread[variants.Length + 1];

        var baseParts = AlgUtils.Copy_CParts(parts);
        threads[0] = new Thread(() =>
        {
            if (algorithmType == 1)
            {
                var lengthAlgorithm = new Length_Alg();
                results[0] = lengthAlgorithm.GetCSheet_LENGTH_CUT(baseParts, listLength, listWidth, blade, padding, true, true, true);
            }
            else
            {
                var widthAlgorithm = new Width_Alg();
                results[0] = widthAlgorithm.GetCSheet_WIDTH_CUT(baseParts, listLength, listWidth, blade, padding, true, true, true);
            }
        });
        threads[0].IsBackground = true;
        threads[0].Start();

        for (int v = 0; v < variants.Length; v++)
        {
            int variantIndex = v;
            var variantParts = AlgUtils.Copy_CParts(parts);
            var parameters = new LW16(variants[variantIndex].sameMax, variants[variantIndex].maxSq, variants[variantIndex].optiOn, variants[variantIndex].turnOn);
            threads[variantIndex + 1] = new Thread(() =>
            {
                try
                {
                    if (algorithmType == 1)
                    {
                        var length2 = new Length2();
                        results[variantIndex + 1] = length2.GetCSheet_LENGTH_CUT(variantParts, listLength, listWidth, blade, padding, true, parameters, order.PartsSq, 0, out _);
                    }
                    else
                    {
                        var width2 = new Width2();
                        results[variantIndex + 1] = width2.GetCSheet_WIDTH_CUT(variantParts, listLength, listWidth, blade, padding, true, parameters, order.PartsSq, 0, out _);
                    }
                }
                catch
                {
                }
            });

            threads[variantIndex + 1].IsBackground = true;
            threads[variantIndex + 1].Start();
        }

        foreach (var thread in threads)
            thread.Join(TimeSpan.FromSeconds(30));

        return results.Where(sheet => sheet != null).Cast<CSheet>().ToList();
    }

    private static CSheet? PickBest(List<CSheet> sheets, int algorithmType)
    {
        if (sheets.Count == 0)
            return null;

        var best = sheets[0];
        for (int i = 1; i < sheets.Count; i++)
        {
            if (sheets[i].Parts_Sq > best.Parts_Sq)
            {
                best = sheets[i];
            }
            else if (sheets[i].Parts_Sq == best.Parts_Sq)
            {
                if (algorithmType == 1 && sheets[i].Remain.W > best.Remain.W)
                    best = sheets[i];
                else if (algorithmType == 2 && sheets[i].Remain.L > best.Remain.L)
                    best = sheets[i];
            }
        }

        best.Alg = algorithmType;
        return best;
    }

    private static void MarkPlaced(List<CPart> parts, CSheet sheet)
    {
        foreach (var line in sheet.Lines)
        {
            foreach (var partId in line.PartIDs)
            {
                int index = partId < -1 ? partId * -1 - 2 : partId;
                if (index >= 0 && index < parts.Count)
                    parts[index].Plased++;
            }
        }
    }

    private static int CountSameSheets(CSheet sheet, List<CPart> parts)
    {
        var used = new Dictionary<int, int>();

        foreach (var line in sheet.Lines)
        {
            foreach (var partId in line.PartIDs)
            {
                int index = partId < -1 ? partId * -1 - 2 : partId;
                used[index] = used.GetValueOrDefault(index) + 1;
            }
        }

        if (used.Count == 0)
            return 0;

        int min = int.MaxValue;
        foreach (var item in used)
        {
            int remaining = parts[item.Key].Qty - parts[item.Key].Plased;
            int possible = remaining / item.Value;
            if (possible < min)
                min = possible;
        }

        return Math.Max(0, min);
    }
}
