# Overview

Authentication microservice built with C# and ASP.NET Core. Docker containers and Docker Compose fully supported.

## Features

- JWT access tokens
- User database
- Role-based access control
- gRPC services and REST API
- CLI tool with login

## Database

PostgreSQL user database. Users are managed with .NET Entity Framework and Identity system.

These provide production-ready user and database management that can be extended.

## Passwords

BCrypt hashing algorithm with pepper. Pepper is a secret value that is added to passwords before passing them to BCrypt. It provides an additional layer of security.

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
