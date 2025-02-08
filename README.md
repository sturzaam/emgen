# emgen

Generate Rust Entity Models from MSSQL tables with EMGen.

## Getting Started

### Installation

To install `emgen`, add it to your project using Cargo:

```sh
cargo add emgen
```

### Build script

To use `emgen` in a `build.rs` script, you can add the following code:

```rust
fn main() {
    // Specify the connection string to your MSSQL database
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("sa", "<YourStrong@Passw0rd>"));
    config.trust_cert();

    // Establish a TCP connection to the database
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    // Create a new client using the TCP connection
    let mut client = Client::connect(config, tcp.compat_write()).await?;

    // Generate the Rust entity models
    let entities = information_schema(client, "entity").await;

    // Snapshot the generated entities
    insta::assert_snapshot!(generate_schema(entities.clone()), @r##""##);
}
```

Overall, this script connects to an MSSQL database, generates Rust entity models based on the database schema, and uses snapshot testing to ensure the generated schema remains consistent.


### Prerequisites

- Docker
- Docker Compose
- Rust and Cargo

### Setting Up the Database

To start the database, use Docker Compose:

```sh
docker-compose up -d
```

This command will start the database in detached mode.

### Running Tests

To run the tests, use Cargo:

```sh
cargo test
```

This command will execute all the tests in the project.

## Contributing

We welcome contributions to the project. To contribute, follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes.
4. Ensure all tests pass.
5. Open a pull request with a detailed description of your changes.

If you encounter any issues, feel free to open an issue on GitHub.

## License

MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.