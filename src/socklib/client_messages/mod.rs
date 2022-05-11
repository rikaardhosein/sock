pub mod request;
pub mod version_method;
pub mod username_password_auth;

pub struct VersionMethod;

pub trait ClientMessage where Self: Sized {
    //fn try_parse(bytes: &[u8]) -> Option<Self>;
    fn try_parse<'a>(bytes_iter: &mut impl Iterator<Item=&'a u8>) -> Option<Self>;

    fn size(&self) -> usize;
}