public sealed class CliOptions
{
    public bool ShowHelp { get; set; }

    public string? InputFile { get; set; }

    public string? OutputFile { get; set; }

    public string Format { get; set; } = "text";

    public int? SheetLengthMm { get; set; }

    public int? SheetWidthMm { get; set; }

    public int? BladeMm { get; set; }

    public int? PaddingMm { get; set; }

    public string? Algorithm { get; set; }
}

