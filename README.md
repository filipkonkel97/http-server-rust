# Rust HTTP Server

A simple HTTP server implemented from scratch in Rust.

The project demonstrates how HTTP communication works internally by building a server on top of TCP sockets without using external web frameworks.

## Features

- TCP-based HTTP server
- HTTP request parsing
- HTTP method handling
- Query string parsing
- HTTP response generation
- Status code management
- Static file serving
- Configurable public directory

## Project Structure

```text
.
├── Cargo.lock
├── Cargo.toml
├── public
│   ├── hello.html
│   ├── index.html
│   └── style.css
├── src
│   ├── http
│   │   ├── method.rs
│   │   ├── mod.rs
│   │   ├── query_string.rs
│   │   ├── request.rs
│   │   ├── response.rs
│   │   └── status_code.rs
│   ├── main.rs
│   ├── server.rs
│   └── website_handler.rs
```

## Architecture

The server consists of several main components.

### Server

`server.rs`

Responsible for:

- Creating TCP listener
- Accepting incoming connections
- Reading incoming HTTP requests
- Passing requests to the handler
- Sending responses back to clients

Request flow:

```
Client
  |
  v
TCP Connection
  |
  v
HTTP Request
  |
  v
Request Parser
  |
  v
Website Handler
  |
  v
HTTP Response
```

## HTTP Module

The `http` module contains HTTP-related logic.

### Request Parser

`request.rs`

Converts raw bytes received from a TCP stream into a structured request.

Example:

```
GET /index.html?name=rust HTTP/1.1
```

is parsed into:

- HTTP method
- Path
- Query string

The parser supports:

- HTTP/1.1 validation
- HTTP method validation
- Path extraction
- Query string extraction

## HTTP Methods

`method.rs`

Contains supported HTTP methods:

- GET
- POST
- PUT
- DELETE
- HEAD
- CONNECT
- OPTIONS
- TRACE
- PATCH

Methods are parsed using `FromStr`.

Example:

```rust
let method: Method = "GET".parse()?;
```

## Query String Parser

`query_string.rs`

Parses URL query parameters.

Example:

```
/search?name=rust&language=en
```

becomes:

```
name = rust
language = en
```

The parser also supports multiple values:

```
?id=1&id=2
```

Result:

```
id = [1,2]
```

## Response

`response.rs`

Responsible for creating HTTP responses returned to clients.

Example:

```
HTTP/1.1 200 OK

<html>
    <body>Hello World</body>
</html>
```

## Status Codes

`status_code.rs`

Contains HTTP response status codes:

- 200 OK
- 404 NOT_FOUND
- 500 INTERNAL_SERVER_ERROR

## Website Handler

`website_handler.rs`

Handles serving static files from the public directory.

Example request:

```
GET /index.html HTTP/1.1
```

returns:

```
public/index.html
```

The default public directory:

```
./public
```

can be changed using an environment variable.

Example:

Linux/macOS:

```bash
PUBLIC_PATH=/var/www cargo run
```

Windows:

```powershell
$env:PUBLIC_PATH="C:\website"
cargo run
```

## Running the Server

Install Rust:

https://www.rust-lang.org/tools/install

Run the project:

```bash
cargo run
```

The server starts on:

```
127.0.0.1:8080
```

## Testing

Open in browser:

```
http://127.0.0.1:8080
```

or use curl:

```bash
curl http://127.0.0.1:8080/index.html
```

## Technologies

- Rust
- TCP networking
- HTTP protocol
- File system handling
- Error handling
- Rust ownership and lifetimes

## Learning Goals

The purpose of this project was to understand:

- How HTTP works internally
- How TCP servers handle connections
- How requests are parsed
- How responses are generated
- How Rust ownership and borrowing work in networking applications

## Future Improvements

Possible improvements:

- Multithreading
- Async support with Tokio
- HTTP headers parsing
- MIME type detection
- Better error pages
- Routing system
- Middleware support