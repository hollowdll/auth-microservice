namespace AuthService.Config;

// Application configurations
public class AppConfig
{
    /// <summary>Pepper value used when encrypting passwords.</summary>
    public string PasswordPepper { get; set; } = null!;

    /// <summary>Username for system admin user.</summary>
    public string AdminUsername { get; set; } = null!;

    /// <summary>Password for system admin user.</summary>
    public string AdminPassword { get; set; } = null!;
}