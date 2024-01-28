# Google OAuth Axum Starter

This project serves as a start point for a new Axum web project. It includes Google OAuth
already integrated and a handful of other endpoints. It supports using a Postgres DB to
store the user data acquired from google. The aim is to provide an expandable base for others
to use with authentication already integrated.

## Endpoints

- `/api/v1/users/oauth/google` - Endpoint for Google login to redirect to, creates the user if appropriate returns an access token.
- `/api/v1/whoami` - Auth test endpoint using JWT middleware, will return the JWTClaims as JSON.
- `/api/v1/health_check` - Simply pings the database to check the connection.
