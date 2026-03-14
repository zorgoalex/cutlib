internal static class SampleRequests
{
    public static LibCutRequest CreateValidRequest()
    {
        return new LibCutRequest
        {
            Sheet = new LibCutSheetRequest
            {
                LengthMm = 2440,
                WidthMm = 1220,
            },
            Blade = 4,
            Padding = 10,
            Algorithm = "optimal",
            Parts = new List<LibCutPartRequest>
            {
                new() { LengthMm = 800, WidthMm = 400, Quantity = 5, CanRotate = true, Name = "Panel A" },
                new() { LengthMm = 600, WidthMm = 300, Quantity = 8, CanRotate = true, Name = "Panel B" },
                new() { LengthMm = 500, WidthMm = 250, Quantity = 4, CanRotate = false, Name = "Shelf" },
                new() { LengthMm = 1200, WidthMm = 600, Quantity = 2, CanRotate = true, Name = "Door" },
            },
        };
    }
}
