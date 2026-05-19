use tripfuse::OnceFuse;
fn main() {
    

// Create a fuse with your secret
let mut fuse = OnceFuse::new(String::from("my_secret_api_key"));

// Use it once - value moves out
let secret = fuse.try_use().unwrap();
println!("Secret: {}", secret);

// Second use fails
assert!(fuse.try_use().is_err());
}