using Npgsql;

namespace AuthService.Data;

internal static class DbConnection
{
    // Tries to connect to the database.
    // Exits if multiple attempts failed.
    public static void CheckDatabaseConnection(string connectionString)
    {
        var isConnected = false;
        var maxAttempts = 5;
        var attemptCount = 0;
        var retryIntervalMilliseconds = 5000;

        while (!isConnected && attemptCount < maxAttempts)
        {
            try
            {
                using NpgsqlConnection connection = new(connectionString);
                connection.Open();
                isConnected = true;
                Console.WriteLine("Connected to PostgreSQL!");
            }
            catch (NpgsqlException)
            {
                attemptCount++;
                Console.WriteLine($"Database connection attempt {attemptCount}/{maxAttempts} failed");
                Thread.Sleep(retryIntervalMilliseconds);
            }
        }

        if (!isConnected)
        {
            Console.WriteLine("Failed to connect to PostgreSQL after multiple attempts. Exiting...");
            Environment.Exit(1);
        }
    }
}