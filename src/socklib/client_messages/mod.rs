pub mod request;
pub mod username_password_auth;
pub mod version_method;

pub use crate::socklib::client_messages::{version_method::VersionMethod, request::ClientRequest};

pub trait ClientMessage
where
    Self: Sized,
{
    fn try_parse<'a>(bytes_iter: &mut impl Iterator<Item = &'a u8>) -> Option<Self>;

    fn size(&self) -> usize;
}
