using System.Linq;
using System.Text;

public static class TextResultFormatter
{
    public static string Format(LibCutRequest request, LibCutResult result)
    {
        var sb = new StringBuilder();
        sb.AppendLine("=== CUTTING RESULTS ===");
        sb.AppendLine($"Sheet: {result.SheetSize.LengthMm} x {result.SheetSize.WidthMm} mm");
        sb.AppendLine($"Sheets used: {result.SheetsUsed}");
        sb.AppendLine($"Parts placed: {result.PartsPlaced} / {result.PartsTotal}");
        sb.AppendLine($"Material efficiency: {result.EfficiencyPercent:F1}%");
        sb.AppendLine();
        sb.AppendLine("--- Parts placement ---");

        foreach (var requestedPart in request.Parts)
        {
            string name = string.IsNullOrEmpty(requestedPart.Name) ? "Part" : requestedPart.Name;
            var placements = result.Sheets
                .SelectMany(sheet => sheet.Parts.Select(placement => new { sheet.Sheet, Placement = placement }))
                .Where(item =>
                    item.Placement.Name == name &&
                    item.Placement.Length == requestedPart.LengthMm &&
                    item.Placement.Width == requestedPart.WidthMm)
                .ToList();

            sb.AppendLine($"{name}: {requestedPart.LengthMm}x{requestedPart.WidthMm} mm, placed {placements.Count}/{requestedPart.Quantity}");
            foreach (var placement in placements)
            {
                string rotated = placement.Placement.Rotated ? " [rotated]" : "";
                sb.AppendLine($"    Sheet {placement.Sheet}: ({placement.Placement.X}, {placement.Placement.Y}){rotated}");
            }
        }

        var offcuts = result.Sheets.SelectMany(sheet => sheet.Offcuts.Select(offcut => new { sheet.Sheet, Offcut = offcut })).ToList();
        if (offcuts.Count > 0)
        {
            sb.AppendLine();
            sb.AppendLine("--- Waste/offcuts ---");
            foreach (var item in offcuts)
                sb.AppendLine($"  Sheet {item.Sheet}: {item.Offcut.Length}x{item.Offcut.Width} mm at ({item.Offcut.X}, {item.Offcut.Y})");
        }

        return sb.ToString();
    }
}

