namespace AuthService.Models;

public static class AppRole
{
    public readonly static string User = "User";
    public readonly static string Admin = "Admin";

    public static string[] GetRoleNames()
    {
        string[] roleNames = { User, Admin };
        return roleNames;
    }
}