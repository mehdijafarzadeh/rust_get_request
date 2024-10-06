# Rust HTTP Client Example with Custom Error Handling

This is a simple Rust project that demonstrates how to make HTTP requests using the `reqwest` crate and handle errors using the `thiserror` crate. The project fetches data from an external URL (`http://httpbin.org/get`), reads the response, and prints the status, headers, and body of the HTTP response.

## Features

- Makes a synchronous HTTP GET request.
- Handles both HTTP request errors (`reqwest::Error`) and I/O errors (`std::io::Error`) using a custom error type.
- Uses the `thiserror` crate to simplify error handling with automatic conversion.
- Includes proper Rust documentation and comments for better understanding.

## Dependencies

- `reqwest` crate (for HTTP requests)
- `thiserror` crate (for custom error handling)

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
thiserror = "1.0"
