using AuthService.Data;
using AuthService.Models;
using AuthService.Config;
using AuthService.Services;
using Microsoft.AspNetCore.Identity;
using Microsoft.EntityFrameworkCore;
using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.IdentityModel.Tokens;
using System.Text;
using Microsoft.AspNetCore.Diagnostics.HealthChecks;
using AuthService.Utility;

var builder = WebApplication.CreateBuilder(args);
var databaseConnectionString = builder.Configuration.GetConnectionString("UserDatabase");

// Configure database
builder.Services.AddDbContext<ApplicationDbContext>(options =>
    options.UseNpgsql(databaseConnectionString, npgsqlOptions =>
        npgsqlOptions.CommandTimeout(15)));

// Configure identity system
builder.Services.AddIdentityCore<AppUser>(options =>
    options.SignIn.RequireConfirmedAccount = false)
        .AddRoles<IdentityRole>()
        .AddEntityFrameworkStores<ApplicationDbContext>()
        .AddDefaultTokenProviders();

// Add JWT bearer token authentication
builder.Services.AddAuthentication(JwtBearerDefaults.AuthenticationScheme)
    .AddJwtBearer(options =>
{
    options.TokenValidationParameters = new TokenValidationParameters
    {
        ValidateIssuer = true,
        ValidateAudience = true,
        ValidateIssuerSigningKey = true,
        ValidateLifetime = true,
        ValidIssuer = builder.Configuration["JwtConfig:Issuer"],
        ValidAudience = builder.Configuration["JwtConfig:Audience"],
        IssuerSigningKey = new SymmetricSecurityKey(
            Encoding.UTF8.GetBytes(builder.Configuration["JwtConfig:SigningKey"])),
        ClockSkew = TimeSpan.Zero
    };
});

// Add other services
builder.Services.Configure<AppConfig>(builder.Configuration.GetSection("AppConfig"));
builder.Services.Configure<JwtConfig>(builder.Configuration.GetSection("JwtConfig"));

builder.Services.AddScoped<IJwtService, JwtService>();
builder.Services.AddScoped<ILoginService, LoginService>();

builder.Services.AddGrpc();
builder.Services.AddControllers();
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();
builder.Services.AddHealthChecks()
    .AddDbContextCheck<ApplicationDbContext>("PostgreSQL");

DbConnection.CheckDatabaseConnection(databaseConnectionString);

var app = builder.Build();

// Use Swagger only in development
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

// Add middlewares
app.UseHttpsRedirection();
app.UseAuthentication();
app.UseAuthorization();
app.MapGrpcService<GrpcAuth.LoginService>();
app.MapGrpcService<GrpcAuth.UserService>();
app.MapControllers();
app.MapHealthChecks("/health/summary", new HealthCheckOptions
{
    ResponseWriter = HealthCheckUtility.WriteResponse
});

using (var serviceScope = app.Services.CreateScope())
{
    await SeedData.InitializeDatabase(serviceScope, app.Environment, app.Logger);
}

await app.RunAsync();