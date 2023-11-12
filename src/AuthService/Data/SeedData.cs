namespace AuthService.Data;

public static class SeedData
{
    // Initialize database and create it if it doesn't exist.
    public static void InitializeDatabase(ApplicationDbContext context)
    {
        context.Database.EnsureCreated();
    }
}