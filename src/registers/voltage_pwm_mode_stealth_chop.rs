//! Voltage PWM mode stealthChop

use super::Register;
use crate::bits::{read_bool_from_bit, read_from_bit, write_bool_to_bit, write_from_bit};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// PWMCONF: Voltage PWM mode chopper configuration
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PwmConf<const M: u8> {
    /// PWM_ AMPL: User defined amplitude
    ///
    /// pwm_autoscale=false
    ///
    /// User defined PWM amplitude
    ///
    /// The resulting amplitude (0..255) is set by this value.
    ///
    /// pwm_autoscale=true
    ///
    /// User defined maximum PWM amplitude when switching back from current chopper mode to voltage PWM mode (switch over velocity defined by TPWMTHRS).
    /// Do not set too low values, as the regulation cannot measure the current when the actual PWM value goes below a setting specific value.
    ///
    /// Settings above 0x40 recommended.
    pub pwm_ampl: u8,
    /// PWM_GRAD: User defined regulation loop gradient (bits 15..12 currently unused, set to 0)
    ///
    /// pwm_autoscale=false
    /// - 0: stealthChop disabled
    /// - 1..15: stealthChop enabled (the actual value is not used)
    ///
    /// pwm_autoscale=true
    /// - 0: stealthChop disabled
    /// - 1..15: User defined maximum PWM amplitude change per half wave (1 to 15)
    pub pwm_grad: u8,
    /// pwm_freq: PWM frequency selection
    /// - %00: fPWM=2/1024 fCLK
    /// - %01: fPWM=2/683 fCLK
    /// - %10: fPWM=2/512 fCLK
    /// - %11: fPWM=2/410 fCLK
    pub pwm_freq: u8,
    /// pwm_autoscale: PWM automatic amplitude scaling
    /// - false: User defined PWM amplitude. The current settings have no influence.
    /// - true: Enable automatic current control
    ///
    /// Attention: When using a user defined sine wave table, the amplitude of this sine wave table should not be less than 244.
    /// Best results are obtained with 247 to 252 as peak values.
    pub pwm_autoscale: bool,
    /// freewheel: Allows different standstill modes
    ///
    /// Stand still option when motor current setting is zero (I_HOLD=0).
    /// - %00: Normal operation
    /// - %01: Freewheeling
    /// - %10: Coil shorted using LS drivers
    /// - %11: Coil shorted using HS drivers
    pub freewheel: u8,
}

impl<const M: u8> Default for PwmConf<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for PwmConf<M> {
    fn from(data: u32) -> Self {
        Self {
            pwm_ampl: read_from_bit(data, 0, 0xff) as u8,
            pwm_grad: read_from_bit(data, 8, 0xff) as u8,
            pwm_freq: read_from_bit(data, 16, 0x03) as u8,
            pwm_autoscale: read_bool_from_bit(data, 18),
            freewheel: read_from_bit(data, 20, 0x03) as u8,
        }
    }
}

impl<const M: u8> From<PwmConf<M>> for u32 {
    fn from(data: PwmConf<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xff, data.pwm_ampl as u32);
        write_from_bit(&mut value, 8, 0xff, data.pwm_grad as u32);
        write_from_bit(&mut value, 16, 0x03, data.pwm_freq as u32);
        write_bool_to_bit(&mut value, 18, data.pwm_autoscale);
        write_from_bit(&mut value, 20, 0x03, data.freewheel as u32);
        value
    }
}

impl Register for PwmConf<0> {
    fn addr() -> u8 {
        0x10
    }
}
impl Register for PwmConf<1> {
    fn addr() -> u8 {
        0x18
    }
}

#[cfg(test)]
mod pwm_conf {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(PwmConf::<1> {
                pwm_autoscale: true,
                pwm_freq: 0,
                pwm_ampl: 200,
                pwm_grad: 1,
                ..Default::default()
            }),
            0x000401C8
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            PwmConf::<1>::from(0x000401C8),
            PwmConf::<1> {
                pwm_autoscale: true,
                pwm_freq: 0,
                pwm_ampl: 200,
                pwm_grad: 1,
                ..Default::default()
            },
        )
    }
}

/// PWM_STATUS: Actual PWM scaler
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PwmStatus<const M: u8> {
    /// Actual PWM scaler (255=max. Voltage)
    pub pwm_status: u8,
}

impl<const M: u8> Default for PwmStatus<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for PwmStatus<M> {
    fn from(data: u32) -> Self {
        Self {
            pwm_status: read_from_bit(data, 0, 0xff) as u8,
        }
    }
}

impl<const M: u8> From<PwmStatus<M>> for u32 {
    fn from(data: PwmStatus<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xff, data.pwm_status as u32);
        value
    }
}

impl Register for PwmStatus<0> {
    fn addr() -> u8 {
        0x11
    }
}
impl Register for PwmStatus<1> {
    fn addr() -> u8 {
        0x19
    }
}

#[cfg(test)]
mod pwm_status {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(PwmStatus::<1> {
                pwm_status: 0x66,
                ..Default::default()
            }),
            0x00000066
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            PwmStatus::<1>::from(0x00000066),
            PwmStatus::<1> {
                pwm_status: 0x66,
                ..Default::default()
            },
        )
    }
}
