using Microsoft.AspNetCore.Identity;

namespace AuthService.Models;

public class AppUser : IdentityUser
{
    public AppUser()
    {
        CreatedAt = DateTime.UtcNow;
        UpdatedAt = DateTime.UtcNow;
    }

    public AppUser(string username)
    {
        UserName = username;
        CreatedAt = DateTime.UtcNow;
        UpdatedAt = DateTime.UtcNow;
    }

    /// <summary>UTC timestamp when the user is created.</summary>
    public DateTime CreatedAt { get; set; }
    /// <summary>UTC timestamp when the user is updated.</summary>
    public DateTime UpdatedAt { get; set; }
}