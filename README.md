# Overview

Authentication microservice built with C# and ASP.NET Core. Docker containers and Docker Compose fully supported.

## Features

- JWT access tokens
- User database
- Role-based access control
- gRPC services and REST API
- CLI tool

## Database

PostgreSQL user database. Users are managed with .NET Entity Framework and Identity system.

These provide production-ready user and database management that can be extended.

## Passwords

BCrypt hashing algorithm with pepper. Pepper is a secret value that is added to passwords before passing them to BCrypt. It provides an additional layer of security.

## Environment

| Name                            | Default value                                                 |
|---------------------------------|---------------------------------------------------------------|
| ConnectionStrings__UserDatabase | Host=localhost;Port=12345;Database=userdb;Username=userdb_user;Password=userdb_pass                                                                              |
| AppConfig__PasswordPepper       | YourSecretPepper                                              |
| AppConfig__AdminUsername        | admin                                                         |
| AppConfig__AdminPassword        | Admin10!                                                      |
| AppConfig__NormalUserUsername   | user1                                                         |
| AppConfig__NormalUserPassword   | Pass10!                                                       |
| JwtConfig__Issuer               | issuer.example.com                                            |
| JwtConfig__Audience             | audience.example.com                                          |
| JwtConfig__SigningKey           | Your Signing key.                                             |

CLI

| Name                            | Default value                                                 |
|---------------------------------|---------------------------------------------------------------|
| GRPC_API_URL                    | http://localhost:5106                                         |
| REST_API_URL                    | http://localhost:5105/api/v1                                  |

## Health checks

`/health/summary` - Gives service health summary including database status. This endpoint is only available in port `5105`. gRPC port `5106` does not have this.

## REST API

POST `/api/v1/auth/login` - Login with username and password

GET `/api/v1/users` - Get all users. Authentication and admin role required.

## CLI

CLI tool is built with Rust. You need Rust tools to build it.

Login can be done using both gRPC and REST. Login returns JWT access token and it gets saved to a file. After login, you can list all users (Admin role required.).

```bash
auth-cli --help
```

Login using gRPC service
```bash
auth-cli login
```

Login using REST API
```bash
auth-cli login --rest
```

List all users using gRPC service
```bash
auth-cli user ls
```

List all users using REST API
```bash
auth-cli user ls --rest
```

## Docker

Go to the project root in terminal
```
cd auth-microservice
```

Run PostgreSQL
```
docker compose -f docker-compose.db.yml up -d
```
Access it from your localhost's port `12345`

Run PostgreSQL and microservice
```
docker compose up -d
```
gRPC services port `5106`

REST API port `5105`
