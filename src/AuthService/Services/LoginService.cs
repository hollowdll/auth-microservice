using System.Security.Authentication;
using System.Security.Claims;
using AuthService.Config;
using AuthService.Models;
using Microsoft.AspNetCore.Identity;
using Microsoft.Extensions.Options;

namespace AuthService.Services;

public class LoginService : ILoginService
{
    private readonly UserManager<AppUser> _userManager;
    private readonly AppConfig _appConfig;
    private readonly ILogger<LoginService> _logger;

    public LoginService(UserManager<AppUser> userManager, IOptions<AppConfig> appConfig, ILogger<LoginService> logger)
    {
        _userManager = userManager;
        _appConfig = appConfig.Value;
        _logger = logger;
    }

    public async Task<ClaimsIdentity> Login(LoginRequest request)
    {
        var appUser = await _userManager.FindByNameAsync(request.Username)
            ?? throw new AuthenticationException("User not found");

        var passwordValid = await ValidatePassword(appUser, request.Password);
        if (!passwordValid)
        {
            throw new AuthenticationException("Incorrect password");
        }

        var claimsIdentity = await GetUserClaimsIdentity(appUser);
        var roles = await _userManager.GetRolesAsync(appUser);

        // Add user roles to claims
        foreach (var role in roles) {
            claimsIdentity.AddClaim(new Claim(ClaimTypes.Role, role));
        }

        return claimsIdentity;
    }

    // Validates user password.
    // Adds pepper to the password before comparison.
    // Identity system handles comparison securely and in constant time to reduce timing attacks.
    private async Task<bool> ValidatePassword(AppUser appUser, string password)
    {
        return await _userManager.CheckPasswordAsync(appUser, password + _appConfig.PasswordPepper);
    }

    private async Task<ClaimsIdentity> GetUserClaimsIdentity(AppUser appUser)
    {
        var claims = await _userManager.GetClaimsAsync(appUser);
        return new ClaimsIdentity(claims);
    }
}