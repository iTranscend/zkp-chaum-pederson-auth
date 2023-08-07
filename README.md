<!-- markdownlint-disable MD033 -->

# ZKP-Chaum-Pedersen

A Chaum-Pedersen Zero-Knowledge Protocol implementation for password-based authentication.

See [Notes](NOTES.md) for more information on the application design and possible future improvements.

## Usage

### Using cargo

- Start the server

  ```console
  $ cargo run -p zkp-server
  ================== ZKP Auth (Server) ==================
  [i] Listening on '127.0.0.1:3000'
  ```

  <details>
  <summary>See full help information with the <code>--help</code> flag.</summary>

  ```console
  ZKP Auth Server

  Usage: zkp-server [OPTIONS]

  Options:
    -l, --listen <URI>  Sets the address to listen on [default: 127.0.0.1:3000]
                        Valid: `3000`, `127.0.0.1`, `127.0.0.1:3000` [env: PORT]
    -h, --help          Print help
    -V, --version       Print version
  ```

  You can specify the address and port you want your server to run on as such:

  ```console
  $ cargo run -p zkp-server -- -l 127.0.0.1:3004
  ================== ZKP Auth (Server) ==================
  [i] Listening on '127.0.0.1:3004'
  ```

  Additionally, the app checks to see if the `PORT` environment variable is defined:

  ```console
  $ PORT=5004 cargo run -p zkp-server
  ================== ZKP Auth (Server) ==================
  [i] Listening on '127.0.0.1:5004'
  ```

  </details>

- In another terminal, connect to the server and register a user

  ```console
  $ cargo run -p zkp-client register
  =============== ZKP Auth (Registration) ===============
  [?] Enter a User ID: peggy
  [?] Select a Password:
  [i] Successfully registered user
  =============== ZKP Auth (Registration) ===============
  ```

  <details>
  <summary>See full help information with the <code>--help</code> flag.</summary>

  ```console
  Registers a new user

  Usage: zkp-client register [OPTIONS]

  Options:
    -u, --username <USERNAME>  Specifies the username to register
    -p, --password <PASSWORD>  Specifies the password to register [env: PASSWORD]
    -s, --server <URI>         Specifies the server address to connect to [default: http://127.0.0.1:3000]
    -h, --help                 Print help
  ```

  </details>

- Now you can login (you get 3 tries to enter the correct password)

  ```console
  $ cargo run -p zkp-client login
  =================== ZKP Auth (Login) ==================
  [?] Enter Your User ID: peggy
  [?] Enter Your Password:
  [i] Successfully authenticated user, session ID is: "F0tkGNreN6Cy"
  =================== ZKP Auth (Login) ==================
  ```

  <details>
  <summary>See full help information with the <code>--help</code> flag.</summary>

  ```console
  Logs in an existing user

  Usage: zkp-client login [OPTIONS]

  Options:
    -u, --username <USERNAME>  Specifies the username to login with
    -p, --password <PASSWORD>  Specifies the password to login with [env: PASSWORD]
    -s, --server <URI>         Specifies the server address to connect to [default: http://127.0.0.1:3000]
    -h, --help                 Print help
  ```

  </details>

### Usage with Docker

Alternatively, if you want to use docker and you have docker installed. Follow the steps below:

- Build the images

  ```console
  docker compose build
  ```

- Start the server

  ```console
  docker compose up zkp-server
  ```

- In another terminal, connect to the server and register a user

  ```console
  docker compose run zkp-client register
  ```

- Now you can login (you get 3 tries to enter the correct password)

  ```console
  docker compose run zkp-client login
  ```

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
