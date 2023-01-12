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

But with this implementation, the the serialized response's timestamp field will not have correct type.


```rust
// rest response
{
  timestamp: "123",
}

// grpc response
{
  timestamp: "123",
}

```

## How to reproduce it

```bash
// Install necessary protoc lib in your $PATH
cargo install protoc-gen-prost protoc-gen-prost-crate protoc-gen-prost-serdeprotoc-gen-tonic

// Build the program
cargo build

// Set up the server
cargo run

// Get the rest response
curl http://localhost:8010
```