using System.Text.Json;

public static class JsonResultFormatter
{
    private static readonly JsonSerializerOptions Options = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        WriteIndented = true,
    };

    public static string Format(LibCutResult result)
    {
        return JsonSerializer.Serialize(result, Options);
    }
}

