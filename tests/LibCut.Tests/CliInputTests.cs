public sealed class CsvOrderReaderTests
{
    [Fact]
    public void Read_SkipsHeaderAndParsesRows()
    {
        string path = TemporaryFiles.Create(
            """
            length;width;qty;rotate;name
            800;400;2;1;Panel A
            500;250;1;0;Shelf
            """,
            ".csv");

        try
        {
            var request = CsvOrderReader.Read(path);

            Assert.Equal(2, request.Parts.Count);
            Assert.Equal(800, request.Parts[0].LengthMm);
            Assert.Equal(400, request.Parts[0].WidthMm);
            Assert.Equal(2, request.Parts[0].Quantity);
            Assert.True(request.Parts[0].CanRotate);
            Assert.Equal("Panel A", request.Parts[0].Name);
            Assert.False(request.Parts[1].CanRotate);
        }
        finally
        {
            TemporaryFiles.Delete(path);
        }
    }

    [Fact]
    public void Read_InvalidRow_ThrowsClearValidationError()
    {
        string path = TemporaryFiles.Create(
            """
            length;width;qty;rotate;name
            800;bad;2;1;Panel A
            """,
            ".csv");

        try
        {
            var exception = Assert.Throws<LibCutValidationException>(() => CsvOrderReader.Read(path));

            Assert.Contains("csv.line.2", exception.ToErrorDictionary().Keys);
            Assert.Contains("Invalid CSV row at line 2", exception.Message);
        }
        finally
        {
            TemporaryFiles.Delete(path);
        }
    }
}

public sealed class JsonOrderReaderTests
{
    [Fact]
    public void Read_InitializesMissingCollections()
    {
        string path = TemporaryFiles.Create(
            """
            {
              "sheet": { "length": 2440, "width": 1220 }
            }
            """,
            ".json");

        try
        {
            var request = JsonOrderReader.Read(path);

            Assert.NotNull(request.Sheet);
            Assert.NotNull(request.Parts);
            Assert.NotNull(request.Options);
            Assert.Empty(request.Parts);
        }
        finally
        {
            TemporaryFiles.Delete(path);
        }
    }

    [Fact]
    public void Read_InvalidJson_ThrowsClearValidationError()
    {
        string path = TemporaryFiles.Create("{", ".json");

        try
        {
            var exception = Assert.Throws<LibCutValidationException>(() => JsonOrderReader.Read(path));

            Assert.Contains("json", exception.ToErrorDictionary().Keys);
            Assert.Equal("Input JSON is invalid.", exception.Message);
        }
        finally
        {
            TemporaryFiles.Delete(path);
        }
    }
}
