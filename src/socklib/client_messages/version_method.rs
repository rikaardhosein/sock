use crate::socklib::{AuthMethod, ProtocolVersion};

use super::ClientMessage;

#[derive(Debug)]
pub struct VersionMethod {
    pub ver: ProtocolVersion,
    pub nmethods: u8,
    pub methods: Vec<AuthMethod>,
}

impl ClientMessage for VersionMethod {

    fn try_parse<'a>(byte_iter: &mut impl Iterator<Item=&'a u8>) -> Option<VersionMethod> {
        let version_byte = *byte_iter.next()?;
        let nmethods = *byte_iter.next()?;
        let methods: Vec<&u8> = byte_iter.take(nmethods as usize).collect::<Vec<&u8>>();
        if methods.len() != nmethods as usize {return None;}

        let version = ProtocolVersion::try_from(version_byte).unwrap_or(ProtocolVersion::Unknown);
        let methods: Vec<AuthMethod> = methods
            .iter()
            .map(|m| AuthMethod::try_from(**m).unwrap_or(AuthMethod::Unknown))
            .collect();

        return Some(VersionMethod {
            ver: version,
            nmethods: nmethods,
            methods: methods,
        });
    }



    fn size(&self) -> usize {
        return (2 + self.nmethods) as usize;
    }
}

#[cfg(test)]
mod tests {}
