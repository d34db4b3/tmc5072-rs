#![no_std]
use embedded_hal as hal;
use hal::digital::OutputPin;
use hal::spi::SpiBus;
use registers::{Register, IC_VERSION, READ_FLAG, WRITE_FLAG};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use spi::{SpiError, SpiOk, SpiResult};

#[doc(hidden)]
mod bits;
pub mod registers;
pub mod spi;
pub mod status;

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
    /// Creates a new Tmc5072 driver from an SPI bus and a Chip Select pin
    pub fn new<SPI: SpiBus<u8>>(
        spi: &mut SPI,
        cs: CS,
    ) -> Result<Self, InitError<SPI::Error, CS::Error>> {
        let mut tmc5072 = Tmc5072 { buffer: [0; 5], cs };
        // check IC version
        let version = tmc5072
            .read_register::<registers::general_configuration_register::Input, SPI>(spi)?
            .data
            .version;
        if version != IC_VERSION {
            return Err(InitError::VersionError(version));
        };
        Ok(tmc5072)
    }

    /// Read a typed register from the Tmc5072
    pub fn read_register<R, SPI: SpiBus<u8>>(
        &mut self,
        spi: &mut SPI,
    ) -> SpiResult<R, SPI::Error, CS::Error>
    where
        R: Register,
        u32: From<R>,
    {
        self.read_raw(R::addr(), spi).map(|x| x.map(|x| R::from(x)))
    }

    /// Write a typed register to the Tmc5072
    pub fn write_register<R, SPI: SpiBus<u8>>(
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

    /// Read a raw register from the Tmc5072
    pub fn read_raw<SPI: SpiBus<u8>>(
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
        // send read command, discard returned junk
        spi.transfer_in_place(&mut self.buffer).map_err(SpiError::SpiError)?;
        self.cs.set_high().map_err(SpiError::CSError)?;

        // now actually read the register
        self.buffer[0] = READ_FLAG | addr;
        self.cs.set_low().map_err(SpiError::CSError)?;
        spi.transfer_in_place(&mut self.buffer).map_err(SpiError::SpiError)?;
        self.cs.set_high().map_err(SpiError::CSError)?;

        Ok(SpiOk::<u32>::from_buffer(&self.buffer))
    }

    /// Write a raw register to the Tmc5072
    pub fn write_raw<SPI: SpiBus<u8>>(
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
        spi.transfer_in_place(&mut self.buffer).map_err(SpiError::SpiError)?;
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
        ramp_generator_register::{A1, AMax, D1, DMax, RampMode, V1, VMax, VStop, XActual},
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
