public sealed class LibCutEngineTests
{
    [Fact]
    public void Optimize_ReturnsExpectedSummaryForSampleRequest()
    {
        var engine = new LibCutEngine();

        var result = engine.Optimize(SampleRequests.CreateValidRequest());

        Assert.Equal(2, result.SheetsUsed);
        Assert.Equal(19, result.PartsPlaced);
        Assert.Equal(19, result.PartsTotal);
        Assert.Equal(83.6, result.EfficiencyPercent, 1);
        Assert.Equal(result.SheetsUsed, result.Sheets.Count);
    }
}
