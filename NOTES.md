# ZKP-Chaum-Pedersen

A Chaum-Pedersen Zero-Knowledge Protocol implementation for password-based authentication.

## Approach

- The application is a gRPC client and server, with a common library for functionality needed by both client and server binaries.
- A rich CLI to ease interaction with the client.
- The public API is carefully crafted to provide a clean and intuitive interface for external dependants.

## Future Extensions and Integration

- Auth pair entries could expire after a certain period of time.
- Without the requirement of a gRPC interface, the server can be packaged into a smart contract and deployed on a blockchain which will act as a persistent database.
- The algorithm for proving can be made non interactive by requiring the prover (Peggy) to generate `c` on their end.
  
  This way, we can shrink the three-way login process to one step in which Peggy sends `r1`, `r2`, `c` and `s` to the verifier.

  This also alleviates the need to store `r1`, `r2` and `c` during the login process, meaning we would no longer need `auth_id`.
