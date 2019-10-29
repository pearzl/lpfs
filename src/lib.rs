#![cfg(target_os = "linux")]

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    BadFormat,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

type Result<T> = std::result::Result<T, Error>;

macro_rules! default_read {
    ($fn_name: ident, $path: expr) => {
        pub fn $fn_name() -> Result<String> {
            Ok(std::fs::read_to_string($path)?)
        }
    };
}

macro_rules! default_pairs {
    ($fn_name: ident, $path: expr) => {
        default_pairs! {$fn_name, $path, "\n\n", ':'}
    };
    ($fn_name: ident, $path: expr, $sep_block: expr, $sep_pair: expr) => {
        pub fn $fn_name() -> Result<Vec<std::collections::HashMap<String, String>>> {
            let content = std::fs::read_to_string($path)?;
            let mut ret = vec![];

            for processor in content.split($sep_block) {
                let mut map = std::collections::HashMap::new();
                for line in processor.lines() {
                    let mut kv = line.split($sep_pair);
                    let key = kv.next().ok_or(Error::BadFormat)?;
                    let value = kv.next().ok_or(Error::BadFormat)?;
                    map.insert(key.trim().to_string(), value.trim().to_string());
                }
                if !map.is_empty() {
                    ret.push(map);
                }
            }

            Ok(ret)
        }
    };
}

macro_rules! output_unit_test {
    ($fn_name: ident) => {
        #[test]
        fn $fn_name() {
            println!("{:#?}", super::$fn_name().unwrap());
        }
    };
}

pub mod proc;
