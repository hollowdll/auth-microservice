using System.Security.Claims;
using AuthService.Models;

namespace AuthService.Services;

public interface ILoginService
{
    /// <summary>
    /// Validate user login and return ClaimsIdentity representing user's claims.
    /// It can then be used to generate a JWT token.
    /// </summary>
    public Task<ClaimsIdentity> Login(LoginRequest request);
}