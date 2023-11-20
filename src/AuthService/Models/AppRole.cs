namespace AuthService.Models;

public static class AppRole
{
    public const string User = "User";
    public const string Admin = "Admin";

    public static string[] GetRoleNames()
    {
        string[] roleNames = { User, Admin };
        return roleNames;
    }
}