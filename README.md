# Single-Threaded Web Server in Rust

This project is a simple single-threaded web server implemented in Rust. The server listens on port 7878 and handles HTTP requests. It serves a `hello.html` file for the home route and a `404.html` file for any other routes. Additionally, a sleep function is implemented to demonstrate the single-threaded nature of the server.

## Features

- **Single-threaded**: The server processes one request at a time.
- **Routing**:
  - Home route (`/`): Redirects to `hello.html`.
  - Any other route: Returns `404.html`.
- **Sleep Function**: Simulates a delay to test single-threading.

## Getting Started

### Prerequisites

- Rust installed on your machine.

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/tusharagrahari/web-server.git
    cd single-threaded-web-server
    ```

2. Build the project:
    ```sh
    cargo build
    ```

### Usage

1. Run the server:
    ```sh
    cargo run
    ```

2. Open your web browser and navigate to `http://localhost:7878`.

### Project Structure

- `src/main.rs`: The main file containing the server implementation.
- `hello.html`: The HTML file served at the home route.
- `404.html`: The HTML file served for any other routes.
