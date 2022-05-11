pub mod client_messages;
pub mod server_messages;


use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum AuthMethod {
    NoAuth = 0x00,
    // not supported at the moment
    //gssapi = 0x01, 
    UsernamePassword = 0x02,
    Unknown = 0xff,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ProtocolVersion {
    V5 = 0x05,
    Unknown = 0xff,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Command {
    Connect = 0x01,
    Bind = 0x02,
    UdpAssociate = 0x03,
    Unknown = 0xFF,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum AddressType {
    IPv4 = 0x01,
    DomainName = 0x03,
    // We don't support IPv6 at the moment
    //IPv6 = 0x04,
    Unknown = 0xFF,
}
