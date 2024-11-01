<div align="center">
  <h1>Monitor</h1>
</div>

Simple cronjob monitoring tool for your servers

## Installation and usage

Use [Nixpacks](https://nixpacks.com/docs/getting-started)
to build the frontend and backend images and run them with Docker

Note that they require some configuration in `.env` files,
see the example files in the `src/backend` and `src/frontend` directories.
More specifically, the backend requires both a redis and a postgres database 
and API keys for some SMTP Server.

Note that the backend requires the `PRODUCTION` environment variable to be set to true to send emails.

### Generate a cookie key
To generate a cookie key,
you need to spin up a new Rust project with `cargo new your_project_name`
and paste the following code:

```toml
# Cargo.toml
rand = "0.9.0-alpha.1"
axum-extra = { version = "0.9.3", features = ["cookie-private"]}
```

```rust
// src/main.rs
use axum_extra::extract::cookie::Key;
use rand::{Rng, thread_rng};

fn main() {
    // Generate a cryptographically random key of 64 bytes
    let mut rng = thread_rng();
    let mut random_key = [0u8; 64];
    rng.fill(&mut random_key);
    match Key::try_from(&random_key[..]) {
        Ok(key) => {
            println!("Random key: {:?}", key.master());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
```

Then run the project with cargo run and copy the cookie key to the .env file in the backend directory.

```bash
cd src/frontend
nixpacks build .
cd ../backend
nixpacks build .
```
and then run the command provided by Nixpacks to run the images.
Remember to set the environment variables as `.env` won't be copied to the Docker container.

## Development

To run the frontend and backend in development mode, you can use the following commands:

```bash
cd src/frontend
pnpm install
pnpm dev
```

```bash
cd src/backend
cargo run
```

## License
This project is licensed under the MIT License, see the [LICENSE](LICENSE) file for details

