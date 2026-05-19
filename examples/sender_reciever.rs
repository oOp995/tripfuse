use tripfuse::{OnceFuse, TripError};

struct Sender<T> {
    otp: OnceFuse<T>,
}
impl<T> Sender<T> {
    fn send(otp: T) -> Self
    //Tmust be 'static to disable the usage
    //of most references
    where
        T: 'static
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
                unreachable!()
            }
        },
    }
}
