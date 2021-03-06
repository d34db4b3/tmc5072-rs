//! TMC5072 driver
//!
//! Dual controller/driver for up to two 2-phase bipolar stepper motors.
//! No-noise stepper operation. Integrated motion controller and encoder counter. SPI, UART (single wire) and Step/Dir
//!
//! # Description:
//!
//! The TMC5072 is a dual high performance stepper motor controller and driver IC with serial communication interfaces.
//! It combines flexible ramp generators for automatic target positioning with industries' most advanced stepper motor drivers.
//! Based on TRINAMICs sophisticated stealthChop chopper, the driver ensures absolutely noiseless operation combined with maximum efficiency and best motor torque.
//! High integration, high energy efficiency and a small form factor enable miniaturized and scalable systems for cost effective solutions.
//! The complete solution reduces learning curve to a minimum while giving best performance in class.
//!
//!
//! # Key Concepts:
//!
//! The TMC5072 implements several advanced features which are exclusive to TRINAMIC products.
//! These features contribute toward greater precision, greater energy efficiency, higher reliability, smoother motion, and cooler operation in many stepper motor applications.
//!
//! stealthChop™: No-noise, high-precision chopper algorithm for inaudible motion and inaudible standstill of the motor.
//!
//! dcStep™: Load dependent speed control. The motor moves as fast as possible and never loses a step.
//!
//! stallGuard2™: High-precision load measurement using the back EMF on the motor coils.
//!
//! coolStep™: Load-adaptive current control which reduces energy consumption by as much as 75%.
//!
//! spreadCycle™: High-precision chopper algorithm available as an alternative to the traditional constant off-time algorithm.
//!
//! sixPoint™: Fast and precise positioning using a hardware ramp generator with a set of four acceleration / deceleration settings. Quickest response due to dedicated hardware.
//!
//! In addition to these performance enhancements, TRINAMIC motor drivers offer safeguards to detect and protect against shorted outputs,
//! output open-circuit, overtemperature, and undervoltage conditions for enhancing safety and recovery from equipment malfunctions.
//!
//! # Documents
//!
//! - [TCM5072 Datasheet (Trinamics)](https://www.trinamic.com/fileadmin/assets/Products/ICs_Documents/TMC5072_datasheet.pdf)

//! # Example
//!
//! ```rust
//! # use tmc5072::{Tmc5072, spi::{SpiError, SpiOk}, InitError, registers::ramp_generator_register::XActual};
//! #
//! # struct SpiMock;
//! # impl embedded_hal::blocking::spi::Transfer<u8> for SpiMock {
//! #     type Error = ();
//! #
//! #     fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
//! #         words[0] = 0x00;
//! #         words[1] = 0x10;
//! #         words[2] = 0x00;
//! #         words[3] = 0x00;
//! #         words[4] = 0x00;
//! #         Ok(words)
//! #     }
//! # }
//! # struct CsMock;
//! # impl embedded_hal::digital::v2::OutputPin for CsMock {
//! #     type Error = ();
//! #
//! #     fn set_low(&mut self) -> Result<(), Self::Error> {
//! #         Ok(())
//! #     }
//! #
//! #     fn set_high(&mut self) -> Result<(), Self::Error> {
//! #         Ok(())
//! #     }
//! # }
//! #
//! # #[derive(Debug)]
//! # struct Error;
//! # impl<SPI, CS> From<InitError<SPI, CS>> for Error {
//! #     fn from(e: InitError<SPI, CS>) -> Self {
//! #         Error
//! #     }
//! # }
//! # impl<SPI, CS> From<SpiError<SPI, CS>> for Error {
//! #     fn from(e: SpiError<SPI, CS>) -> Self {
//! #         Error
//! #     }
//! # }
//! #
//! # fn main() -> Result<(), Error> {
//! #    let mut spi = SpiMock;
//! #    let cs = CsMock;
//! let mut tmc5072 = Tmc5072::new(&mut spi, cs)?;
//! let spi_ok: SpiOk<XActual<0>> = tmc5072.read_register::<XActual<0>, _>(&mut spi)?;
//! let x_actual: i32 = spi_ok.data.x_actual;
//! #    Ok(())
//! # }
//! ```
//!
//! # Warnings
//!
//! Not production ready yet, API could change in the future
//!
//! This crate only implements raw register access

