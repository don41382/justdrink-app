use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct DeviceId {
    id: String,
}

impl DeviceId {
    pub fn lookup() -> Result<Self, anyhow::Error> {
        Ok(Self {
            id: machine_uid::get()
                .map_err(|err| anyhow::anyhow!("failed to get machine id: {}", err))?,
        })
    }

    pub fn get_hash_hex_id(&self) -> String {
        format!("{:x}", Sha256::digest(self.id.as_bytes()))
    }
}
