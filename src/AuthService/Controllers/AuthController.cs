using System.Net;
using System.Security.Authentication;
using AuthService.Models;
using AuthService.Services;
using Microsoft.AspNetCore.Mvc;

namespace AuthService.Controllers;

[ApiController]
[Route("api/v1/auth")]
public class AuthController : ControllerBase
{
    private readonly IJwtService _jwtService;
    private readonly ILoginService _loginService;
    private readonly ILogger<AuthController> _logger;

    public AuthController(IJwtService jwtService, ILoginService loginService, ILogger<AuthController> logger)
    {
        _jwtService = jwtService;
        _loginService = loginService;
        _logger = logger;
    }

    // Login user and return JWT token in response header if successful
    [HttpPost]
    [Route("login")]
    [ProducesResponseType((int)HttpStatusCode.OK)]
    [ProducesResponseType((int)HttpStatusCode.BadRequest)]
    public async Task<ActionResult> Login(LoginRequest request)
    {
        try
        {
            var claimsIdentity = await _loginService.Login(request);
            var jwtToken = _jwtService.GenerateToken(claimsIdentity);

            Response.Headers.Add("Authorization", "Bearer " + jwtToken);

            return Ok("Login successful");
        }
        catch (AuthenticationException e)
        {
            _logger.LogError("Login error: {Error}", e);
            return BadRequest("Incorrect login credentials");
        }
        catch (Exception e)
        {
            _logger.LogError("Login error: {Error}", e);
            return BadRequest("Service error");
        }
    }
}