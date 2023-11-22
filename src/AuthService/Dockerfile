FROM mcr.microsoft.com/dotnet/sdk:6.0 AS build
WORKDIR /src

COPY ./*.csproj .
RUN dotnet restore

COPY . .
RUN dotnet publish --no-restore -c Release -o /app

# Alpine Linux OS for smaller image size
FROM mcr.microsoft.com/dotnet/aspnet:6.0-alpine
WORKDIR /app
EXPOSE 5105
EXPOSE 5106
COPY --from=build /app .

ENTRYPOINT ["dotnet", "AuthService.dll"]