// Application configurations
public class Config
{
    public class Secrets
    {
        /// <summary>Pepper value used when encrypting passwords.</summary>
        public static string PasswordPepper { get; set; } = null!;
    }
}