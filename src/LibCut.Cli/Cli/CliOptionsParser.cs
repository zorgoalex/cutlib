public static class CliOptionsParser
{
    public static string Usage =>
@"LibCut CLI - 2D sheet cutting optimizer

Usage: LibCutCLI -i <input> [options]

Options:
  -i, --input <file>       Input CSV/JSON file with parts list
  -o, --output <file>      Output file (default: stdout)
  -s, --sheet <LxW>        Sheet size in mm, e.g. 2440x1220
  -b, --blade <mm>         Blade/kerf width in mm (default: from input or 4)
  -p, --padding <mm>       Edge padding in mm (default: from input or 0)
  -a, --algorithm <alg>    Algorithm: length|width|optimal
  -f, --format <fmt>       Output format: text|json (default: text)
  -h, --help               Show usage info and exit";

    public static CliOptions Parse(string[] args)
    {
        if (args.Length == 0)
            return new CliOptions { ShowHelp = true };

        var options = new CliOptions();

        for (int i = 0; i < args.Length; i++)
        {
            switch (args[i])
            {
                case "-h":
                case "--help":
                    options.ShowHelp = true;
                    return options;
                case "-i":
                case "--input":
                    options.InputFile = GetValue(args, ref i, "input");
                    break;
                case "-o":
                case "--output":
                    options.OutputFile = GetValue(args, ref i, "output");
                    break;
                case "-a":
                case "--algorithm":
                    options.Algorithm = GetValue(args, ref i, "algorithm");
                    break;
                case "-s":
                case "--sheet":
                    ParseSheet(GetValue(args, ref i, "sheet"), options);
                    break;
                case "-b":
                case "--blade":
                    options.BladeMm = ParseInt(GetValue(args, ref i, "blade"), "blade");
                    break;
                case "-p":
                case "--padding":
                    options.PaddingMm = ParseInt(GetValue(args, ref i, "padding"), "padding");
                    break;
                case "-f":
                case "--format":
                    options.Format = GetValue(args, ref i, "format").Trim().ToLowerInvariant();
                    if (options.Format != "text" && options.Format != "json")
                        throw new ArgumentException($"Unsupported format '{options.Format}'.");
                    break;
                default:
                    throw new ArgumentException($"Unknown option '{args[i]}'.");
            }
        }

        if (string.IsNullOrWhiteSpace(options.InputFile))
            throw new ArgumentException("Input file is required (-i).");

        return options;
    }

    private static string GetValue(string[] args, ref int index, string optionName)
    {
        if (index + 1 >= args.Length)
            throw new ArgumentException($"Option '{optionName}' requires a value.");

        index++;
        return args[index];
    }

    private static int ParseInt(string value, string optionName)
    {
        if (!int.TryParse(value, out var parsed))
            throw new ArgumentException($"Option '{optionName}' requires an integer value.");

        return parsed;
    }

    private static void ParseSheet(string value, CliOptions options)
    {
        var dims = value.Split('x', 'X', '*');
        if (dims.Length != 2 || !int.TryParse(dims[0], out var length) || !int.TryParse(dims[1], out var width))
            throw new ArgumentException("Sheet size must be in LxW format, for example 2440x1220.");

        options.SheetLengthMm = length;
        options.SheetWidthMm = width;
    }
}

