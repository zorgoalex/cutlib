using System.Linq;

public static class ResultMapper
{
    public static LibCutResult Map(Order order)
    {
        var result = new LibCutResult
        {
            SheetSize = new LibCutSheetRequest
            {
                LengthMm = order.parameters.ListLength_mm,
                WidthMm = order.parameters.ListWidth_mm,
            },
            SheetsUsed = order.SheetCount,
            PartsPlaced = order.PartsPlased,
            PartsTotal = order.Parts.Sum(part => part.Amount),
        };

        long totalSq = (long)order.parameters.ListLength_mm * order.parameters.ListWidth_mm * order.SheetCount;
        long placedSq = order.Parts.Sum(part => part.Sq * part.nPlased);
        result.EfficiencyPercent = totalSq > 0 ? Math.Round((double)placedSq / totalSq * 100, 1) : 0;

        for (int sheetNumber = 1; sheetNumber <= order.SheetCount; sheetNumber++)
        {
            var sheet = new LibCutSheetResult
            {
                Sheet = sheetNumber,
            };

            foreach (var part in order.Parts)
            {
                foreach (var coord in part.Coords)
                {
                    if (!coord.Cutted || coord.list != sheetNumber)
                        continue;

                    sheet.Parts.Add(new LibCutPartPlacement
                    {
                        Name = string.IsNullOrEmpty(part.Name) ? $"Part#{part.Npart + 1}" : part.Name,
                        Length = part.Length_mm,
                        Width = part.Width_mm,
                        X = coord.X,
                        Y = coord.Y,
                        Rotated = coord.isTurn,
                    });
                }
            }

            sheet.Offcuts = order.NSnips
                .Where(snip => snip.list == sheetNumber && snip.Length_mm > 0 && snip.Width_mm > 0)
                .Select(snip => new LibCutOffcut
                {
                    Length = snip.Length_mm,
                    Width = snip.Width_mm,
                    X = snip.X,
                    Y = snip.Y,
                })
                .ToList();

            result.Sheets.Add(sheet);
        }

        return result;
    }
}
