public sealed class LibCutRequestValidatorTests
{
    [Fact]
    public void Validate_AllowsValidRequest()
    {
        var request = SampleRequests.CreateValidRequest();

        LibCutRequestValidator.Validate(request);
    }

    [Fact]
    public void Validate_ReportsFieldLevelErrors()
    {
        var request = new LibCutRequest
        {
            Sheet = new LibCutSheetRequest
            {
                LengthMm = 0,
                WidthMm = -5,
            },
            Algorithm = "broken",
            Parts = new List<LibCutPartRequest>
            {
                new()
                {
                    LengthMm = 0,
                    WidthMm = 100,
                    Quantity = 0,
                },
            },
        };

        var exception = Assert.Throws<LibCutValidationException>(() => LibCutRequestValidator.Validate(request));
        var errors = exception.ToErrorDictionary();

        Assert.Equal("Request validation failed.", exception.Message);
        Assert.Contains("sheet.length", errors.Keys);
        Assert.Contains("sheet.width", errors.Keys);
        Assert.Contains("parts[0].length", errors.Keys);
        Assert.Contains("parts[0].qty", errors.Keys);
        Assert.Contains("algorithm", errors.Keys);
        Assert.Contains("Allowed values: length, width, optimal.", errors["algorithm"][0]);
    }
}
