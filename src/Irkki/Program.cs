namespace Irkki;

class Program
{
    static async Task<int> Main(string[] args)
    {
        int exitCode = 1;
        try
        {
            var app = new App();
            exitCode = await app.RunAsync();
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"An error occurred: {ex.Message}");
        }
        
        return exitCode;
    }
}
