use tripfuse::{OnceFuse, TripError};

fn main() {
    // #1
    let mut fuse: OnceFuse<&str> = OnceFuse::new("123abc");

    // #2
    // will yield &str because the fuse never used earlier the burn the fuse
    // or yield nothing and burn nothing if the fuse already used between
    // points #1 and #2

    let _oto = fuse.try_use();
    match fuse.try_use() {
        Ok(_s) => {
            //unreachable due to  the call in point #2
            unreachable!()
        }

        // it must reach her due to the call in point #2
        Err(te) => match te {
            TripError::FuseBurntAfterUsage => {
                // fuse used and burnt after usage
            }
            TripError::FuseBurntExplicitly => {
                // fuse not used, burnt by explicit call
            }
            _ => {}
        },
    }
}
