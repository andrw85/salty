use cocoon::Cocoon;

pub trait Cipherble {
    fn new<'a>(pwd: &'a String) -> Cocoon<'a, cocoon::Creation>;
    fn from_bytes<'a>(pwd: &'a [u8]) -> Cocoon<'a, cocoon::Creation>;
}

#[derive(Default)]
pub struct FastCipher;
impl FastCipher {}
impl Cipherble for FastCipher {
    fn new<'a>(pwd: &'a String) -> Cocoon<'a, cocoon::Creation> {
        Cocoon::new(pwd.as_bytes()).with_weak_kdf()
    }
    fn from_bytes<'a>(pwd: &'a [u8]) -> Cocoon<'a, cocoon::Creation> {
        Cocoon::new(pwd).with_weak_kdf()
    }
}

#[derive(Default)]
pub struct SlowCipher;
impl SlowCipher {}

impl Cipherble for SlowCipher {
    fn new<'a>(pwd: &'a String) -> Cocoon<'a, cocoon::Creation> {
        Cocoon::new(pwd.as_bytes())
    }
    fn from_bytes<'a>(pwd: &'a [u8]) -> Cocoon<'a, cocoon::Creation> {
        Cocoon::new(pwd)
    }
}
