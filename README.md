# tripfuse 🔥

[![Crates.io](https://img.shields.io/crates/v/tripfuse.svg)](https://crates.io/crates/tripfuse)
[![Documentation](https://docs.rs/tripfuse/badge.svg)](https://docs.rs/tripfuse)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**One-time use container for sensitive values** - like OTPs, API keys, authenticators, and secrets.

## ⚡ Features

- ✅ **One-time guarantee** - Value can be extracted exactly once
- ✅ **Zero dependencies** - No external crates, minimal footprint
- ✅ **No unsafe code** - 100% safe Rust
- ✅ **Compile-time enforcement** - Prevents accidental reference passing with `'static` bound
- ✅ **Explicit burn** - Manually destroy the fuse without extracting
- ✅ **Security-focused** - Values are moved out, not copied or cloned

## Exambles folder follow link [examples](/examples/)

## 🚀 Quick Start

```rust
use tripfuse::OnceFuse;

// Create a fuse with your secret
let mut fuse = OnceFuse::new(String::from("my_secret_api_key"));

// Use it once - value moves out
let secret = fuse.try_use().unwrap();
println!("Secret: {}", secret);

// Second use fails
assert!(fuse.try_use().is_err());
```

## 📦Installation
Add to your Cargo.toml:

```toml
[dependencies]
tripfuse = "0.1.0"
```

## 🔒Security Guarantees
✅ Do this:
```rust
// Owned types are perfect
let fuse = OnceFuse::new(String::from("secret"));
let fuse = OnceFuse::new(vec![1, 2, 3]);
let fuse = OnceFuse::new(MySecretStruct { ... });

// String literals work too
let fuse = OnceFuse::new("hardcoded_secret");
```
❌ Don't do this:
```rust
let secret = String::from("secret");
let fuse = OnceFuse::new(&secret);  // ❌ Won't compile with 'static bound
// secret remains accessible - security risk!
```

## 📖 API Overview
- `OnceFuse::new(value)`
Creates a new fuse, taking ownership of the value.

- `try_use(&mut self) -> Result<T, TripError>`
Extracts the inner value - succeeds only once.

- `burn_it(&mut self) -> bool`
Explicitly destroys the fuse. Returns true if this call caused the burn.

- `is_armed(&self) -> bool`
Checks if the fuse is still usable.

## 🎯 Use Cases
* OTP (One-Time Passwords) - Ensure each code is used once

* API Keys - Prevent key reuse after initial authentication

* Cryptographic Keys - Move sensitive keys out of memory after use

* Authenticators - One-time tokens for 2FA

* Consumable Resources - Any value that should be used exactly once

## 🔧 Error Handling
```rust
use tripfuse::{OnceFuse, TripError};

let mut fuse = OnceFuse::new("secret");

match fuse.try_use() {
    Ok(value) => println!("Got: {}", value),
    Err(TripError::FuseBurntAfterUsage) => println!("Already used!"),
    Err(TripError::FuseBurntExplicitly) => println!("Was explicitly burned!"),
}
```
## 🧪 Example: OTP Verification
```rust
use tripfuse::OnceFuse;

struct OtpCode(String);

fn send_otp() -> OnceFuse<OtpCode> {
    let code = OtpCode("123456".to_string());
    OnceFuse::new(code)
}

fn verify_otp(mut fuse: OnceFuse<OtpCode>, user_input: &str) -> bool {
    match fuse.try_use() {
        Ok(actual) => actual.0 == user_input,
        Err(_) => false,  // Already used!
    }
}

let otp = send_otp();
assert!(verify_otp(otp, "123456"));  // First use - OK
// Second use would fail automatically
```
## 🧪 Example: OTP Simple Send, Recieve
```rust
use tripfuse::{OnceFuse, TripError};

struct Sender<T> {
    otp: OnceFuse<T>,
}
impl<T> Sender<T> {
    fn send(otp: T) -> Self
    //T must implement `PartialEq` + `Eq`
    // and must be 'static to disable the usage
    //of most references
    where
        T: 'static + PartialEq + Eq,
    {
        Self {
            otp: OnceFuse::new(otp),
        }
    }
}
struct Reciever<T> {
    _n: T,
}
impl<T> Reciever<T> {
    fn rcv(mut sender: Sender<T>) -> Result<T, TripError>
    where
        // once again, 'static is not harmfull
        // is not meant to restrict your usage
        // it is been here to force you use owned
        // values not references(at most cases)
        T: 'static,
    {
        sender.otp.try_use()
    }
}

fn main() {
    let rec: Result<i32, TripError> = Reciever::rcv(Sender::send(12502));
    match rec {
        Ok(otp) => {
            println!("received OTP: {}", otp)
        }
        Err(triperror) => match triperror {
            TripError::FuseBurntAfterUsage => {
                println!("the otp has been used earlier")
            }
            TripError::FuseBurntExplicitly => {
                println!("the otp has been explicitly burnt")
            }
            _ => {
                // `TripError` is marked #[non-exhaustive]
                // wild card is necessary in `match`
                unreachable!()
            }
        },
    }
}

```
## 🧠 Design Notes
- Clone is intentionally not implemented - prevents - duplicate access

- Copy is impossible due to internal state tracking

- Drop is automatic - no manual cleanup needed

- Uses std::mem::replace for safe state transitions

# 📝 License
MIT © [oOp995]

## 🤝 Contributing
Contributions welcome! Please ensure:

- No external dependencies

- Maintain 100% test coverage

- Document all public APIs

- No unsafe code (unless no other solution exists)

Made with 🔒 for security-conscious Rust developers