using Microsoft.EntityFrameworkCore;

namespace AuthService.Data;

public static class SeedData
{
    // Initialize database and create it if it doesn't exist.
    public static async Task InitializeDatabase(ApplicationDbContext context)
    {
        await context.Database.MigrateAsync();
    }
}