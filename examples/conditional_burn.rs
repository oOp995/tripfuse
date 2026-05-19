use tripfuse::OnceFuse;
// some arbitrary code or method to determine suspicous
//try make it `false`
static SUSPICOUS: bool = false;
fn main() {
    let mut fuse = OnceFuse::new([1; 10]);
    //let _ = Once.try_use(); // try uncomment this
    if SUSPICOUS {
        if fuse.burn_it() {
            println!("fuse burnt explictly");
            println!("{:?}", fuse)
        }
    } else {
        if let Ok(oto) = fuse.try_use() {
            // or fuse.try_get_clone() if you need owned value, and performance cost is acceptable
            println!("we got: {:?}", oto)
        } else {
            println!("fuse burnt after usage")
        }
    }
}
