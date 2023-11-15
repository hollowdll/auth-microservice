using System.Security.Claims;

namespace AuthService.Services;

public interface IJwtService
{
    public string GenerateToken(ClaimsIdentity claimsIdentity);
}