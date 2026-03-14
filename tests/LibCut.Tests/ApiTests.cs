using System.Net;
using System.Net.Http.Json;
using System.Text;
using System.Text.Json;
using Microsoft.AspNetCore.Mvc.Testing;
using Microsoft.AspNetCore.TestHost;
using Microsoft.Extensions.Configuration;

public sealed class LibCutApiFactory : WebApplicationFactory<Program>
{
    protected override void ConfigureWebHost(IWebHostBuilder builder)
    {
        builder.UseEnvironment("Development");
        builder.ConfigureAppConfiguration((_, configBuilder) =>
        {
            configBuilder.AddInMemoryCollection(new Dictionary<string, string?>
            {
                ["LibCut:MaxConcurrentOptimizations"] = "1",
            });
        });
    }
}

public sealed class ApiTests : IClassFixture<LibCutApiFactory>
{
    private readonly HttpClient _client;

    public ApiTests(LibCutApiFactory factory)
    {
        _client = factory.CreateClient();
    }

    [Fact]
    public async Task Health_ReturnsOk()
    {
        var response = await _client.GetAsync("/health");

        response.EnsureSuccessStatusCode();

        var payload = await response.Content.ReadFromJsonAsync<HealthResponse>();
        Assert.NotNull(payload);
        Assert.Equal("ok", payload.Status);
        Assert.Equal("LibCut.Api", payload.Service);
    }

    [Fact]
    public async Task OpenApi_ReturnsDocumentWithOptimizeEndpoint()
    {
        var response = await _client.GetAsync("/openapi/v1.json");

        response.EnsureSuccessStatusCode();

        using var document = JsonDocument.Parse(await response.Content.ReadAsStringAsync());
        var paths = document.RootElement.GetProperty("paths");
        Assert.True(paths.TryGetProperty("/api/cut/optimize", out _));
        Assert.True(paths.TryGetProperty("/health", out _));
    }

    [Fact]
    public async Task Optimize_ReturnsResultForValidRequest()
    {
        var response = await _client.PostAsJsonAsync("/api/cut/optimize", SampleRequests.CreateValidRequest());

        response.EnsureSuccessStatusCode();

        var result = await response.Content.ReadFromJsonAsync<LibCutResult>();
        Assert.NotNull(result);
        Assert.Equal(2, result.SheetsUsed);
        Assert.Equal(19, result.PartsPlaced);
        Assert.Equal(19, result.PartsTotal);
    }

    [Fact]
    public async Task Optimize_ReturnsValidationProblemForInvalidRequest()
    {
        var request = new LibCutRequest
        {
            Sheet = new LibCutSheetRequest
            {
                LengthMm = 0,
                WidthMm = 1220,
            },
            Parts = new List<LibCutPartRequest>(),
        };

        var response = await _client.PostAsJsonAsync("/api/cut/optimize", request);

        Assert.Equal(HttpStatusCode.BadRequest, response.StatusCode);
        Assert.Equal("application/problem+json", response.Content.Headers.ContentType?.MediaType);

        using var document = JsonDocument.Parse(await response.Content.ReadAsStringAsync());
        Assert.Equal("Invalid cut optimization request.", document.RootElement.GetProperty("title").GetString());
        Assert.Contains("traceId", document.RootElement.EnumerateObject().Select(property => property.Name));

        var errors = document.RootElement.GetProperty("errors");
        Assert.True(errors.TryGetProperty("sheet.length", out _));
        Assert.True(errors.TryGetProperty("parts", out _));
    }

    [Fact]
    public async Task Optimize_ReturnsValidationProblemForMalformedJson()
    {
        using var content = new StringContent("{\"sheet\":", Encoding.UTF8, "application/json");

        var response = await _client.PostAsync("/api/cut/optimize", content);

        Assert.Equal(HttpStatusCode.BadRequest, response.StatusCode);

        using var document = JsonDocument.Parse(await response.Content.ReadAsStringAsync());
        var errors = document.RootElement.GetProperty("errors");
        Assert.True(errors.TryGetProperty("json", out var jsonErrors));
        Assert.Contains("Malformed JSON", jsonErrors[0].GetString());
    }
}
