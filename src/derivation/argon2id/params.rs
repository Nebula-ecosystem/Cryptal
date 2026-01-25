#[derive(Clone, Debug)]
pub struct Argon2Params {
    pub mem_kib: u32,
    pub time: u32,
    pub lanes: u32,
    pub tag_len: usize,
}

#[derive(Debug)]
pub enum Argon2ParamError {
    MemoryTooSmall,
    MemoryNotMultipleOfLanes,
    TooFewLanes,
    TooFewPasses,
    TagLengthInvalid,
}

impl Argon2Params {
    pub(crate) fn validate(&self) -> Result<(), Argon2ParamError> {
        if self.lanes < 1 {
            return Err(Argon2ParamError::TooFewLanes);
        }

        if self.time < 1 {
            return Err(Argon2ParamError::TooFewPasses);
        }

        if self.mem_kib < 8 * self.lanes {
            return Err(Argon2ParamError::MemoryTooSmall);
        }

        if !self.mem_kib.is_multiple_of(2 * self.lanes) {
            return Err(Argon2ParamError::MemoryNotMultipleOfLanes);
        }

        if self.tag_len < 4 || self.tag_len > 1024 {
            return Err(Argon2ParamError::TagLengthInvalid);
        }

        Ok(())
    }
}

impl Default for Argon2Params {
    fn default() -> Self {
        Self {
            mem_kib: 64 * 1024,
            time: 3,
            lanes: 1,
            tag_len: 32,
        }
    }
}
