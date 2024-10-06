use reqwest::blocking;
use std::io::Read;
use thiserror::Error;

/// Custom error type for handling both I/O and HTTP errors.
///
/// This enum defines two possible error types:
/// - `Io` for handling `std::io::Error` during file or stream I/O operations.
/// - `HttpRequest` for handling `reqwest::Error` during HTTP requests.
#[derive(Error, Debug)]
pub enum MyError {
    /// I/O error variant with a detailed error message from `std::io::Error`.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP request error variant with a detailed error message from `reqwest::Error`.
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),
}

/// Main function to send an HTTP GET request and read the response body.
///
/// Sends a request to `http://httpbin.org/get`, reads the response,
/// and prints the status, headers, and body to the console.
///
/// # Returns
/// - `Ok(())` if successful.
/// - `Err(MyError)` if an I/O or HTTP error occurs.
///
/// This function uses the `reqwest::blocking::get` function to send the HTTP request.
/// Any errors are automatically propagated and converted into `MyError` using `?`.
fn main() -> Result<(), MyError> {
    // Send an HTTP GET request to the provided URL
    let mut res = blocking::get("http://httpbin.org/get")?;

    // Create an empty string buffer to store the response body
    let mut body = String::new();

    // Read the response body into the buffer using the `Read` trait
    res.read_to_string(&mut body)?;

    // Print the status code of the HTTP response
    println!("Status: {}", res.status());

    // Print the headers of the HTTP response
    println!("Headers: {:?}", res.headers());

    // Print the body content of the HTTP response
    println!("Body: {}", body);

    Ok(())
}
