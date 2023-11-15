using Microsoft.AspNetCore.Identity;

namespace AuthService.Models;

public class AppUser : IdentityUser
{
    public DateTime CreatedAt { get; set; }
    public DateTime UpdatedAt { get; set; }
}