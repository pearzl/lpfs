#![cfg(any(target_os = "linux", target_os = "android"))]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
