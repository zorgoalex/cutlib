using System.Text.Json.Serialization;

public enum LibCutAlgorithm
{
    Length = 1,
    Width = 2,
    Optimal = 3,
}

public sealed class LibCutValidationIssue
{
    public LibCutValidationIssue(string path, string message)
    {
        Path = string.IsNullOrWhiteSpace(path) ? "request" : path;
        Message = message;
    }

    public string Path { get; }

    public string Message { get; }
}

public sealed class LibCutValidationException : Exception
{
    public IReadOnlyList<string> Errors { get; }

    public IReadOnlyList<LibCutValidationIssue> Issues { get; }

    public LibCutValidationException(string message)
        : this(message, new[] { new LibCutValidationIssue("request", message) })
    {
    }

    public LibCutValidationException(string message, IReadOnlyList<string> errors)
        : this(message, errors.Select(error => new LibCutValidationIssue("request", error)).ToArray())
    {
    }

    public LibCutValidationException(string message, IReadOnlyList<LibCutValidationIssue> issues)
        : base(message)
    {
        Issues = issues;
        Errors = issues.Select(issue => issue.Message).ToArray();
    }

    public IReadOnlyDictionary<string, string[]> ToErrorDictionary()
    {
        return Issues
            .GroupBy(issue => issue.Path)
            .ToDictionary(group => group.Key, group => group.Select(issue => issue.Message).Distinct().ToArray());
    }
}

public sealed class LibCutSheetRequest
{
    [JsonPropertyName("length")]
    public int LengthMm { get; set; }

    [JsonPropertyName("width")]
    public int WidthMm { get; set; }
}

public sealed class LibCutPartRequest
{
    [JsonPropertyName("name")]
    public string Name { get; set; } = "";

    [JsonPropertyName("length")]
    public int LengthMm { get; set; }

    [JsonPropertyName("width")]
    public int WidthMm { get; set; }

    [JsonPropertyName("qty")]
    public int Quantity { get; set; } = 1;

    [JsonPropertyName("rotate")]
    public bool CanRotate { get; set; } = true;
}

public sealed class LibCutOptions
{
    [JsonPropertyName("blade")]
    public int? BladeMm { get; set; }

    [JsonPropertyName("padding")]
    public int? PaddingMm { get; set; }

    [JsonPropertyName("algorithm")]
    public string? Algorithm { get; set; }
}

public sealed class LibCutResolvedOptions
{
    public int BladeMm { get; set; } = 4;

    public int PaddingMm { get; set; }

    public LibCutAlgorithm Algorithm { get; set; } = LibCutAlgorithm.Optimal;
}

public sealed class LibCutRequest
{
    [JsonPropertyName("sheet")]
    public LibCutSheetRequest? Sheet { get; set; } = new();

    [JsonPropertyName("parts")]
    public List<LibCutPartRequest> Parts { get; set; } = new();

    [JsonPropertyName("blade")]
    public int? Blade { get; set; }

    [JsonPropertyName("padding")]
    public int? Padding { get; set; }

    [JsonPropertyName("algorithm")]
    public string? Algorithm { get; set; }

    [JsonPropertyName("options")]
    public LibCutOptions? Options { get; set; } = new();

    public LibCutResolvedOptions ResolveOptions()
    {
        var resolved = new LibCutResolvedOptions();

        if (Options?.BladeMm is int optionsBlade)
            resolved.BladeMm = optionsBlade;
        if (Options?.PaddingMm is int optionsPadding)
            resolved.PaddingMm = optionsPadding;
        if (!string.IsNullOrWhiteSpace(Options?.Algorithm))
            resolved.Algorithm = LibCutAlgorithmParser.Parse(Options.Algorithm);

        if (Blade is int blade)
            resolved.BladeMm = blade;
        if (Padding is int padding)
            resolved.PaddingMm = padding;
        if (!string.IsNullOrWhiteSpace(Algorithm))
            resolved.Algorithm = LibCutAlgorithmParser.Parse(Algorithm);

        return resolved;
    }
}

public sealed class LibCutPartPlacement
{
    public string Name { get; set; } = "";

    public int Length { get; set; }

    public int Width { get; set; }

    public int X { get; set; }

    public int Y { get; set; }

    public bool Rotated { get; set; }
}

public sealed class LibCutOffcut
{
    public int Length { get; set; }

    public int Width { get; set; }

    public int X { get; set; }

    public int Y { get; set; }
}

public sealed class LibCutSheetResult
{
    public int Sheet { get; set; }

    public List<LibCutPartPlacement> Parts { get; set; } = new();

    public List<LibCutOffcut> Offcuts { get; set; } = new();
}

public sealed class LibCutResult
{
    public LibCutSheetRequest SheetSize { get; set; } = new();

    public int SheetsUsed { get; set; }

    public int PartsPlaced { get; set; }

    public int PartsTotal { get; set; }

    public double EfficiencyPercent { get; set; }

    public List<LibCutSheetResult> Sheets { get; set; } = new();
}

public static class LibCutAlgorithmParser
{
    public static LibCutAlgorithm Parse(string? value)
    {
        if (string.IsNullOrWhiteSpace(value))
            return LibCutAlgorithm.Optimal;

        return value.Trim().ToLowerInvariant() switch
        {
            "length" or "l" or "1" => LibCutAlgorithm.Length,
            "width" or "w" or "2" => LibCutAlgorithm.Width,
            "optimal" or "opt" or "3" => LibCutAlgorithm.Optimal,
            _ => throw new LibCutValidationException(
                $"Unsupported algorithm value '{value}'. Allowed values: length, width, optimal.",
                new[] { new LibCutValidationIssue("algorithm", $"Unsupported algorithm value '{value}'. Allowed values: length, width, optimal.") }),
        };
    }
}
