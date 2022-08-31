# Requirements

- rustup - latest, my version 1.25.1
- rustc - latest, my version 1.63.0

# Scripts

- `cargo build` - compiles `src` folder, after completion server can be run from `target/debug/sat-recruitment-task-rust.exe`
- `cargo run` - developer mode

# Environment

By default server runs on [localhost:3001](http://localhost:3001)

# Endpoints

```http
GET /calculateDisselUsageForDistance?distance=10&yearOfProduction=2000&fuelUsagePer100KM=10
```

| Parameter           | Type     | Description                                                                                 |
| :------------------ | :------- | :------------------------------------------------------------------------------------------ |
| `distance`          | `string` | **Required**. Total distance between point A and point B in kilometers                      |
| `yearOfProduction`  | `string` | **Required**. Year of production of the car, number                                         |
| `fuelUsagePer100KM` | `string` | **Required**. Natural number that represents approximate fuel consumption per 100KM, number |

```http
GET /probabilityOfUnitInjectorFail?VIN=4JGDA5HB4FA521206
```

| Parameter | Type     | Description                                                   |
| :-------- | :------- | :------------------------------------------------------------ |
| `VIN`     | `string` | **Required**. Not relevant, but customer really wants it here |

# Responses

- success response- on success api returns object which contains expected response with status 200 OK

```rust
{
  "field": string | number
}
```

- error response- on error api returns object with error message and status 400 Bad Request

```rust
{
  "message": string
}
```
