using System.Runtime.CompilerServices;
using AuthService.Models;
using AuthService.Config;
using Microsoft.AspNetCore.Identity;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Options;

namespace AuthService.Data;

public static class SeedData
{
    // Create database and initial data.
    public static async Task InitializeDatabase(IServiceScope scope, IWebHostEnvironment environment, ILogger logger)
    {
        var context = scope.ServiceProvider.GetRequiredService<ApplicationDbContext>();
        var userManager = scope.ServiceProvider.GetRequiredService<UserManager<AppUser>>();
        var roleManager = scope.ServiceProvider.GetRequiredService<RoleManager<IdentityRole>>();
        var config = scope.ServiceProvider.GetRequiredService<IOptions<AppConfig>>();

        // Reset database in dev mode.
        if (environment.IsDevelopment())
        {
            await context.Database.EnsureDeletedAsync();
        }

        await context.Database.MigrateAsync();

        // Create roles
        foreach (var roleName in AppRole.GetRoleNames())
        {
            var roleExist = await roleManager.RoleExistsAsync(roleName);
            if (!roleExist)
            {
                await roleManager.CreateAsync(new IdentityRole(roleName));
            }
        }

        // Create users.
        var adminUser = await userManager.FindByNameAsync(config.Value.AdminUsername);
        if (adminUser == null)
        {
            adminUser = new AppUser
            {
                UserName = config.Value.AdminUsername
            };

            var result = await userManager.CreateAsync(adminUser, config.Value.AdminPassword);
            if (!result.Succeeded)
            {
                throw new Exception(result.Errors.First().Description);
            }

            var createdAdminUser = await userManager.FindByNameAsync(config.Value.AdminUsername)
                ?? throw new Exception("Created user was not found");

            // Admin user has all roles
            await userManager.AddToRolesAsync(createdAdminUser, AppRole.GetRoleNames());
        }
    }
}