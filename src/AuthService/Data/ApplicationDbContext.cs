using AuthService.Models;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Identity.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;

namespace AuthService.Data;

public class ApplicationDbContext : IdentityDbContext<AppUser>
{
    public ApplicationDbContext(DbContextOptions<ApplicationDbContext> options)
        : base(options)
    {
    }

    protected override void OnModelCreating(ModelBuilder builder)
    {
        base.OnModelCreating(builder);
        
        // Customize the ASP.NET Identity model and override the defaults if needed.
        // For example, you can rename the ASP.NET Identity table names and more.
        // Add your customizations after calling base.OnModelCreating(builder);

        builder.Entity<AppUser>(b =>
        {
            b.ToTable("users");
        });

        builder.Entity<IdentityRole>(b =>
        {
            b.ToTable("roles");
        });

        builder.Entity<IdentityRoleClaim<string>>(b =>
        {
            b.ToTable("role_claims");
        });

        builder.Entity<IdentityUserClaim<string>>(b =>
        {
            b.ToTable("user_claims");
        });

        builder.Entity<IdentityUserLogin<string>>(b =>
        {
            b.ToTable("user_logins");
        });

        builder.Entity<IdentityUserRole<string>>(b =>
        {
            b.ToTable("user_roles");
        });

        builder.Entity<IdentityUserToken<string>>(b =>
        {
            b.ToTable("user_tokens");
        });
    }
}