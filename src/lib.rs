#![cfg(target_os="linux")]

pub mod acpi;

#[derive(Debug)]
pub enum Error{
    IO(std::io::Error),
    BadFormat
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    } 
}

type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
