using Grpc.Core;
using Microsoft.AspNetCore.Identity;
using AuthService.Models;
using Microsoft.EntityFrameworkCore;
using Microsoft.AspNetCore.Authorization;

namespace GrpcAuth;

[Authorize]
public class UserService : User.UserBase
{
    private readonly ILogger<UserService> _logger;
    private readonly UserManager<AppUser> _userManager;

    public UserService(ILogger<UserService> logger, UserManager<AppUser> userManager)
    {
        _logger = logger;
        _userManager = userManager;
    }

    // Get users from database and return them.
    public override async Task<GetUsersResponse> GetUsers(GetUsersRequest request, ServerCallContext context)
    {
        _logger.LogInformation("gRPC call method {Method}", context.Method);

        var response = new GetUsersResponse();
        var users = await _userManager.Users.ToListAsync();

        foreach (var user in users)
        {
            var roles = await _userManager.GetRolesAsync(user);
            response.Users.Add(MapToUserData(user, roles));
        }

        context.Status = new Status(StatusCode.OK, "Get users success");

        return response;
    }

    private static UserData MapToUserData(AppUser appUser, IList<string> roles)
    {
        var userData = new UserData
        {
            Id = appUser.Id,
            Username = appUser.UserName,
        };
        
        foreach (var role in roles)
        {
            userData.Roles.Add(role);
        }

        return userData;
    }
}