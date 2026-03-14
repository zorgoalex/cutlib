internal static class Program
{
    private static int Main(string[] args)
    {
        try
        {
            var cliOptions = CliOptionsParser.Parse(args);
            if (cliOptions.ShowHelp)
            {
                Console.WriteLine(CliOptionsParser.Usage);
                return 0;
            }

            var request = LoadRequest(cliOptions.InputFile!);
            ApplyOverrides(request, cliOptions);

            var engine = new LibCutEngine();
            var result = engine.Optimize(request);
            var resolvedOptions = request.ResolveOptions();

            Console.Error.WriteLine($"Sheet: {result.SheetSize.LengthMm}x{result.SheetSize.WidthMm} mm");
            Console.Error.WriteLine($"Blade: {resolvedOptions.BladeMm} mm, Padding: {resolvedOptions.PaddingMm} mm");
            Console.Error.WriteLine($"Algorithm: {resolvedOptions.Algorithm}");
            Console.Error.WriteLine($"Parts: {request.Parts.Count} types, {request.Parts.Sum(part => part.Quantity)} total pieces");

            string output = cliOptions.Format == "json"
                ? JsonResultFormatter.Format(result)
                : TextResultFormatter.Format(request, result);

            if (!string.IsNullOrWhiteSpace(cliOptions.OutputFile))
            {
                File.WriteAllText(cliOptions.OutputFile!, output);
                Console.Error.WriteLine($"Results written to {cliOptions.OutputFile}");
            }
            else
            {
                Console.WriteLine(output);
            }

            return 0;
        }
        catch (ArgumentException ex)
        {
            Console.Error.WriteLine($"Error: {ex.Message}");
            Console.WriteLine(CliOptionsParser.Usage);
            return 1;
        }
        catch (LibCutValidationException ex)
        {
            Console.Error.WriteLine($"Error: {ex.Message}");
            foreach (var issue in ex.Issues)
            {
                if (string.Equals(issue.Message, ex.Message, StringComparison.Ordinal) && issue.Path == "request")
                    continue;

                Console.Error.WriteLine($"  - {issue.Path}: {issue.Message}");
            }
            return 1;
        }
        catch (FileNotFoundException ex)
        {
            Console.Error.WriteLine($"Error: input file not found: {ex.FileName ?? ex.Message}");
            return 1;
        }
    }

    private static LibCutRequest LoadRequest(string inputFile)
    {
        string extension = Path.GetExtension(inputFile).ToLowerInvariant();
        return extension switch
        {
            ".json" => JsonOrderReader.Read(inputFile),
            ".csv" => CsvOrderReader.Read(inputFile),
            _ => throw new ArgumentException($"Unsupported input format '{extension}'. Use CSV or JSON."),
        };
    }

    private static void ApplyOverrides(LibCutRequest request, CliOptions cliOptions)
    {
        request.Sheet ??= new LibCutSheetRequest();
        request.Options ??= new LibCutOptions();

        if (cliOptions.SheetLengthMm.HasValue && cliOptions.SheetWidthMm.HasValue)
        {
            request.Sheet.LengthMm = cliOptions.SheetLengthMm.Value;
            request.Sheet.WidthMm = cliOptions.SheetWidthMm.Value;
        }

        if (cliOptions.BladeMm.HasValue)
            request.Blade = cliOptions.BladeMm.Value;

        if (cliOptions.PaddingMm.HasValue)
            request.Padding = cliOptions.PaddingMm.Value;

        if (!string.IsNullOrWhiteSpace(cliOptions.Algorithm))
            request.Algorithm = cliOptions.Algorithm;
    }
}
