using AuthService.Data;
using AuthService.Models;
using AuthService.Config;
using Microsoft.AspNetCore.Identity;
using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateBuilder(args);

// Configure database
builder.Services.AddDbContext<ApplicationDbContext>(options =>
    options.UseNpgsql(builder.Configuration.GetConnectionString("UserDatabase"), npgsqlOptions =>
        npgsqlOptions.CommandTimeout(15)));

// Configure identity system
builder.Services.AddIdentity<AppUser, IdentityRole>()
    .AddEntityFrameworkStores<ApplicationDbContext>()
    .AddDefaultTokenProviders();

builder.Services.Configure<AppConfig>(builder.Configuration.GetSection("AppConfig"));

builder.Services.AddControllers();
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();
app.UseAuthorization();
app.MapControllers();

using (var serviceScope = app.Services.CreateScope())
{
    await SeedData.InitializeDatabase(serviceScope, app.Environment, app.Logger);
}

await app.RunAsync();