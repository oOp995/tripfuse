use tripfuse::{OnceFuse, TripError};
fn main() {
    //some arbitrary code to get or generate one-time password
    let secret = "123ABC";
    let mut fuse = OnceFuse::new(secret);

    //use-it
    //note that usage is only one time
    for _ in 0..1 {
        match fuse.try_use() {
            Ok(_otp) => {
                //use the value
            }
            Err(e) => {
                //TripError
                //value has been used earlier
                //you can match against the TripError variants to know exactly what happened
                match e {
                    TripError::FuseBurntAfterUsage => {}
                    TripError::FuseBurntExplicitly => {
                        //..
                    }
                    _ => {
                        //Trip error is marked non-exhaustive
                        //the wildcard is necessary
                    }
                }
            }
        }
    }
}
