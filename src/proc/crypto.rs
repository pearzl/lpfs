// 5.2.5.  `/proc/crypto`
// 
// This file lists all installed cryptographic ciphers used by the Linux kernel, including additional details for each. 
// A sample `/proc/crypto` file looks like the following:
// 
// <pre class="screen">name         : sha1
// module       : kernel
// type         : digest
// blocksize    : 64
// digestsize   : 20
// name         : md5
// module       : md5
// type         : digest
// blocksize    : 64
// digestsize   : 16</pre>
//
// -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-crypto

use std::collections::HashMap;
define_struct! {
    /// Represent an entry in /proc/crypto
    /// 
    /// This is an HashMap inside, and Deref trait is implement. 
    /// However it provide some methods to retrive common fields.
    /// 
    /// See [`crypto/proc.c`](https://github.com/torvalds/linux/blob/master/crypto/proc.c).
    pub struct Crypto(HashMap<String, String>);
}

impl Crypto {
    /// It is assumed that `name` is exist in an entry, if not it will panic.
    pub fn name(&self) -> &str {
        self.get("name").unwrap()
    }

    /// It is assumed that `name` is exist in an entry, if not it will panic.
    pub fn driver(&self) -> &str {
        self.get("driver").unwrap()
    }

    /// It is assumed that `name` is exist in an entry, if not it will panic.
    pub fn module(&self) -> &str {
        self.get("module").unwrap()
    }

    /// It is assumed that `name` is exist in an entry, if not it will panic.
    /// 
    /// Return true is the value is "passed".
    pub fn selftest(&self) -> bool {
        self.get("selftest").unwrap() == "passed"
    }
}

use std::str::FromStr;
impl FromStr for Crypto {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Crypto, crate::ProcErr> {
        let mut ret = HashMap::new();
        for line in s.trim().lines() {
            let columns: Vec<&str> = line.split(':').collect();
            if columns.len() != 2 {
                return Err("not an key-value pair in crypto".into());
            }
            ret.insert(columns[0].trim().to_string(), columns[1].trim().to_string());
        }
        Ok(Crypto(ret ))
    }
}

list_impl! {
    crypto, "/proc/crypto", Crypto, "\n\n", 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_crypto() {
        let source = {
"name         : aes
driver       : aes-asm
module       : kernel
priority     : 200
refcnt       : 1
selftest     : passed
type         : cipher
blocksize    : 16
min keysize  : 16
max keysize  : 32"
        };
        let mut map = std::collections::HashMap::new();
        map.insert(String::from("name"       ), String::from("aes"));
        map.insert(String::from("driver"     ), String::from("aes-asm"));
        map.insert(String::from("module"     ), String::from("kernel"));
        map.insert(String::from("priority"   ), String::from("200"));
        map.insert(String::from("refcnt"     ), String::from("1"));
        map.insert(String::from("selftest"   ), String::from("passed"));
        map.insert(String::from("type"       ), String::from("cipher"));
        map.insert(String::from("blocksize"  ), String::from("16"));
        map.insert(String::from("min keysize"), String::from("16"));
        map.insert(String::from("max keysize"), String::from("32"));
        let correct = Crypto (map);
        assert_eq!(correct, source.parse().unwrap());
    }
}
