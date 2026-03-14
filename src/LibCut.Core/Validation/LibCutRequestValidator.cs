public static class LibCutRequestValidator
{
    public static void Validate(LibCutRequest request)
    {
        var issues = new List<LibCutValidationIssue>();

        if (request.Sheet == null)
        {
            issues.Add(new LibCutValidationIssue("sheet", "Sheet is required."));
        }
        else
        {
            if (request.Sheet.LengthMm <= 0)
                issues.Add(new LibCutValidationIssue("sheet.length", "Sheet length must be greater than zero."));
            if (request.Sheet.WidthMm <= 0)
                issues.Add(new LibCutValidationIssue("sheet.width", "Sheet width must be greater than zero."));
        }

        if (request.Parts == null || request.Parts.Count == 0)
        {
            issues.Add(new LibCutValidationIssue("parts", "At least one part is required."));
        }
        else
        {
            for (int i = 0; i < request.Parts.Count; i++)
            {
                var part = request.Parts[i];
                if (part.LengthMm <= 0)
                    issues.Add(new LibCutValidationIssue($"parts[{i}].length", $"Part #{i + 1} length must be greater than zero."));
                if (part.WidthMm <= 0)
                    issues.Add(new LibCutValidationIssue($"parts[{i}].width", $"Part #{i + 1} width must be greater than zero."));
                if (part.Quantity <= 0)
                    issues.Add(new LibCutValidationIssue($"parts[{i}].qty", $"Part #{i + 1} quantity must be greater than zero."));
            }
        }

        ValidateNonNegative(request.Options?.BladeMm, "options.blade", "Blade must be zero or greater.", issues);
        ValidateNonNegative(request.Options?.PaddingMm, "options.padding", "Padding must be zero or greater.", issues);
        ValidateNonNegative(request.Blade, "blade", "Blade must be zero or greater.", issues);
        ValidateNonNegative(request.Padding, "padding", "Padding must be zero or greater.", issues);
        ValidateAlgorithm(request.Options?.Algorithm, "options.algorithm", issues);
        ValidateAlgorithm(request.Algorithm, "algorithm", issues);

        if (issues.Count > 0)
            throw new LibCutValidationException("Request validation failed.", issues);
    }

    private static void ValidateNonNegative(int? value, string path, string message, List<LibCutValidationIssue> issues)
    {
        if (value is < 0)
            issues.Add(new LibCutValidationIssue(path, message));
    }

    private static void ValidateAlgorithm(string? value, string path, List<LibCutValidationIssue> issues)
    {
        if (string.IsNullOrWhiteSpace(value))
            return;

        try
        {
            LibCutAlgorithmParser.Parse(value);
        }
        catch (LibCutValidationException ex)
        {
            string message = ex.Issues.FirstOrDefault()?.Message ?? ex.Message;
            issues.Add(new LibCutValidationIssue(path, message));
        }
    }
}
