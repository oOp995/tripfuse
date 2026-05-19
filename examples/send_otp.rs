use tripfuse::OnceFuse;

#[derive(PartialEq, Eq)]
struct OtpCode(String);

fn send_otp() -> OnceFuse<OtpCode> {
    //some arbirtary code to get the otp code
    let code = OtpCode("123456".to_string());
    OnceFuse::new(code)
}

fn verify_otp(fuse: &mut OnceFuse<OtpCode>, user_input: &str) -> bool {
    match fuse.try_use() {
        Ok(actual) => actual.0 == user_input,
        Err(_) => false, // Already used!, or match against the error
                         // to know how the fuse is burnt, explicit or after usage
    }
}
fn main() {
    let mut otp = send_otp();
    assert!(verify_otp(&mut otp, "123456")); // First use - OK
    // Second use would fail automatically
    let ve = verify_otp(&mut otp, "123");
    assert_eq!(ve, false);
}
