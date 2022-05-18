//! Encoder Register Set
//!
//! The encoder register set offers all registers needed for proper ABN encoder operation.

use super::Register;
use crate::bits::{read_bool_from_bit, read_from_bit, write_bool_to_bit, write_from_bit};

#[derive(Debug, Clone, Copy, PartialEq)]
/// ENCMODE: Encoder configuration and use of N channel
pub struct EncMode<const N: u8> {
    /// pol_A: Required A polarity for an N channel event (false=neg., true=pos.)
    pub pol_a: bool,
    /// pol_B: Required B polarity for an N channel event (false=neg., true=pos.)
    pub pol_b: bool,
    /// pol_N: Defines active polarity of N (false=neg., true=pos.)
    pub pol_n: bool,
    /// ignore_AB:
    /// - false: An N event occurs only when polarities given by pol_N, pol_A and pol_B match.
    /// - true: Ignore A and B polarity for N channel event
    pub ignore_ab: bool,
    /// clr_cont:
    /// - true: Always latch or latch and clear X_ENC upon an N event (once per revolution, it is recommended to combine this setting with edge sensitive N event)
    pub clr_cont: bool,
    /// clr_once:
    /// - true: Latch or latch and clear X_ENC on the next N event following the write access
    pub clr_once: bool,
    /// neg_edge, pos_edge:
    /// - false false: N channel event is active during an active N event level
    /// - false true: N channel is valid upon active going N event
    /// - true false: N channel is valid upon inactive going N event
    /// - true true: N channel is valid upon active going and inactive going N event
    pub pos_edge: bool,
    /// neg_edge, pos_edge:
    /// - false false: N channel event is active during an active N event level
    /// - false true: N channel is valid upon active going N event
    /// - true false: N channel is valid upon inactive going N event
    /// - true true: N channel is valid upon active going and inactive going N event
    pub neg_edge: bool,
    /// clr_enc_x:
    /// - false: Upon N event, X_ENC becomes latched to ENC_LATCH only
    /// - true: Latch and additionally clear encoder counter X_ENC at N-event
    pub clr_enc_x: bool,
    /// latch_x_act:
    /// - true: Also latch XACTUAL position together with X_ENC. Allows latching the ramp generator position upon an N channel event as selected by pos_edge and neg_edge.
    pub latch_x_act: bool,
    /// enc_sel_decimal:
    /// - false: Encoder prescaler divisor binary mode: Counts in ENC_CONST(fractional part) /65536
    /// - true: Encoder prescaler divisor decimal mode: Counts in ENC_CONST(fractional part) /10000
    pub enc_sel_decimal: bool,
    /// latch_now:
    /// - true: Latch X_ENC (and XACTUAL if selected by bit latch_x_act) directly upon write access to ENCMODE. This allows checking the encoder deviation by comparing the X_LATCH and ENC_LATCH.
    /// - false: No action
    pub latch_now: bool,
}

impl<const N: u8> Default for EncMode<N> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const N: u8> From<u32> for EncMode<N> {
    fn from(data: u32) -> Self {
        Self {
            pol_a: read_bool_from_bit(data, 0),
            pol_b: read_bool_from_bit(data, 1),
            pol_n: read_bool_from_bit(data, 2),
            ignore_ab: read_bool_from_bit(data, 3),
            clr_cont: read_bool_from_bit(data, 4),
            clr_once: read_bool_from_bit(data, 5),
            pos_edge: read_bool_from_bit(data, 6),
            neg_edge: read_bool_from_bit(data, 7),
            clr_enc_x: read_bool_from_bit(data, 8),
            latch_x_act: read_bool_from_bit(data, 9),
            enc_sel_decimal: read_bool_from_bit(data, 10),
            latch_now: read_bool_from_bit(data, 11),
        }
    }
}

impl<const N: u8> From<EncMode<N>> for u32 {
    fn from(data: EncMode<N>) -> Self {
        let mut value = 0;
        write_bool_to_bit(&mut value, 0, data.pol_a);
        write_bool_to_bit(&mut value, 1, data.pol_b);
        write_bool_to_bit(&mut value, 2, data.pol_n);
        write_bool_to_bit(&mut value, 3, data.ignore_ab);
        write_bool_to_bit(&mut value, 4, data.clr_cont);
        write_bool_to_bit(&mut value, 5, data.clr_once);
        write_bool_to_bit(&mut value, 6, data.pos_edge);
        write_bool_to_bit(&mut value, 7, data.neg_edge);
        write_bool_to_bit(&mut value, 8, data.clr_enc_x);
        write_bool_to_bit(&mut value, 9, data.latch_x_act);
        write_bool_to_bit(&mut value, 10, data.enc_sel_decimal);
        write_bool_to_bit(&mut value, 11, data.latch_now);
        value
    }
}

impl Register for EncMode<0> {
    fn addr() -> u8 {
        0x38
    }
}
impl Register for EncMode<1> {
    fn addr() -> u8 {
        0x58
    }
}

