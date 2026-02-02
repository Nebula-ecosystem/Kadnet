use cryptal::derivation::{Argon2Error, Argon2Params, argon2id};
use cryptal::hash::sha256;
use cryptal::primitives::U256;

#[derive(Debug)]
pub enum IdError {
    HashError(Argon2Error),
    VersionError,
}

pub(crate) fn generate_id(public_key: [u8; 32], version: usize) -> Result<U256, IdError> {
    match version {
        1 => {
            let params = Argon2Params {
                mem_kib: 256,
                time: 3,
                lanes: 4,
                tag_len: 32,
                secret: None,
                associated_data: None,
            };

            let argon = argon2id(&public_key, b"version_1", &params).map_err(IdError::HashError)?;

            Ok(sha256(&argon))
        }
        _ => Err(IdError::VersionError),
    }
}
