public static class OrderFactory
{
    public static Order Create(LibCutRequest request)
    {
        var resolved = request.ResolveOptions();
        var order = new Order();

        order.parameters.Blade = resolved.BladeMm;
        order.parameters.Padding = resolved.PaddingMm;
        order.parameters.Algoritm = (int)resolved.Algorithm;
        order.parameters.ListLength_mm = request.Sheet!.LengthMm;
        order.parameters.ListWidth_mm = request.Sheet.WidthMm;
        order.sheet = new Sheet
        {
            Length = request.Sheet.LengthMm,
            Width = request.Sheet.WidthMm,
        };

        for (int i = 0; i < request.Parts.Count; i++)
        {
            var inputPart = request.Parts[i];
            var part = new Part
            {
                Length_mm = inputPart.LengthMm,
                Width_mm = inputPart.WidthMm,
                Amount = inputPart.Quantity,
                Sq = (long)inputPart.LengthMm * inputPart.WidthMm,
                Turn = inputPart.CanRotate,
                Name = inputPart.Name ?? "",
                Npart = i,
            };

            for (int c = 0; c < part.Amount; c++)
                part.Coords.Add(new Coord());

            order.Parts.Add(part);
            order.PartsSq += part.Sq * part.Amount;
        }

        return order;
    }
}
