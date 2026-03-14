using System.Text.Json;
using Microsoft.AspNetCore.OpenApi;
using Microsoft.AspNetCore.Mvc;
using Microsoft.OpenApi.Models;

var builder = WebApplication.CreateBuilder(args);

builder.Services.ConfigureHttpJsonOptions(options =>
{
    options.SerializerOptions.PropertyNamingPolicy = JsonNamingPolicy.CamelCase;
    options.SerializerOptions.WriteIndented = true;
});
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen(options =>
{
    options.SwaggerDoc("v1", new OpenApiInfo
    {
        Title = "LibCut API",
        Version = "v1",
        Description = "HTTP API for sheet cutting optimization requests.",
    });
});
builder.Services.AddSingleton<LibCutEngine>();
builder.Services.AddSingleton<RequestConcurrencyGate>();

var app = builder.Build();

app.UseSwagger(options =>
{
    options.RouteTemplate = "openapi/{documentName}.json";
});
app.UseSwaggerUI(options =>
{
    options.SwaggerEndpoint("/openapi/v1.json", "LibCut API v1");
    options.RoutePrefix = "swagger";
});

app.MapGet("/health", () => TypedResults.Ok(new HealthResponse()))
    .WithName("LibCutHealth")
    .WithSummary("Check service health.")
    .WithDescription("Returns a lightweight response for uptime probes and smoke checks.")
    .WithTags("System")
    .Produces<HealthResponse>(StatusCodes.Status200OK)
    .WithOpenApi();

app.MapPost("/api/cut/optimize", async (
    HttpContext httpContext,
    LibCutEngine engine,
    RequestConcurrencyGate gate,
    IHostEnvironment environment,
    CancellationToken cancellationToken) =>
{
    LibCutRequest? request;

    try
    {
        request = await httpContext.Request.ReadFromJsonAsync<LibCutRequest>(cancellationToken: cancellationToken);
    }
    catch (JsonException ex)
    {
        return ErrorMapping.InvalidJsonProblem(httpContext, ex);
    }

    if (request == null)
    {
        return ErrorMapping.ValidationProblem(
            httpContext,
            new LibCutValidationException(
                "Request body is required.",
                new[] { new LibCutValidationIssue("json", "Request body is required.") }));
    }

    try
    {
        using var lease = await gate.EnterAsync(cancellationToken);
        var result = engine.Optimize(request);
        return Results.Ok(result);
    }
    catch (LibCutValidationException ex)
    {
        return ErrorMapping.ValidationProblem(httpContext, ex);
    }
    catch (Exception ex)
    {
        return ErrorMapping.UnexpectedFailure(httpContext, environment, ex);
    }
})
    .WithName("OptimizeCutLayout")
    .WithSummary("Optimize part placement on rectangular sheets.")
    .WithDescription("Accepts sheet dimensions, requested parts, and cutting options, then returns sheet layouts and offcuts.")
    .WithTags("Cut")
    .Accepts<LibCutRequest>("application/json")
    .Produces<LibCutResult>(StatusCodes.Status200OK)
    .Produces<HttpValidationProblemDetails>(StatusCodes.Status400BadRequest, "application/problem+json")
    .Produces<ProblemDetails>(StatusCodes.Status500InternalServerError, "application/problem+json")
    .WithOpenApi();

app.Run();

public partial class Program
{
}
