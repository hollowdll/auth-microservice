namespace AuthService.Config;

public class JwtConfig
{
    /// <summary>Issuer for JWT bearer tokens.</summary>
    public string Issuer { get; set; } = null!;

    /// <summary>Audience for JWT bearer tokens.</summary>
    public string Audience { get; set; } = null!;

    /// <summary>
    /// Signing key for JWT bearer tokens.
    /// Should be long and secure.
    /// </summary>
    public string SigningKey { get; set; } = null!;
}