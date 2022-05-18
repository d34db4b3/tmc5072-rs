//! General Configuration Registers
//!
//! These registers contain
//! - global configuration
//! - global status flags
//! - slave address configuration
//! - and I/O configuration

use super::Register;
use crate::bits::{read_bool_from_bit, read_from_bit, write_bool_to_bit, write_from_bit};

#[derive(Debug, Clone, Copy, PartialEq)]
/// GCONF: Global configuration flags
pub struct GConf {
    /// single_driver:
    /// - false: Two motors can be operated.
    /// - true: Single motor, double current operation - driver 2
    ///
    /// outputs are identical to driver 1, all driver 2
    /// related controls are unused in this mode.
    ///
    /// Attention: Set correctly before driver enable!
    pub single_diver: bool,
    /// stepdir1_enable:
    /// - false: Motor 1 is driven by internal ramp generator 1.
    /// - true: External control of motor 1 using STEP1 and DIR1 - ramp generator 1 is not used.
    pub stepdir1_enable: bool,
    /// stepdir2_enable:
    /// - false: Motor 2 is driven by internal ramp generator 2.
    /// - true: External control of motor 2 using STEP2 and DIR2 - ramp generator 2 is not used.
    pub stepdir2_enable: bool,
    /// poscmp_enable:
    /// - false: Encoder 1 A and B inputs are mapped.
    /// - true: Position compare pulse (PP) and interrupt output (INT) are available, Encoder 1 is unused.
    pub poscmp_enable: bool,
    /// enc1_refsel:
    /// - false: N channel 1 mapped depending on interface to SWIOP (if SW_SEL=0) or IO0 (if SW_SEL=1).
    /// - true: N channel 1 mapped to REFL1
    pub enc1_refsel: bool,
    /// enc2_enable:
    /// - false: Right reference switches are available.
    /// - true: Encoder 2 A and B signals are mapped to REFR1 and REFR2 inputs.
    pub enc2_enable: bool,
    /// enc2_refsel:
    /// - false: N channel 2 mapped depending on interface to SWION (if SW_SEL=0) or IO1 (if SW_SEL=1).
    /// - true: N channel 2 mapped to REFL2.
    pub enc2_refsel: bool,
    /// test_mode:
    /// - false: Normal operation
    /// - true: Enable analog test output on pin REFR2 SLAVEADDR selects the function of REFR2: 0..4: T120, DAC1, VDDH1, DAC2, VDDH2
    ///
    /// Attention: Not for user, set to false for normal operation!
    pub test_mode: bool,
    /// shaft1:
    /// - true: Inverse motor 1 direction
    pub shaft1: bool,
    /// shaft2:
    /// - true: Inverse motor 2 direction
    pub shaft2: bool,
    /// lock_gconf:
    /// - true: GCONF is locked against further write access.
    pub lock_gconf: bool,
    /// dc_sync:
    /// - true: Synchronizes both motors, when both are operated in dcStep mode. The slower motor will slow down the other motor, too.
    pub dc_sync: bool,
}

