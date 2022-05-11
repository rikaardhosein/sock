pub struct UsernamePasswordAuth {
    ver: u8,
    username: String,
    password: String,
}

impl UsernamePasswordAuth {
    pub fn size(self) -> u64 {
        let mut size: u64 = 0;
        size += 1;
        size += 1 + (self.username.len() as u64);
        size += 1 + (self.password.len() as u64);
        return size;
    }

    pub fn try_parse(bytes: &[u8]) -> Option<UsernamePasswordAuth> {
        let ver = *bytes.get(0)?;
        let username_len = *bytes.get(1)? as usize;
        let username_bytes = bytes.get(2..2 + username_len)?;
        let password_offset = 2 + username_len;
        let password_len = *bytes.get(password_offset)? as usize;
        let password_bytes = bytes.get(password_offset + 1..password_offset + 1 + password_len)?;

        if let Ok(username) = String::from_utf8(username_bytes.to_vec()) {
            if let Ok(password) = String::from_utf8(password_bytes.to_vec()) {
                return Some(UsernamePasswordAuth {
                    ver: ver,
                    username: username,
                    password: password,
                });
            }
        }
        return None;
    }
}
