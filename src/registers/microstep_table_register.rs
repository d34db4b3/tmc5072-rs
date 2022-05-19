//! Microstep Table Registers
//!
//! MSLUT: Each bit gives the difference between entry x and entry x+1 when combined with the corresponding MSLUTSEL W bits:
//! - false: W= %00: -1, %01: +0, %10: +1, %11: +2
//! - true: W= %00: +0, %01: +1, %10: +2, %11: +3
//! This is the differential coding for the first quarter of a wave.
//! Start values for CUR_A and CUR_B are stored for MSCNT position 0 in START_SIN and START_SIN90.

use super::Register;
use crate::bits::{read_from_bit, write_from_bit};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// MSLUT\[0\]: Microstep table entries 0..31
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MsLut0 {
    /// Microstep table entries 0..31
    pub ms_lut0: u32,
}

impl Default for MsLut0 {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for MsLut0 {
    fn from(data: u32) -> Self {
        Self {
            ms_lut0: read_from_bit(data, 0, 0xffffffff) as u32,
        }
    }
}

impl From<MsLut0> for u32 {
    fn from(data: MsLut0) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.ms_lut0);
        value
    }
}

impl Register for MsLut0 {
    fn addr() -> u8 {
        0x60
    }
}

#[cfg(test)]
mod ms_lut0 {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(MsLut0 {
                ms_lut0: 0xF0F0F0F0,
                ..Default::default()
            }),
            0xF0F0F0F0
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            MsLut0::from(0xF0F0F0F0),
            MsLut0 {
                ms_lut0: 0xF0F0F0F0,
                ..Default::default()
            },
        )
    }
}

/// MSLUT\[1\]: Microstep table entries 32..63
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MsLut1 {
    /// Microstep table entries 32..63
    pub ms_lut1: u32,
}

impl Default for MsLut1 {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for MsLut1 {
    fn from(data: u32) -> Self {
        Self {
            ms_lut1: read_from_bit(data, 0, 0xffffffff) as u32,
        }
    }
}

impl From<MsLut1> for u32 {
    fn from(data: MsLut1) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.ms_lut1);
        value
    }
}

impl Register for MsLut1 {
    fn addr() -> u8 {
        0x61
    }
}

#[cfg(test)]
mod ms_lut1 {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(MsLut1 {
                ms_lut1: 0xF0F0F0F0,
                ..Default::default()
            }),
            0xF0F0F0F0
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            MsLut1::from(0xF0F0F0F0),
            MsLut1 {
                ms_lut1: 0xF0F0F0F0,
                ..Default::default()
            },
        )
    }
}
/// MSLUT\[2\]: Microstep table entries 64..95
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MsLut2 {
    /// Microstep table entries 64..95
    pub ms_lut2: u32,
}

impl Default for MsLut2 {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for MsLut2 {
    fn from(data: u32) -> Self {
        Self {
            ms_lut2: read_from_bit(data, 0, 0xffffffff) as u32,
        }
    }
}

impl From<MsLut2> for u32 {
    fn from(data: MsLut2) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.ms_lut2);
        value
    }
}

impl Register for MsLut2 {
    fn addr() -> u8 {
        0x62
    }
}

#[cfg(test)]
mod ms_lut2 {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(MsLut2 {
                ms_lut2: 0xF0F0F0F0,
                ..Default::default()
            }),
            0xF0F0F0F0
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            MsLut2::from(0xF0F0F0F0),
            MsLut2 {
                ms_lut2: 0xF0F0F0F0,
                ..Default::default()
            },
        )
    }
}
/// MSLUT\[3\]: Microstep table entries 96..127
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MsLut3 {
    /// Microstep table entries 96..127
    pub ms_lut3: u32,
}

impl Default for MsLut3 {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for MsLut3 {
    fn from(data: u32) -> Self {
        Self {
            ms_lut3: read_from_bit(data, 0, 0xffffffff) as u32,
        }
    }
}

impl From<MsLut3> for u32 {
    fn from(data: MsLut3) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.ms_lut3);
        value
    }
}

impl Register for MsLut3 {
    fn addr() -> u8 {
        0x63
    }
}

