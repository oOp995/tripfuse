use std::error::Error;
use tripfuse::OnceFuse;
fn main() -> Result<(), Box<dyn Error>> {
    let oto = String::from("123abc");
    let mut fuse = OnceFuse::new(oto); // oto can be used once and only once.

    let oto = fuse.try_use(); //first usage

    let oto1 = fuse.try_use(); //second usage will return error

    let _ = dbg!(oto);
    let _ = dbg!(oto1);

    Ok(())
}
