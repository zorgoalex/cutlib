public sealed class CliOptionsParserTests
{
    [Fact]
    public void Parse_ReadsKnownFlags()
    {
        var options = CliOptionsParser.Parse(new[]
        {
            "-i", "parts.csv",
            "-s", "2800x2070",
            "-b", "7",
            "-p", "10",
            "-a", "optimal",
            "-f", "json",
            "-o", "result.json",
        });

        Assert.Equal("parts.csv", options.InputFile);
        Assert.Equal(2800, options.SheetLengthMm);
        Assert.Equal(2070, options.SheetWidthMm);
        Assert.Equal(7, options.BladeMm);
        Assert.Equal(10, options.PaddingMm);
        Assert.Equal("optimal", options.Algorithm);
        Assert.Equal("json", options.Format);
        Assert.Equal("result.json", options.OutputFile);
    }

    [Fact]
    public void Parse_RejectsUnsupportedFormat()
    {
        var exception = Assert.Throws<ArgumentException>(() => CliOptionsParser.Parse(new[]
        {
            "-i", "parts.csv",
            "-f", "xml",
        }));

        Assert.Contains("Unsupported format", exception.Message);
    }
}