#[cfg(test)]
mod ms_lut3 {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(MsLut3 {
                ms_lut3: 0xF0F0F0F0,
                ..Default::default()
            }),
            0xF0F0F0F0
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            MsLut3::from(0xF0F0F0F0),
            MsLut3 {
                ms_lut3: 0xF0F0F0F0,
                ..Default::default()
            },
        )
    }
}
/// MSLUT\[4\]: Microstep table entries 128..159
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MsLut4 {
    /// Microstep table entries 128..159
    pub ms_lut4: u32,
}

impl Default for MsLut4 {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for MsLut4 {
    fn from(data: u32) -> Self {
        Self {
            ms_lut4: read_from_bit(data, 0, 0xffffffff) as u32,
        }
    }
}

impl From<MsLut4> for u32 {
    fn from(data: MsLut4) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.ms_lut4);
        value
    }
}

impl Register for MsLut4 {
    fn addr() -> u8 {
        0x64
    }
}

#[cfg(test)]
mod ms_lut4 {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(MsLut4 {
                ms_lut4: 0xF0F0F0F0,
                ..Default::default()
            }),
            0xF0F0F0F0
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            MsLut4::from(0xF0F0F0F0),
            MsLut4 {
                ms_lut4: 0xF0F0F0F0,
                ..Default::default()
            },
        )
    }
}

/// MSLUT\[5\]: Microstep table entries 160..191
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MsLut5 {
    /// Microstep table entries 160..191
    pub ms_lut5: u32,
}

impl Default for MsLut5 {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for MsLut5 {
    fn from(data: u32) -> Self {
        Self {
            ms_lut5: read_from_bit(data, 0, 0xffffffff) as u32,
        }
    }
}

impl From<MsLut5> for u32 {
    fn from(data: MsLut5) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.ms_lut5);
        value
    }
}

impl Register for MsLut5 {
    fn addr() -> u8 {
        0x65
    }
}

#[cfg(test)]
mod ms_lut5 {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(MsLut5 {
                ms_lut5: 0xF0F0F0F0,
                ..Default::default()
            }),
            0xF0F0F0F0
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            MsLut5::from(0xF0F0F0F0),
            MsLut5 {
                ms_lut5: 0xF0F0F0F0,
                ..Default::default()
            },
        )
    }
}

/// MSLUT\[6\]: Microstep table entries 192..223
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MsLut6 {
    /// Microstep table entries 192..223
    pub ms_lut6: u32,
}

impl Default for MsLut6 {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for MsLut6 {
    fn from(data: u32) -> Self {
        Self {
            ms_lut6: read_from_bit(data, 0, 0xffffffff) as u32,
        }
    }
}

impl From<MsLut6> for u32 {
    fn from(data: MsLut6) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.ms_lut6);
        value
    }
}

impl Register for MsLut6 {
    fn addr() -> u8 {
        0x66
    }
}

#[cfg(test)]
mod ms_lut6 {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(MsLut6 {
                ms_lut6: 0xF0F0F0F0,
                ..Default::default()
            }),
            0xF0F0F0F0
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            MsLut6::from(0xF0F0F0F0),
            MsLut6 {
                ms_lut6: 0xF0F0F0F0,
                ..Default::default()
            },
        )
    }
}

/// MSLUT\[7\]: Microstep table entries 224..255
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MsLut7 {
    /// Microstep table entries 224..255
    pub ms_lut7: u32,
}

impl Default for MsLut7 {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for MsLut7 {
    fn from(data: u32) -> Self {
        Self {
            ms_lut7: read_from_bit(data, 0, 0xffffffff) as u32,
        }
    }
}

impl From<MsLut7> for u32 {
    fn from(data: MsLut7) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.ms_lut7);
        value
    }
}

impl Register for MsLut7 {
    fn addr() -> u8 {
        0x67
    }
}

#[cfg(test)]
mod ms_lut7 {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(MsLut7 {
                ms_lut7: 0xF0F0F0F0,
                ..Default::default()
            }),
            0xF0F0F0F0
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            MsLut7::from(0xF0F0F0F0),
            MsLut7 {
                ms_lut7: 0xF0F0F0F0,
                ..Default::default()
            },
        )
    }
}

