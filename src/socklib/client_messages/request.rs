use crate::socklib::{AddressType, Command, ProtocolVersion};
use std::net::Ipv4Addr;

use super::ClientMessage;

#[derive(Debug)]
pub struct ClientRequest {
    pub ver: ProtocolVersion,
    pub cmd: Command,
    pub rsv: u8,
    pub atyp: AddressType,
    pub ipv4_addr: Ipv4Addr,
    pub domain_name: String,
    pub port: u16,
}

impl ClientMessage for ClientRequest {
    fn try_parse<'a>(bytes_iter: &mut impl Iterator<Item = &'a u8>) -> Option<ClientRequest> {
        let mut cr = ClientRequest {
            ver: ProtocolVersion::Unknown,
            cmd: Command::Unknown,
            rsv: 0,
            atyp: AddressType::Unknown,
            ipv4_addr: Ipv4Addr::UNSPECIFIED,
            domain_name: String::new(),
            port: 0,
        };

        let ver_byte = *bytes_iter.next()?;
        let cmd_byte = *bytes_iter.next()?;
        let rsv = *bytes_iter.next()?;
        let atyp_byte = *bytes_iter.next()?;

        cr.ver = ProtocolVersion::try_from(ver_byte).unwrap_or(ProtocolVersion::Unknown);
        cr.cmd = Command::try_from(cmd_byte).unwrap_or(Command::Unknown);
        cr.rsv = rsv;
        cr.atyp = AddressType::try_from(atyp_byte).unwrap_or(AddressType::Unknown);

        match cr.atyp {
            AddressType::IPv4 => {
                let ipv4_addr_bytes = bytes_iter.take(4).map(|x| *x).collect::<Vec<u8>>();
                if ipv4_addr_bytes.len() != 4 {
                    return None;
                }
                cr.ipv4_addr = Ipv4Addr::new(
                    ipv4_addr_bytes[0],
                    ipv4_addr_bytes[1],
                    ipv4_addr_bytes[2],
                    ipv4_addr_bytes[3],
                );
            }
            AddressType::DomainName => {
                let domain_len = *bytes_iter.next()?;
                let domain_bytes = bytes_iter
                    .take(domain_len as usize)
                    .map(|x| *x)
                    .collect::<Vec<u8>>();
                if domain_bytes.len() != domain_len as usize {
                    return None;
                }
                if let Ok(domain_str) = String::from_utf8(domain_bytes) {
                    cr.domain_name = domain_str;
                } else {
                    return None;
                }
            }
            AddressType::Unknown => {
                return Some(cr);
            }
        };

        let port_bytes = bytes_iter.take(2).map(|x| *x).collect::<Vec<u8>>();
        cr.port = u16::from_be_bytes(
            port_bytes
                .try_into()
                .expect("Failed to convert port slice to array"),
        );

        return Some(cr);
    }

    fn size(&self) -> usize {
        let size = match self.atyp {
            AddressType::IPv4 => 4,
            AddressType::DomainName => self.domain_name.len(),
            AddressType::Unknown => 0,
        };
        return size + 6;
    }
}
