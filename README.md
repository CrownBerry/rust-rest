# Rust REST API 

## Install

Clone this repo:
```bash
git clone git@github.com:CrownBerry/rust-rest.git
```
Set `nightly` build default for this project:
```bash
cd rust-rest && rustup override set nightly
```
Update crates:
```bash
cargo update && cargo build
```
Copy `.env` file
```bash
cp .env.example .env
```
Fill it with your `DATABASE_URL` and run migrations:
```bash
diesel migration run
```
Run app:
```bash
cargo run
```
## Authentication

Use table `users` for storing users. Column `password_hash` store hashed password. 
Hash function is `bcrypt` with 12 round of hashing.

For authenticate on `person` updating request, use header `Authentication` with JWT-token.
For acquiring token, proceed on route `/login` with JSON-body `UserRequest`

Example:
```json
{
  "username": "test_user",
  "password": "12345"
}
```

## Routes
### Person JSON object
Schema:
```
{
    "id": Option<int>,
    "name": String,
    "age": i32
}
```
Example:
```json
{
    "name": "John Wick",
    "age": 47
}
```
#### Persones
##### Get
Route: `<base_url>/persons`

Request:

Response: Array of Person JSON objects

#### Person
##### Post
Route: `<base_url>/person`

Request: JSON body -> Person JSON object

Response: Person JSON object
##### Put
Route: `<base_url>/person/<id>`

Request: `id` parameter, JSON body -> Person JSON object

Response:
```
{
    "success": bool
}
```
##### Delete
Route: `<base_url>/person/<id>`

Request: `id` parameter

Response:
```
{
    "success": bool
}
```
