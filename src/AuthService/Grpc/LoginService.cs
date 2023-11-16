using Grpc.Core;
using AuthService.Services;
using System.Security.Authentication;

namespace GrpcAuth;

public class LoginService : Login.LoginBase
{
    private readonly ILogger<LoginService> _logger;
    private readonly AuthService.Services.ILoginService _loginService;
    private readonly IJwtService _jwtService;

    public LoginService(ILogger<LoginService> logger, AuthService.Services.ILoginService loginService, IJwtService jwtService)
    {
        _logger = logger;
        _loginService = loginService;
        _jwtService = jwtService;
    }

    // Login user and return JWT token if successful.
    public override async Task<LoginResponse> LoginUser(LoginRequest request, ServerCallContext context)
    {
        _logger.LogInformation("gRPC call method {Method}", context.Method);

        try {
            var claimsIdentity = await _loginService.Login(MapFromGrpcLoginRequest(request));
            var jwtToken = _jwtService.GenerateToken(claimsIdentity);

            context.Status = new Status(StatusCode.OK, "Call successful");
            // Include JWT token in gRPC metadata
            context.ResponseTrailers.Add("Authorization", "Bearer " + jwtToken);

            return new LoginResponse { Message = "Login successful" };
        }
        catch (AuthenticationException e)
        {
            _logger.LogError("gRPC login error: {Error}", e);
            context.Status = new Status(StatusCode.InvalidArgument, "Incorrect login credentials");

            return new LoginResponse { Message = "Incorrect login credentials" };
        }
        catch (Exception e)
        {
            _logger.LogError("gRPC login error: {Error}", e);
            context.Status = new Status(StatusCode.Internal, "Service error");

            return new LoginResponse { Message = "Service error" };
        }
    }

    private static AuthService.Models.LoginRequest MapFromGrpcLoginRequest(LoginRequest request)
    {
        return new AuthService.Models.LoginRequest
        {
            Username = request.Username,
            Password = request.Password,
        };
    }
}