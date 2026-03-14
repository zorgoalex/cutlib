using System.Text.Json;

public static class JsonOrderReader
{
    private static readonly JsonSerializerOptions Options = new()
    {
        PropertyNameCaseInsensitive = true,
    };

    public static LibCutRequest Read(string path)
    {
        try
        {
            var request = JsonSerializer.Deserialize<LibCutRequest>(File.ReadAllText(path), Options);
            if (request == null)
            {
                throw new LibCutValidationException(
                    "Input JSON produced an empty request.",
                    new[] { new LibCutValidationIssue("json", "Input JSON produced an empty request.") });
            }

            request.Sheet ??= new LibCutSheetRequest();
            request.Parts ??= new List<LibCutPartRequest>();
            request.Options ??= new LibCutOptions();

            return request;
        }
        catch (JsonException ex)
        {
            throw new LibCutValidationException(
                "Input JSON is invalid.",
                new[] { new LibCutValidationIssue("json", ex.Message) });
        }
    }
}
