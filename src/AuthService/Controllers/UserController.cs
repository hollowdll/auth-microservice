using System.Net;
using System.Security.Authentication;
using AuthService.Data;
using AuthService.Models;
using AuthService.Services;
using AuthService.Utility;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Authorization;

namespace AuthService.Controllers;

[ApiController]
[Route("api/v1/users")]
[Authorize]
public class UserController : ControllerBase
{
    private readonly UserManager<AppUser> _userManager;

    public UserController(UserManager<AppUser> userManager)
    {
        _userManager = userManager;
    }

    [HttpGet]
    [Authorize(Roles = "Admin")]
    public async Task<ActionResult<List<UserData>>> GetUsers()
    {
        var users = await _userManager.Users.ToListAsync();
        var usersResponse = new List<UserData>();

        foreach (var user in users)
        {
            var roles = await _userManager.GetRolesAsync(user);
            usersResponse.Add(MapToUserData(user, roles));
        }

        return Ok(usersResponse);
    }

    private static UserData MapToUserData(AppUser appUser, IList<string> roles)
    {
        return new UserData
        {
            Id = appUser.Id,
            Username = appUser.UserName,
            Roles = roles,
        };
    }
}