//! SPI Error handling

use crate::status::SpiStatus;

/// Wrapper for SPI communication Result
pub type SpiResult<T, SPI> = Result<SpiOk<T>, SPI>;

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
            data: u32::from_be_bytes(buffer[1..].try_into().unwrap()),
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
