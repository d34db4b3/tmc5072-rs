//! SPI Error handling

use crate::status::SpiStatus;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Wrapper for SPI communication Result
pub type SpiResult<T, SPI, CS> = Result<SpiOk<T>, SpiError<SPI, CS>>;

/// Bundles the SPI status register and the actual read data
pub struct SpiOk<T> {
    /// Spi Status register
    pub status: SpiStatus,
    /// Actual transfer data
    pub data: T,
}

impl<T> SpiOk<T> {
    /// Maps an SpiOk<T> to SpiOk<U> by applying a function to a contained value.
    pub fn map<U, F>(self, f: F) -> SpiOk<U>
    where
        F: Fn(T) -> U,
    {
        SpiOk {
            status: self.status,
            data: f(self.data),
        }
    }
}

impl SpiOk<u32> {
    /// Parses TMC5072 SPI buffer into the SPI status and u32 data
    pub fn from_buffer(buffer: &[u8; 5]) -> Self {
        Self {
            status: SpiStatus::from(buffer[0]),
            data: ((buffer[1] as u32) << 24u32)
                | ((buffer[2] as u32) << 16u32)
                | ((buffer[3] as u32) << 8u32)
                | buffer[4] as u32,
        }
    }
}

impl SpiOk<()> {
    /// Only parses the SPI status from a TMC5072 SPI buffer
    pub fn from_buffer(buffer: &[u8; 5]) -> Self {
        Self {
            status: SpiStatus::from(buffer[0]),
            data: (),
        }
    }
}

/// Errors that can occur while using SPI
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SpiError<SPI, CS> {
    /// SPI communication error
    SpiError(SPI),
    /// Chip Select pin error
    CSError(CS),
}