#[cfg(test)]
mod enc_mode {
    use super::*;

    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(EncMode::<1> {
                latch_now: true,
                pos_edge: true,
                ..Default::default()
            }),
            0x00000840
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            EncMode::<1>::from(0x00000840),
            EncMode::<1> {
                latch_now: true,
                pos_edge: true,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// X_ENC: Actual encoder position (signed)
pub struct XEnc<const N: u8> {
    /// Actual encoder position (signed)
    pub x_enc: i32,
}

impl<const N: u8> Default for XEnc<N> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const N: u8> From<u32> for XEnc<N> {
    fn from(data: u32) -> Self {
        Self {
            x_enc: read_from_bit(data, 0, 0xffffffff) as i32,
        }
    }
}

impl<const N: u8> From<XEnc<N>> for u32 {
    fn from(data: XEnc<N>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.x_enc as u32);
        value
    }
}

impl Register for XEnc<0> {
    fn addr() -> u8 {
        0x39
    }
}
impl Register for XEnc<1> {
    fn addr() -> u8 {
        0x59
    }
}

#[cfg(test)]
mod x_enc {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(XEnc::<1> {
                x_enc: -0x0666,
                ..Default::default()
            }),
            0xFFFFF99A
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            XEnc::<1>::from(0xFFFFF99A),
            XEnc::<1> {
                x_enc: -0x0666,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// ENC_CONST: Accumulation constant (signed) 16 bit integer part, 16 bit fractional part
///
/// X_ENC accumulates:
///
/// +/- ENC_CONST / (2^16*X_ENC) (binary)
///
/// or
///
/// +/-ENC_CONST / (10^4*X_ENC) (decimal)
///
/// ENCMODE bit enc_sel_decimal switches between decimal and binary setting.
///
/// Use the sign, to match rotation direction!
pub struct EncConst<const N: u8> {
    /// integer part
    pub enc_const_int: i16,
    /// fractional part
    pub enc_const_frac: u16,
}

impl<const N: u8> Default for EncConst<N> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const N: u8> From<u32> for EncConst<N> {
    fn from(data: u32) -> Self {
        Self {
            enc_const_frac: read_from_bit(data, 0, 0xffff) as u16,
            enc_const_int: read_from_bit(data, 16, 0xffff) as i16,
        }
    }
}

impl<const N: u8> From<EncConst<N>> for u32 {
    fn from(data: EncConst<N>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffff, data.enc_const_frac as u32);
        write_from_bit(&mut value, 16, 0xffff, data.enc_const_int as u32);
        value
    }
}

impl Register for EncConst<0> {
    fn addr() -> u8 {
        0x3A
    }
}
impl Register for EncConst<1> {
    fn addr() -> u8 {
        0x5A
    }
}

#[cfg(test)]
mod enc_const {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(EncConst::<1> {
                enc_const_int: -66,
                ..Default::default()
            }),
            0xffbe0000
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            EncConst::<1>::from(0xffbe0000),
            EncConst::<1> {
                enc_const_int: -66,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// ENC_STATUS
pub struct EncStatus<const N: u8> {
    /// n_event:
    /// - true: Encoder N event detected. Status bit is cleared on read: Read (R) + clear (C)
    /// This bit is ORed to the interrupt output signal
    pub enc_status: bool,
}

impl<const N: u8> Default for EncStatus<N> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const N: u8> From<u32> for EncStatus<N> {
    fn from(data: u32) -> Self {
        Self {
            enc_status: read_bool_from_bit(data, 0),
        }
    }
}

impl<const N: u8> From<EncStatus<N>> for u32 {
    fn from(data: EncStatus<N>) -> Self {
        let mut value = 0;
        write_bool_to_bit(&mut value, 0, data.enc_status);
        value
    }
}

impl Register for EncStatus<0> {
    fn addr() -> u8 {
        0x3B
    }
}
impl Register for EncStatus<1> {
    fn addr() -> u8 {
        0x5B
    }
}

#[cfg(test)]
mod enc_status {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(EncStatus::<1> {
                enc_status: true,
                ..Default::default()
            }),
            0x00000001
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            EncStatus::<1>::from(0x00000001),
            EncStatus::<1> {
                enc_status: true,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// ENC_LATCH: Encoder position X_ENC latched on N event
pub struct EncLatch<const N: u8> {
    /// Encoder position X_ENC latched on N event
    pub enc_latch: i32,
}

impl<const N: u8> Default for EncLatch<N> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const N: u8> From<u32> for EncLatch<N> {
    fn from(data: u32) -> Self {
        Self {
            enc_latch: read_from_bit(data, 0, 0xffffffff) as i32,
        }
    }
}

impl<const N: u8> From<EncLatch<N>> for u32 {
    fn from(data: EncLatch<N>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.enc_latch as u32);
        value
    }
}

impl Register for EncLatch<0> {
    fn addr() -> u8 {
        0x3C
    }
}
impl Register for EncLatch<1> {
    fn addr() -> u8 {
        0x5C
    }
}

#[cfg(test)]
mod enc_latch {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(EncLatch::<1> {
                enc_latch: -0x0666,
                ..Default::default()
            }),
            0xFFFFF99A
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            EncLatch::<1>::from(0xFFFFF99A),
            EncLatch::<1> {
                enc_latch: -0x0666,
                ..Default::default()
            },
        )
    }
}
