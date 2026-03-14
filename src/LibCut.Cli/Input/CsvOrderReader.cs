public static class CsvOrderReader
{
    public static LibCutRequest Read(string path)
    {
        var request = new LibCutRequest
        {
            Sheet = new LibCutSheetRequest(),
        };

        var lines = File.ReadAllLines(path);
        int lineNumber = 0;
        foreach (var rawLine in lines)
        {
            lineNumber++;
            var line = rawLine.Trim();
            if (line.Length == 0 || line.StartsWith("#"))
                continue;

            var columns = line.Split(';', ',', '\t');
            if (columns.Length < 3)
                continue;

            if (!int.TryParse(columns[0].Trim(), out var length) ||
                !int.TryParse(columns[1].Trim(), out var width) ||
                !int.TryParse(columns[2].Trim(), out var quantity))
            {
                bool looksLikeHeader =
                    columns[0].Trim().Equals("length", StringComparison.OrdinalIgnoreCase) &&
                    columns[1].Trim().Equals("width", StringComparison.OrdinalIgnoreCase) &&
                    columns[2].Trim().Equals("qty", StringComparison.OrdinalIgnoreCase);

                if (looksLikeHeader)
                    continue;

                throw new LibCutValidationException(
                    $"Invalid CSV row at line {lineNumber}: {line}",
                    new[] { new LibCutValidationIssue($"csv.line.{lineNumber}", $"Invalid CSV row at line {lineNumber}: {line}") });
            }

            request.Parts.Add(new LibCutPartRequest
            {
                LengthMm = length,
                WidthMm = width,
                Quantity = quantity,
                CanRotate = columns.Length > 3 && columns[3].Trim() == "1",
                Name = columns.Length > 4 ? columns[4].Trim() : "",
            });
        }

        return request;
    }
}
