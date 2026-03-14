internal static class TemporaryFiles
{
    public static string Create(string content, string extension)
    {
        string path = Path.Combine(Path.GetTempPath(), $"{Guid.NewGuid():N}{extension}");
        File.WriteAllText(path, content);
        return path;
    }

    public static void Delete(string path)
    {
        if (File.Exists(path))
            File.Delete(path);
    }
}