#![no_std]
#![deny(missing_docs)]

#[doc(hidden)]
mod bits;
pub mod registers;
pub mod spi;
pub mod status;

use embedded_hal as hal;
use hal::{blocking::spi::Transfer, digital::v2::OutputPin};
use registers::{Register, IC_VERSION, READ_FLAG, WRITE_FLAG};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use spi::{SpiError, SpiOk, SpiResult};

/// TMC5072 initialisation error
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InitError<SPI, CS> {
    /// SPI bus error
    SpiError(SpiError<SPI, CS>),
    /// IC Version error (should be 0x10)
    VersionError(u8),
}

impl<SPI, CS> From<SpiError<SPI, CS>> for InitError<SPI, CS> {
    fn from(e: SpiError<SPI, CS>) -> Self {
        InitError::SpiError(e)
    }
}

/// TMC5072 driver
pub struct Tmc5072<CS> {
    cs: CS,
    buffer: [u8; 5],
}

impl<CS: OutputPin> Tmc5072<CS> {
    /// Creates a new Tmc5072 driver from an SPI interface and a Chip Select pin
    pub fn new<SPI: Transfer<u8>>(
        spi: &mut SPI,
        cs: CS,
    ) -> Result<Self, InitError<SPI::Error, CS::Error>> {
        let mut tmc5072 = Tmc5072 { buffer: [0; 5], cs };
        // check IC version
        let version = tmc5072
            .read_register::<registers::general_configuration_register::Input, _>(spi)?
            .data
            .version;
        if version != IC_VERSION {
            return Err(InitError::VersionError(version));
        };
        Ok(tmc5072)
    }
    /// Read a typed register from the Tmc5072
    pub fn read_register<'a, R, SPI: Transfer<u8>>(
        &mut self,
        spi: &mut SPI,
    ) -> SpiResult<R, SPI::Error, CS::Error>
    where
        R: Register,
        u32: From<R>,
    {
        self.read_raw(R::addr(), spi).map(|x| x.map(|x| R::from(x)))
    }
    /// Write a typed register from the Tmc5072
    pub fn write_register<'a, R, SPI: Transfer<u8>>(
        &mut self,
        r: R,
        spi: &mut SPI,
    ) -> SpiResult<(), SPI::Error, CS::Error>
    where
        R: Register,
        u32: From<R>,
    {
        let data = u32::from(r);
        self.write_raw(R::addr(), data, spi)
    }
    // TODO: optimize read (multiple commands (maybe iterators ?) to divide transfers by 2)
    /// Read a raw register from the Tmc5072
    pub fn read_raw<SPI: Transfer<u8>>(
        &mut self,
        addr: u8,
        spi: &mut SPI,
    ) -> SpiResult<u32, SPI::Error, CS::Error> {
        self.buffer[0] = READ_FLAG | addr;
        self.buffer[1] = 0;
        self.buffer[2] = 0;
        self.buffer[3] = 0;
        self.buffer[4] = 0;
        self.cs.set_low().map_err(SpiError::CSError)?;
        // send read command
        spi.transfer(&mut self.buffer).map_err(SpiError::SpiError)?;
        self.cs.set_high().map_err(SpiError::CSError)?;
        // received previous command junk ignore
        self.buffer[0] = READ_FLAG | addr;
        self.cs.set_low().map_err(SpiError::CSError)?;
        // repeat command to get result
        spi.transfer(&mut self.buffer).map_err(SpiError::SpiError)?;
        self.cs.set_high().map_err(SpiError::CSError)?;
        Ok(SpiOk::<u32>::from_buffer(&self.buffer))
    }
    /// Write a raw register from the Tmc5072
    pub fn write_raw<SPI: Transfer<u8>>(
        &mut self,
        addr: u8,
        data: u32,
        spi: &mut SPI,
    ) -> SpiResult<(), SPI::Error, CS::Error> {
        self.buffer[0] = WRITE_FLAG | addr;
        self.buffer[1] = (data >> 24) as u8;
        self.buffer[2] = (data >> 16) as u8;
        self.buffer[3] = (data >> 8) as u8;
        self.buffer[4] = data as u8;
        self.cs.set_low().map_err(SpiError::CSError)?;
        // send write command
        spi.transfer(&mut self.buffer).map_err(SpiError::SpiError)?;
        self.cs.set_high().map_err(SpiError::CSError)?;
        Ok(SpiOk::<()>::from_buffer(&self.buffer))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::registers::{
        general_configuration_register::GConf,
        motor_driver_register::ChopConf,
        ramp_generator_driver_feature_control_register::{IHoldIRun, VCoolThrs, VHigh},
        ramp_generator_register::{AMax, DMax, RampMode, VMax, VStop, XActual, A1, D1, V1},
        voltage_pwm_mode_stealth_chop::PwmConf,
    };

    #[test]
    fn test() {
        assert_eq!(GConf::addr() | WRITE_FLAG, 0x80);
        assert_eq!(
            u32::from(GConf {
                poscmp_enable: true,
                ..Default::default()
            }),
            0x00000008
        );
        assert_eq!(ChopConf::<0>::addr() | WRITE_FLAG, 0xEC);
        assert_eq!(
            u32::from(ChopConf::<0> {
                toff: 5,
                hstrt: 4,
                hend: 1,
                tbl: 2,
                chm: false,
                ..Default::default()
            }),
            0x000100C5
        );
        assert_eq!(IHoldIRun::<0>::addr() | WRITE_FLAG, 0xB0);
        assert_eq!(
            u32::from(IHoldIRun::<0> {
                i_hold: 5,
                i_run: 31,
                i_hold_delay: 1,
                ..Default::default()
            }),
            0x00011F05
        );
        assert_eq!(PwmConf::<0>::addr() | WRITE_FLAG, 0x90);
        assert_eq!(
            u32::from(PwmConf::<0> {
                pwm_autoscale: true,
                pwm_freq: 0,
                pwm_ampl: 200,
                pwm_grad: 1,
                ..Default::default()
            }),
            0x000401C8
        );
        assert_eq!(VHigh::<0>::addr() | WRITE_FLAG, 0xB2);
        assert_eq!(
            u32::from(VHigh::<0> {
                v_high: 400000,
                ..Default::default()
            }),
            0x00061A80
        );
        assert_eq!(VCoolThrs::<0>::addr() | WRITE_FLAG, 0xB1);
        assert_eq!(
            u32::from(VCoolThrs::<0> {
                v_cool_thrs: 30000,
                ..Default::default()
            }),
            0x00007530
        );
        assert_eq!(A1::<0>::addr() | WRITE_FLAG, 0xA4);
        assert_eq!(
            u32::from(A1::<0> {
                a1: 1000,
                ..Default::default()
            }),
            0x000003E8
        );
        assert_eq!(V1::<0>::addr() | WRITE_FLAG, 0xA5);
        assert_eq!(
            u32::from(V1::<0> {
                v1: 50000,
                ..Default::default()
            }),
            0x0000C350
        );
        assert_eq!(AMax::<0>::addr() | WRITE_FLAG, 0xA6);
        assert_eq!(
            u32::from(AMax::<0> {
                a_max: 500,
                ..Default::default()
            }),
            0x000001F4
        );
        assert_eq!(VMax::<0>::addr() | WRITE_FLAG, 0xA7);
        assert_eq!(
            u32::from(VMax::<0> {
                v_max: 200000,
                ..Default::default()
            }),
            0x00030D40
        );
        assert_eq!(DMax::<0>::addr() | WRITE_FLAG, 0xA8);
        assert_eq!(
            u32::from(DMax::<0> {
                d_max: 700,
                ..Default::default()
            }),
            0x000002BC
        );
        assert_eq!(D1::<0>::addr() | WRITE_FLAG, 0xAA);
        assert_eq!(
            u32::from(D1::<0> {
                d1: 1400,
                ..Default::default()
            }),
            0x00000578
        );
        assert_eq!(VStop::<0>::addr() | WRITE_FLAG, 0xAB);
        assert_eq!(
            u32::from(VStop::<0> {
                v_stop: 10,
                ..Default::default()
            }),
            0x0000000A
        );
        assert_eq!(RampMode::<0>::addr() | WRITE_FLAG, 0xA0);
        assert_eq!(
            u32::from(RampMode::<0> {
                ramp_mode: 0,
                ..Default::default()
            }),
            0x00000000
        );
        assert_eq!(XActual::<0>::addr() | READ_FLAG, 0x21);
    }
}
