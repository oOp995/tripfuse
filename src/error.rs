use std::{error::Error, fmt::Display};

/// main error type for tripfuse crate
/// includes errors for all the currently implemented types
/// marked `#[non-exhaustive]` to allow add variants in the
/// future without breaking users' code
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum TripError {
    //OnceFuse errors
    ///fuse burnt by explicit call
    FuseBurntExplicitly,

    ///fuse burnt after usage
    FuseBurntAfterUsage,
}

impl Display for TripError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TripError::*;
        match self {
            FuseBurntExplicitly => {
                write!(
                    f,
                    "fuse burnt by explicit call to OnceFuse::burn_it(&mut self)"
                )
            }
            FuseBurntAfterUsage => {
                write!(f, "fuse burnt after usage (one-time) use")
            }
        }
    }
}
impl Error for TripError {}
