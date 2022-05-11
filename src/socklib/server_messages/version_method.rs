use crate::socklib::{AuthMethod, ProtocolVersion};

#[derive(Debug)]
pub struct VersionMethod {
    pub ver: ProtocolVersion,
    pub method: AuthMethod,
}

impl VersionMethod {
    pub fn serialize(&self) -> [u8; 2] {
        return [self.ver as u8, self.method as u8];
    }
}
