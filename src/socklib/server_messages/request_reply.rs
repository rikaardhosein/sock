use std::net::Ipv4Addr;

use crate::socklib::{AddressType, ProtocolVersion};

use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ReplyType {
    Succeeded = 0x00,
    GeneralFailure = 0x01,
    ConnectionNotAllowed = 0x02,
    NetworkUnreachable = 0x03,
    HostUnreachable = 0x04,
    ConnectionRefused = 0x05,
    TTLExpired = 0x06,
    CommandNotSupported = 0x07,
    AddressTypeNotSupported = 0x08,
    Unassigned = 0xFF,
}

#[derive(Debug)]
pub struct RequestReply {
    pub ver: ProtocolVersion,
    pub rep: ReplyType,
    pub rsv: u8,
    pub atyp: AddressType,
    pub ipv4_bind_addr: Ipv4Addr,
    pub bind_port: u16,
}

impl RequestReply {
    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();

        let port = self.bind_port.to_be_bytes();
        let bind_addr = self.ipv4_bind_addr.octets();

        res.extend([self.ver as u8, self.rep as u8, self.rsv, self.atyp as u8].iter());
        res.extend(bind_addr.iter());
        res.extend(port.iter());

        return res;
    }
}
