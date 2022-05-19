//! TMC5072 status
use crate::bits::{read_bool_from_bit, write_bool_to_bit};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// SPI Status Bits `SPI_STATUS`
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpiStatus {
    /// GSTAT\[0\] – 1: Signals, that a reset has occurred (clear by reading GSTAT)
    pub reset_flag: bool,
    /// GSTAT\[1\] – 1: Signals driver 1 driver error (clear by reading GSTAT)
    pub driver_error1: bool,
    /// GSTAT\[2\] – 1: Signals driver 2 driver error (clear by reading GSTAT)
    pub driver_error2: bool,
    /// RAMP_STAT1\[8\] – 1: Signals motor 1 has reached its target velocity
    pub velocity_reached1: bool,
    /// RAMP_STAT2\[8\] – 1: Signals motor 2 has reached its target velocity
    pub velocity_reached2: bool,
    /// RAMP_STAT1\[0\] – 1: Signals motor 1 stop left switch status
    pub status_stop_l1: bool,
    /// RAMP_STAT2\[0\] – 1: Signals motor 2 stop left switch status
    pub status_stop_l2: bool,
}

impl Default for SpiStatus {
    fn default() -> Self {
        Self::from(0u8)
    }
}

impl From<u8> for SpiStatus {
    fn from(data: u8) -> Self {
        Self {
            reset_flag: read_bool_from_bit(data, 0),
            driver_error1: read_bool_from_bit(data, 1),
            driver_error2: read_bool_from_bit(data, 2),
            velocity_reached1: read_bool_from_bit(data, 3),
            velocity_reached2: read_bool_from_bit(data, 4),
            status_stop_l1: read_bool_from_bit(data, 5),
            status_stop_l2: read_bool_from_bit(data, 6),
        }
    }
}

impl From<SpiStatus> for u8 {
    fn from(data: SpiStatus) -> u8 {
        let mut value = 0;
        write_bool_to_bit(&mut value, 0, data.reset_flag);
        write_bool_to_bit(&mut value, 1, data.driver_error1);
        write_bool_to_bit(&mut value, 2, data.driver_error2);
        write_bool_to_bit(&mut value, 3, data.velocity_reached1);
        write_bool_to_bit(&mut value, 4, data.velocity_reached2);
        write_bool_to_bit(&mut value, 5, data.status_stop_l1);
        write_bool_to_bit(&mut value, 6, data.status_stop_l2);
        value
    }
}
