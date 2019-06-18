# Rust REST API 

## Install

Clone this repo:

```git clone git@github.com:CrownBerry/rust-rest.git```

Set `nightly` build default for this project:

```cd rust-rest && rustup override set nightly```

Update crates:

```cargo update && cargo build```

Run app:

```cargo run```

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
### Persones
#### Get
Route: `<base_url>/persons`

Request:

Response: Array of Person JSON objects

### Person
#### Post
Route: `<base_url>/person`

Request: JSON body -> Person JSON object

Response: Person JSON object
#### Put
Route: `<base_url>/person/<id>`

Request: `id` parameter, JSON body -> Person JSON object

Response:
```
{
    "success": bool
}
```
#### Delete
Route: `<base_url>/person/<id>`

Request: `id` parameter

Response:
```
{
    "success": bool
}
```
