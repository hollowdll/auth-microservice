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
