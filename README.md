## axum-tonic-api

This repo is a clone of Curioucity server to test specific issue below

- Because we use protobuf as our single source of truth, we will use the protobuf generated struct as rest api response struct.

```rust
use crate::pb; // protobuf types

pub async fn greeter_handler(
    Json(data): Json<pb::helloworld::HelloRequest>,
) -> Result<impl IntoResponse, Response> {
    let resp = pb::helloworld::HelloResponse {
        message: format!("Hello {}", data.name),
        test_array: Vec::new(),
    };

    Ok((StatusCode::OK, Json(resp)))
}
```

But with this implementation, the the serialized response's test_array field will be omitted when you use rest api endpint. But with the grpc api endpoint it will not be omitted.

This behavior is not consistent.

```rust
// rest response
{
  message: "Hello Summerbud",
}

// grpc response
{
  message: "Hello Summerbud",
  test_array: []
}

```

## How to reproduce it

```
// Install necessary protoc lib in your $PATH
cargo install protoc-gen-prost protoc-gen-prost-crate protoc-gen-prost-serdeprotoc-gen-tonic

// Build the program
cargo build

// Set up the server
cargo run
```

The server will be run on http://localhost:8010

## Some thought

- We are using protoc-gen-serde(which use pbjson under the hood) to generate serde serializer and deserializer for us to speed up the development.
- In protobuf the default values are omitted and through this nature prost won't serialize the field with default value [source](https://discord.com/channels/500028886025895936/664895722121986061/784543806472192000)
- What pbjson do is consumes the descriptor output of prost-build and generate the serializer and deserializer

## Potential solution

- We write our own serializer (Which is very repetitive and trouble-prone)
- We side walk the issue somehow