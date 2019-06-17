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
```json
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
Route:
Request:
Response:

### Person
#### Post
Route:
Request:
Response:
#### Put
Route:
Request:
Response:
#### Delete
Route:
Request:
Response: