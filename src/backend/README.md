# Backend

## Generating a key to put in env

```rust
use tower_sessions::cookie::Key;

fn main() {
    let key = Key::generate();

    println!("{:?}", key.master());
}
```