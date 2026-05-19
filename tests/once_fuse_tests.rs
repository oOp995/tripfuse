#[cfg(test)]
mod tests {
    use tripfuse::OnceFuse;

    #[test]
    fn new_fuse_test() {
        let secret = vec![1];
        let mut fuse = OnceFuse::new(secret);
        assert_eq!(fuse.try_use().unwrap(), vec![1]);
    }
}