/// MSLUTSEL: Look up Table Segmentation
///
/// Width control bit coding W0…W3:
/// - %00: MSLUT entry 0, 1 select: -1, +0
/// - %01: MSLUT entry 0, 1 select: +0, +1
/// - %10: MSLUT entry 0, 1 select: +1, +2
/// - %11: MSLUT entry 0, 1 select: +2, +3
///
/// The sine wave look up table can be divided into up to four segments using an individual step width control entry Wx.
/// The segment borders are selected by X1, X2 and X3.
/// - Segment 0 goes from 0 to X1-1.
/// - Segment 1 goes from X1 to X2-1.
/// - Segment 2 goes from X2 to X3-1.
/// - Segment 3 goes from X3 to 255.
/// For defined response the values shall satisfy: 0<X1<X2<X3
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MsLutSel {
    /// W0: LUT width select from ofs00 to ofs(X1-1)
    pub w0: u8,
    /// W1: LUT width select from ofs(X1) to ofs(X2-1)
    pub w1: u8,
    /// W2: LUT width select from ofs(X2) to ofs(X3-1)
    pub w2: u8,
    /// W3: LUT width select from ofs(X3) to ofs255
    pub w3: u8,
    /// X1: LUT segment 1 start
    pub x1: u8,
    /// X2: LUT segment 2 start
    pub x2: u8,
    /// X3: LUT segment 3 start
    pub x3: u8,
}

impl Default for MsLutSel {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl From<u32> for MsLutSel {
    fn from(data: u32) -> Self {
        Self {
            w0: read_from_bit(data, 0, 0x3) as u8,
            w1: read_from_bit(data, 2, 0x3) as u8,
            w2: read_from_bit(data, 4, 0x3) as u8,
            w3: read_from_bit(data, 6, 0x3) as u8,
            x1: read_from_bit(data, 8, 0xff) as u8,
            x2: read_from_bit(data, 16, 0xff) as u8,
            x3: read_from_bit(data, 24, 0xff) as u8,
        }
    }
}

impl From<MsLutSel> for u32 {
    fn from(data: MsLutSel) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0x3, data.w0 as u32);
        write_from_bit(&mut value, 2, 0x3, data.w1 as u32);
        write_from_bit(&mut value, 4, 0x3, data.w2 as u32);
        write_from_bit(&mut value, 6, 0x3, data.w3 as u32);
        write_from_bit(&mut value, 8, 0xff, data.x1 as u32);
        write_from_bit(&mut value, 16, 0xff, data.x2 as u32);
        write_from_bit(&mut value, 24, 0xff, data.x3 as u32);
        value
    }
}

impl Register for MsLutSel {
    fn addr() -> u8 {
        0x68
    }
}

#[cfg(test)]
mod ms_lut_sel {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(MsLutSel {
                x1: 0x66,
                ..Default::default()
            }),
            0x00006600
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            MsLutSel::from(0x00006600),
            MsLutSel {
                x1: 0x66,
                ..Default::default()
            },
        )
    }
}

/// MSLUTSTART
///
/// Start values are transferred to the microstep registers CUR_A and CUR_B, whenever the reference position MSCNT=0 is passed.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MsLutStart {
    /// START_SIN: gives the absolute current at microstep table entry 0.
    pub start_sin: u8,
    /// START_SIN90: gives the absolute current for microstep table entry at positions 256.
    pub start_sin90: u8,
}

impl Default for MsLutStart {
    fn default() -> Self {
        Self {
            start_sin: 0,
            start_sin90: 247,
        }
    }
}

impl From<u32> for MsLutStart {
    fn from(data: u32) -> Self {
        Self {
            start_sin: read_from_bit(data, 0, 0xff) as u8,
            start_sin90: read_from_bit(data, 8, 0xff) as u8,
        }
    }
}

impl From<MsLutStart> for u32 {
    fn from(data: MsLutStart) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xff, data.start_sin as u32);
        write_from_bit(&mut value, 8, 0xff, data.start_sin90 as u32);
        value
    }
}

impl Register for MsLutStart {
    fn addr() -> u8 {
        0x69
    }
}

#[cfg(test)]
mod ms_lut_start {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(MsLutStart {
                start_sin90: 247,
                ..Default::default()
            }),
            0x0000F700
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            MsLutStart::from(0x0000F700),
            MsLutStart {
                start_sin90: 247,
                ..Default::default()
            },
        )
    }
}