impl Default for GConf {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for GConf {
    fn from(data: u32) -> Self {
        Self {
            single_diver: read_bool_from_bit(data, 0),
            stepdir1_enable: read_bool_from_bit(data, 1),
            stepdir2_enable: read_bool_from_bit(data, 2),
            poscmp_enable: read_bool_from_bit(data, 3),
            enc1_refsel: read_bool_from_bit(data, 4),
            enc2_enable: read_bool_from_bit(data, 5),
            enc2_refsel: read_bool_from_bit(data, 6),
            test_mode: read_bool_from_bit(data, 7),
            shaft1: read_bool_from_bit(data, 8),
            shaft2: read_bool_from_bit(data, 9),
            lock_gconf: read_bool_from_bit(data, 10),
            dc_sync: read_bool_from_bit(data, 11),
        }
    }
}

impl From<GConf> for u32 {
    fn from(data: GConf) -> u32 {
        let mut value = 0;
        write_bool_to_bit(&mut value, 0, data.single_diver);
        write_bool_to_bit(&mut value, 1, data.stepdir1_enable);
        write_bool_to_bit(&mut value, 2, data.stepdir2_enable);
        write_bool_to_bit(&mut value, 3, data.poscmp_enable);
        write_bool_to_bit(&mut value, 4, data.enc1_refsel);
        write_bool_to_bit(&mut value, 5, data.enc2_enable);
        write_bool_to_bit(&mut value, 6, data.enc2_refsel);
        write_bool_to_bit(&mut value, 7, data.test_mode);
        write_bool_to_bit(&mut value, 8, data.shaft1);
        write_bool_to_bit(&mut value, 9, data.shaft2);
        write_bool_to_bit(&mut value, 10, data.lock_gconf);
        write_bool_to_bit(&mut value, 11, data.dc_sync);
        value
    }
}

impl Register for GConf {
    fn addr() -> u8 {
        0x00
    }
}

#[cfg(test)]
mod g_conf {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(GConf {
                poscmp_enable: true,
                ..Default::default()
            }),
            0x00000008
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            GConf::from(0x00000008),
            GConf {
                poscmp_enable: true,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// GSTAT: Global status flags
pub struct GStat {
    /// reset:
    /// - true: Indicates that the IC has been reset since the last read access to GSTAT. All registers have been cleared to reset values.
    pub reset: bool,
    /// drv_err1:
    /// - true: Indicates, that driver 1 has been shut down due to overtemperature or short circuit detection since the last read access.
    /// Read DRV_STATUS1 for details. The flag can only be reset when all error conditions are cleared.
    pub drv_err1: bool,
    /// drv_err2:
    /// - true: Indicates, that driver 2 has been shut down due to overtemperature or short circuit detection since the last read access.
    /// Read DRV_STATUS2 for details. The flag can only be reset when all error conditions are cleared.
    pub drv_err2: bool,
    /// uv_cp:
    /// - true: Indicates an undervoltage on the charge pump. The driver is disabled in this case.
    pub uv_cp: bool,
}

impl Default for GStat {
    fn default() -> Self {
        Self::from(0u32)
    }
}
impl From<u32> for GStat {
    fn from(data: u32) -> Self {
        Self {
            reset: read_bool_from_bit(data, 0),
            drv_err1: read_bool_from_bit(data, 1),
            drv_err2: read_bool_from_bit(data, 2),
            uv_cp: read_bool_from_bit(data, 3),
        }
    }
}
impl From<GStat> for u32 {
    fn from(data: GStat) -> u32 {
        let mut value = 0;
        write_bool_to_bit(&mut value, 0, data.reset);
        write_bool_to_bit(&mut value, 1, data.drv_err1);
        write_bool_to_bit(&mut value, 2, data.drv_err2);
        write_bool_to_bit(&mut value, 3, data.uv_cp);
        value
    }
}

impl Register for GStat {
    fn addr() -> u8 {
        0x01
    }
}

#[cfg(test)]
mod g_stat {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(GStat {
                drv_err1: true,
                drv_err2: true,
                ..Default::default()
            }),
            0x00000006
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            GStat::from(0x00000006),
            GStat {
                drv_err1: true,
                drv_err2: true,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// IFCNT: Interface transmission counter
pub struct IfCnt {
    /// Interface transmission counter. This register becomes incremented with each successful UART interface write access.
    /// It can be read out to check the serial transmission for lost data.
    /// Read accesses do not change the content. Disabled in SPI operation.
    /// The counter wraps around from 255 to 0.
    pub if_cnt: u8,
}

impl Default for IfCnt {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for IfCnt {
    fn from(data: u32) -> Self {
        Self {
            if_cnt: read_from_bit(data, 0, 0xff) as u8,
        }
    }
}
impl From<IfCnt> for u32 {
    fn from(data: IfCnt) -> u32 {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xff, data.if_cnt as u32);
        value
    }
}

impl Register for IfCnt {
    fn addr() -> u8 {
        0x02
    }
}

#[cfg(test)]
mod if_cnt {
    use super::*;

    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(IfCnt {
                if_cnt: 0x66,
                ..Default::default()
            }),
            0x00000066
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            IfCnt::from(0x00000066),
            IfCnt {
                if_cnt: 0x66,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// SLAVECONF
pub struct SlaveConf {
    /// SLAVEADDR:
    /// Sets the address of unit for the UART interface. The address becomes incremented by one when the external address pin NEXTADDR is active.
    ///
    /// Range: 0-253 (254), default=0
    ///
    /// In ring mode, 0 disables forwarding.
    pub slave_addr: u8,
    /// SENDDELAY:
    /// - 0, 1: 8 bit times (not allowed with multiple slaves)
    /// - 2, 3: 3*8 bit times
    /// - 4, 5: 5*8 bit times
    /// - 6, 7: 7*8 bit times
    /// - 8, 9: 9*8 bit times
    /// - 10, 11: 11*8 bit times
    /// - 12, 13: 13*8 bit times
    /// - 14, 15: 15*8 bit times
    pub send_delay: u8,
}

impl Default for SlaveConf {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for SlaveConf {
    fn from(data: u32) -> Self {
        Self {
            slave_addr: read_from_bit(data, 0, 0xff) as u8,
            send_delay: read_from_bit(data, 8, 0x0f) as u8,
        }
    }
}

impl From<SlaveConf> for u32 {
    fn from(data: SlaveConf) -> u32 {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xff, data.slave_addr as u32);
        write_from_bit(&mut value, 8, 0x0f, data.send_delay as u32);
        value
    }
}

impl Register for SlaveConf {
    fn addr() -> u8 {
        0x03
    }
}

#[cfg(test)]
mod slave_conf {
    use super::*;

    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(SlaveConf {
                slave_addr: 0x55,
                send_delay: 8,
                ..Default::default()
            }),
            0x00000855
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            SlaveConf::from(0x00000855),
            SlaveConf {
                slave_addr: 0x55,
                send_delay: 8,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// INPUT: Reads the digital state of all input pins available plus the state of IO pins set to output.
pub struct Input {
    /// io0_in: IO0 polarity
    pub io0: bool,
    /// io1_in: IO1 polarity
    pub io1: bool,
    /// io2_in: IO2 polarity
    pub io2: bool,
    /// io3_in: IO3 polarity
    pub io3: bool,
    /// iop_in: IOP pin polarity (always input in SPI mode)
    pub iop: bool,
    /// ion_in: ION pin polarity (always input in SPI mode)
    pub ion: bool,
    /// nextaddr_in: NEXTADDR pin polarity
    pub next_addr: bool,
    /// drv_enn_in: DRV_ENN pin polarity
    pub drv_enn: bool,
    /// sw_comp_in: UART input comparator (true: IOP voltage is above ION voltage). The accuracy is about 20mV.
    pub sw_comp: bool,
    /// VERSION: 0x10=version of the IC
    /// Identical numbers mean full digital compatibility
    pub version: u8,
}

impl Default for Input {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for Input {
    fn from(data: u32) -> Self {
        Self {
            io0: read_bool_from_bit(data, 0),
            io1: read_bool_from_bit(data, 1),
            io2: read_bool_from_bit(data, 2),
            io3: read_bool_from_bit(data, 3),
            iop: read_bool_from_bit(data, 4),
            ion: read_bool_from_bit(data, 5),
            next_addr: read_bool_from_bit(data, 6),
            drv_enn: read_bool_from_bit(data, 7),
            sw_comp: read_bool_from_bit(data, 8),
            version: read_from_bit(data, 24, 0xff) as u8,
        }
    }
}

impl From<Input> for u32 {
    fn from(data: Input) -> u32 {
        let mut value = 0;
        write_bool_to_bit(&mut value, 0, data.io0);
        write_bool_to_bit(&mut value, 1, data.io1);
        write_bool_to_bit(&mut value, 2, data.io2);
        write_bool_to_bit(&mut value, 3, data.io3);
        write_bool_to_bit(&mut value, 4, data.iop);
        write_bool_to_bit(&mut value, 5, data.ion);
        write_bool_to_bit(&mut value, 6, data.next_addr);
        write_bool_to_bit(&mut value, 7, data.drv_enn);
        write_bool_to_bit(&mut value, 8, data.sw_comp);
        write_from_bit(&mut value, 24, 0xff, data.version as u32);
        value
    }
}

impl Register for Input {
    fn addr() -> u8 {
        0x04
    }
}

#[cfg(test)]
mod input {
    use super::*;

    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(Input {
                version: 0x10,
                ..Default::default()
            }),
            0x10000000
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            Input::from(0x10000000),
            Input {
                version: 0x10,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// OUTPUT: Sets the IO output pin polarity and data direction.
pub struct Output {
    /// io0_out: IO0 output polarity
    pub io0: bool,
    /// io1_out: IO1 output polarity
    pub io1: bool,
    /// io2_out: IO2 output polarity
    pub io2: bool,
    /// ioddr0 (IO0: false=input, true=output)
    pub io_ddr0: bool,
    /// ioddr1 (IO1: false=input, true=output)
    pub io_ddr1: bool,
    /// ioddr2 (IO2: false=input, true=output)
    pub io_ddr2: bool,
}

impl Default for Output {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for Output {
    fn from(data: u32) -> Self {
        Self {
            io0: read_bool_from_bit(data, 0),
            io1: read_bool_from_bit(data, 1),
            io2: read_bool_from_bit(data, 2),
            io_ddr0: read_bool_from_bit(data, 8),
            io_ddr1: read_bool_from_bit(data, 9),
            io_ddr2: read_bool_from_bit(data, 10),
        }
    }
}

impl From<Output> for u32 {
    fn from(data: Output) -> u32 {
        let mut value = 0;
        write_bool_to_bit(&mut value, 0, data.io0);
        write_bool_to_bit(&mut value, 1, data.io1);
        write_bool_to_bit(&mut value, 2, data.io2);
        write_bool_to_bit(&mut value, 8, data.io_ddr0);
        write_bool_to_bit(&mut value, 9, data.io_ddr1);
        write_bool_to_bit(&mut value, 10, data.io_ddr2);
        value
    }
}

impl Register for Output {
    fn addr() -> u8 {
        0x04
    }
}

#[cfg(test)]
mod output {
    use super::*;

    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(Output {
                io_ddr0: true,
                io_ddr1: false,
                io_ddr2: true,
                ..Default::default()
            }),
            0x00000500
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            Output::from(0x00000500),
            Output {
                io_ddr0: true,
                io_ddr1: false,
                io_ddr2: true,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// X_COMPARE: Position comparison register for motor 1 position strobe.
/// Activate poscmp_enable to get position pulse on output PP.
///
/// XACTUAL = X_COMPARE:
/// - Output PP becomes high. It returns to a low state, if the positions mismatch.
pub struct XCompare {
    /// Position comparison register for motor 1 position strobe.
    pub x_compare: u32,
}

impl Default for XCompare {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for XCompare {
    fn from(data: u32) -> Self {
        Self {
            x_compare: read_from_bit(data, 0, 0xffffffff),
        }
    }
}

impl From<XCompare> for u32 {
    fn from(data: XCompare) -> u32 {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.x_compare);
        value
    }
}

impl Register for XCompare {
    fn addr() -> u8 {
        0x05
    }
}

#[cfg(test)]
mod x_compare {
    use super::*;

    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(XCompare {
                x_compare: 0x5566,
                ..Default::default()
            }),
            0x00005566
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            XCompare::from(0x00005566),
            XCompare {
                x_compare: 0x5566,
                ..Default::default()
            },
        )
    }
}
