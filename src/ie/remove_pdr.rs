use crate::ie::pdr_id::PdrId;
use crate::ie::{Ie, IeType};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovePdr {
    pub pdr_id: PdrId,
}

impl RemovePdr {
    pub fn new(pdr_id: PdrId) -> Self {
        RemovePdr { pdr_id }
    }

    pub fn marshal(&self) -> Vec<u8> {
        self.pdr_id.marshal().to_vec()
    }

    pub fn to_ie(self) -> Ie {
        Ie::new(IeType::RemovePdr, self.marshal())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        Ok(RemovePdr {
            pdr_id: PdrId::unmarshal(data)?,
        })
    }
}
