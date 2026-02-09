use kona_proof::HintType;

// Add your HintWrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HintWrapper {
    Standard(HintType),
    CelestiaDA,
}

impl From<HintWrapper> for u8 {
    fn from(v: HintWrapper) -> Self {
        match v {
            HintWrapper::Standard(h) => h.into(),
            HintWrapper::CelestiaDA => 0xda,
        }
    }
}

impl TryFrom<u8> for HintWrapper {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xda => Ok(HintWrapper::CelestiaDA),
            other => Ok(HintWrapper::Standard(HintType::try_from(other)?)),
        }
    }
}
