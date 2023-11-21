namespace AuthService.Models;

public class UserData
{
    public string Id { get; set; } = null!;
    public string Username { get; set; } = null!;
    public IList<string> Roles { get; set; } = null!;
}