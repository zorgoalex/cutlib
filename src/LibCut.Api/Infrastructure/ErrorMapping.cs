using System.Text.Json;
using Microsoft.AspNetCore.Mvc;

public static class ErrorMapping
{
    public static IResult ValidationProblem(HttpContext httpContext, LibCutValidationException exception)
    {
        return Results.ValidationProblem(
            errors: exception.ToErrorDictionary().ToDictionary(pair => pair.Key, pair => pair.Value),
            detail: "Correct the fields listed in the errors section and retry the request.",
            title: "Invalid cut optimization request.",
            type: "https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1",
            statusCode: StatusCodes.Status400BadRequest,
            extensions: CreateExtensions(httpContext));
    }

    public static IResult InvalidJsonProblem(HttpContext httpContext, JsonException exception)
    {
        string path = string.IsNullOrWhiteSpace(exception.Path) ? "$" : exception.Path;
        string location = exception.LineNumber.HasValue && exception.BytePositionInLine.HasValue
            ? $" Line {exception.LineNumber.Value}, byte {exception.BytePositionInLine.Value}."
            : string.Empty;

        var validationException = new LibCutValidationException(
            "Request body contains invalid JSON.",
            new[]
            {
                new LibCutValidationIssue("json", $"Malformed JSON at path '{path}'.{location}".Trim()),
            });

        return ValidationProblem(httpContext, validationException);
    }

    public static IResult UnexpectedFailure(HttpContext httpContext, IHostEnvironment environment, Exception exception)
    {
        var extensions = CreateExtensions(httpContext);
        if (environment.IsDevelopment())
            extensions["exception"] = exception.Message;

        return Results.Problem(
            title: "Cut optimization failed.",
            detail: "Unexpected server error while processing the optimization request.",
            statusCode: StatusCodes.Status500InternalServerError,
            type: "https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.1",
            extensions: extensions);
    }

    private static Dictionary<string, object?> CreateExtensions(HttpContext httpContext)
    {
        return new Dictionary<string, object?>
        {
            ["traceId"] = httpContext.TraceIdentifier,
        };
    }
}
