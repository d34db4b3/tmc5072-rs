//! Ramp Generator Motion Control Register Set
//!
//! This register set offers registers for
//! - choosing a ramp mode
//! - choosing velocities
//! - homing
//! - acceleration and deceleration
//! - target positioning

use super::Register;
use crate::bits::{convert_from_signed_n, convert_to_signed_n, read_from_bit, write_from_bit};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// RAMPMODE
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RampMode<const M: u8> {
    /// RAMPMODE:
    /// - 0: Positioning mode (using all A, D and V parameters)
    /// - 1: Velocity mode to positive VMAX (using AMAX acceleration)
    /// - 2: Velocity mode to negative VMAX (using AMAX acceleration)
    /// - 3: Hold mode (velocity remains unchanged, unless stop event occurs)
    pub ramp_mode: u8,
}

impl<const M: u8> Default for RampMode<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for RampMode<M> {
    fn from(data: u32) -> Self {
        Self {
            ramp_mode: read_from_bit(data, 0, 0x03) as u8,
        }
    }
}

impl<const M: u8> From<RampMode<M>> for u32 {
    fn from(data: RampMode<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0x03, data.ramp_mode as u32);
        value
    }
}

impl Register for RampMode<0> {
    fn addr() -> u8 {
        0x20
    }
}
impl Register for RampMode<1> {
    fn addr() -> u8 {
        0x40
    }
}

#[cfg(test)]
mod ramp_mode {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(RampMode::<1> {
                ramp_mode: 1,
                ..Default::default()
            }),
            0x00000001
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            RampMode::<1>::from(0x00000001),
            RampMode::<1> {
                ramp_mode: 1,
                ..Default::default()
            },
        )
    }
}

/// XACTUAL: Actual motor position (signed)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct XActual<const M: u8> {
    /// Actual motor position (signed)
    ///
    /// Hint: This value normally should only be modified, when homing the drive.
    /// In positioning mode, modifying the register content will start a motion.
    pub x_actual: i32,
}

impl<const M: u8> Default for XActual<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for XActual<M> {
    fn from(data: u32) -> Self {
        Self {
            x_actual: read_from_bit(data, 0, 0xffffffff) as i32,
        }
    }
}

impl<const M: u8> From<XActual<M>> for u32 {
    fn from(data: XActual<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.x_actual as u32);
        value
    }
}

impl Register for XActual<0> {
    fn addr() -> u8 {
        0x21
    }
}
impl Register for XActual<1> {
    fn addr() -> u8 {
        0x41
    }
}

#[cfg(test)]
mod x_actual {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(XActual::<1> {
                x_actual: 0x0666,
                ..Default::default()
            }),
            0x00000666
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            XActual::<1>::from(0x00000666),
            XActual::<1> {
                x_actual: 0x0666,
                ..Default::default()
            },
        )
    }
}

/// VACTUAL: Actual motor velocity from ramp generator (signed)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VActual<const M: u8> {
    /// Actual motor velocity from ramp generator (signed)
    ///
    /// The sign matches the motion direction. A negative sign means motion to lower XACTUAL.
    pub v_actual: i32,
}

impl<const M: u8> Default for VActual<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for VActual<M> {
    fn from(data: u32) -> Self {
        Self {
            v_actual: convert_to_signed_n(read_from_bit(data, 0, 0xffffff) as u32, 24),
        }
    }
}

impl<const M: u8> From<VActual<M>> for u32 {
    fn from(data: VActual<M>) -> Self {
        let mut value = 0;
        write_from_bit(
            &mut value,
            0,
            0xffffff,
            convert_from_signed_n(data.v_actual, 24) as u32,
        );
        value
    }
}

impl Register for VActual<0> {
    fn addr() -> u8 {
        0x22
    }
}
impl Register for VActual<1> {
    fn addr() -> u8 {
        0x42
    }
}

#[cfg(test)]
mod v_actual {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(VActual::<1> {
                v_actual: -0x0666,
                ..Default::default()
            }),
            0x00fff99a
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            VActual::<1>::from(0x00fff99a),
            VActual::<1> {
                v_actual: -0x0666,
                ..Default::default()
            },
        )
    }
}

/// VSTART: Motor start velocity (unsigned)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VStart<const M: u8> {
    /// Motor start velocity (unsigned)
    ///
    /// Set VSTOP ≥ VSTART!
    pub v_start: u32,
}

impl<const M: u8> Default for VStart<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for VStart<M> {
    fn from(data: u32) -> Self {
        Self {
            v_start: read_from_bit(data, 0, 0x3ffff) as u32,
        }
    }
}

impl<const M: u8> From<VStart<M>> for u32 {
    fn from(data: VStart<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0x3ffff, data.v_start as u32);
        value
    }
}

impl Register for VStart<0> {
    fn addr() -> u8 {
        0x23
    }
}
impl Register for VStart<1> {
    fn addr() -> u8 {
        0x43
    }
}

#[cfg(test)]
mod v_start {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(VStart::<1> {
                v_start: 0x0666,
                ..Default::default()
            }),
            0x00000666
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            VStart::<1>::from(0x00000666),
            VStart::<1> {
                v_start: 0x0666,
                ..Default::default()
            },
        )
    }
}

/// A1: First acceleration between VSTART and V1 (unsigned)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct A1<const M: u8> {
    /// First acceleration between VSTART and V1 (unsigned)
    pub a1: u16,
}

impl<const M: u8> Default for A1<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for A1<M> {
    fn from(data: u32) -> Self {
        Self {
            a1: read_from_bit(data, 0, 0xffff) as u16,
        }
    }
}

impl<const M: u8> From<A1<M>> for u32 {
    fn from(data: A1<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffff, data.a1 as u32);
        value
    }
}

impl Register for A1<0> {
    fn addr() -> u8 {
        0x24
    }
}
impl Register for A1<1> {
    fn addr() -> u8 {
        0x44
    }
}

#[cfg(test)]
mod a1 {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(A1::<1> {
                a1: 0x0666,
                ..Default::default()
            }),
            0x00000666
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            A1::<1>::from(0x00000666),
            A1::<1> {
                a1: 0x0666,
                ..Default::default()
            },
        )
    }
}

/// V1: First acceleration / deceleration phase threshold velocity (unsigned)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct V1<const M: u8> {
    /// First acceleration / deceleration phase threshold velocity (unsigned)
    ///
    /// 0: Disables A1 and D1 phase, use AMAX, DMAX only
    pub v1: u32,
}

impl<const M: u8> Default for V1<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for V1<M> {
    fn from(data: u32) -> Self {
        Self {
            v1: read_from_bit(data, 0, 0xfffff) as u32,
        }
    }
}

impl<const M: u8> From<V1<M>> for u32 {
    fn from(data: V1<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xfffff, data.v1 as u32);
        value
    }
}

impl Register for V1<0> {
    fn addr() -> u8 {
        0x25
    }
}
impl Register for V1<1> {
    fn addr() -> u8 {
        0x45
    }
}

#[cfg(test)]
mod v1 {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(V1::<1> {
                v1: 0x0666,
                ..Default::default()
            }),
            0x00000666
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            V1::<1>::from(0x00000666),
            V1::<1> {
                v1: 0x0666,
                ..Default::default()
            },
        )
    }
}

/// AMAX: Second acceleration between V1 and VMAX (unsigned)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AMax<const M: u8> {
    /// Second acceleration between V1 and VMAX (unsigned)
    ///
    /// This is the acceleration and deceleration value for velocity mode.
    pub a_max: u16,
}

impl<const M: u8> Default for AMax<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for AMax<M> {
    fn from(data: u32) -> Self {
        Self {
            a_max: read_from_bit(data, 0, 0xffff) as u16,
        }
    }
}

impl<const M: u8> From<AMax<M>> for u32 {
    fn from(data: AMax<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffff, data.a_max as u32);
        value
    }
}

impl Register for AMax<0> {
    fn addr() -> u8 {
        0x26
    }
}
impl Register for AMax<1> {
    fn addr() -> u8 {
        0x46
    }
}

#[cfg(test)]
mod a_max {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(AMax::<1> {
                a_max: 0x0666,
                ..Default::default()
            }),
            0x00000666
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            AMax::<1>::from(0x00000666),
            AMax::<1> {
                a_max: 0x0666,
                ..Default::default()
            },
        )
    }
}

/// VMAX: Motion ramp target velocity (unsigned)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VMax<const M: u8> {
    /// Motion ramp target velocity (unsigned)
    ///
    /// This is the target velocity in velocity mode. It can be changed any time during a motion.
    pub v_max: u32,
}

impl<const M: u8> Default for VMax<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for VMax<M> {
    fn from(data: u32) -> Self {
        Self {
            v_max: read_from_bit(data, 0, 0x7fffff) as u32,
        }
    }
}

impl<const M: u8> From<VMax<M>> for u32 {
    fn from(data: VMax<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0x7fffff, data.v_max as u32);
        value
    }
}

impl Register for VMax<0> {
    fn addr() -> u8 {
        0x27
    }
}
impl Register for VMax<1> {
    fn addr() -> u8 {
        0x47
    }
}

#[cfg(test)]
mod v_max {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(VMax::<1> {
                v_max: 0x0666,
                ..Default::default()
            }),
            0x00000666
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            VMax::<1>::from(0x00000666),
            VMax::<1> {
                v_max: 0x0666,
                ..Default::default()
            },
        )
    }
}

/// DMAX: Deceleration between VMAX and V1 (unsigned)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DMax<const M: u8> {
    /// Deceleration between VMAX and V1 (unsigned)
    pub d_max: u16,
}

impl<const M: u8> Default for DMax<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for DMax<M> {
    fn from(data: u32) -> Self {
        Self {
            d_max: read_from_bit(data, 0, 0xffff) as u16,
        }
    }
}

impl<const M: u8> From<DMax<M>> for u32 {
    fn from(data: DMax<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffff, data.d_max as u32);
        value
    }
}

impl Register for DMax<0> {
    fn addr() -> u8 {
        0x28
    }
}
impl Register for DMax<1> {
    fn addr() -> u8 {
        0x48
    }
}

#[cfg(test)]
mod d_max {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(DMax::<1> {
                d_max: 0x0666,
                ..Default::default()
            }),
            0x00000666
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            DMax::<1>::from(0x00000666),
            DMax::<1> {
                d_max: 0x0666,
                ..Default::default()
            },
        )
    }
}
/// D1: Deceleration between V1 and VSTOP (unsigned)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct D1<const M: u8> {
    /// Deceleration between V1 and VSTOP (unsigned)
    ///
    /// Attention: Do not set 0 in positioning mode, even if V1=0!
    pub d1: u16,
}

impl<const M: u8> Default for D1<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for D1<M> {
    fn from(data: u32) -> Self {
        Self {
            d1: read_from_bit(data, 0, 0xffff) as u16,
        }
    }
}

impl<const M: u8> From<D1<M>> for u32 {
    fn from(data: D1<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffff, data.d1 as u32);
        value
    }
}

impl Register for D1<0> {
    fn addr() -> u8 {
        0x2a
    }
}
impl Register for D1<1> {
    fn addr() -> u8 {
        0x4a
    }
}

#[cfg(test)]
mod d1 {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(D1::<1> {
                d1: 0x0666,
                ..Default::default()
            }),
            0x00000666
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            D1::<1>::from(0x00000666),
            D1::<1> {
                d1: 0x0666,
                ..Default::default()
            },
        )
    }
}

/// VSTOP: Motor stop velocity (unsigned)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VStop<const M: u8> {
    ///  Motor stop velocity (unsigned)
    ///
    /// Attention: Set VSTOP ≥ VSTART!
    ///
    /// Attention: Do not set 0 in positioning mode, minimum 10 recommended!
    pub v_stop: u32,
}

impl<const M: u8> Default for VStop<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for VStop<M> {
    fn from(data: u32) -> Self {
        Self {
            v_stop: read_from_bit(data, 0, 0x3ffff) as u32,
        }
    }
}

impl<const M: u8> From<VStop<M>> for u32 {
    fn from(data: VStop<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0x3ffff, data.v_stop as u32);
        value
    }
}

impl Register for VStop<0> {
    fn addr() -> u8 {
        0x2b
    }
}
impl Register for VStop<1> {
    fn addr() -> u8 {
        0x4b
    }
}

#[cfg(test)]
mod v_stop {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(VStop::<1> {
                v_stop: 0x0666,
                ..Default::default()
            }),
            0x00000666
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            VStop::<1>::from(0x00000666),
            VStop::<1> {
                v_stop: 0x0666,
                ..Default::default()
            },
        )
    }
}

/// TZEROWAIT: Waiting time after ramping down to zero velocity
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TZeroWait<const M: u8> {
    /// Waiting time after ramping down to zero velocity before next movement or direction inversion can start and before motor power down starts.
    /// Time range is about 0 to 2 seconds.
    ///
    /// This setting avoids excess acceleration e.g. from VSTOP to -VSTART.
    pub t_zero_wait: u16,
}

impl<const M: u8> Default for TZeroWait<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for TZeroWait<M> {
    fn from(data: u32) -> Self {
        Self {
            t_zero_wait: read_from_bit(data, 0, 0xffff) as u16,
        }
    }
}

impl<const M: u8> From<TZeroWait<M>> for u32 {
    fn from(data: TZeroWait<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffff, data.t_zero_wait as u32);
        value
    }
}

impl Register for TZeroWait<0> {
    fn addr() -> u8 {
        0x2c
    }
}
impl Register for TZeroWait<1> {
    fn addr() -> u8 {
        0x4c
    }
}

#[cfg(test)]
mod t_zero_wait {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(TZeroWait::<1> {
                t_zero_wait: 0x0666,
                ..Default::default()
            }),
            0x00000666
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            TZeroWait::<1>::from(0x00000666),
            TZeroWait::<1> {
                t_zero_wait: 0x0666,
                ..Default::default()
            },
        )
    }
}

/// XTARGET: Target position for ramp mode (signed)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct XTarget<const M: u8> {
    ///Target position for ramp mode (signed)
    ///
    /// Target position for ramp mode (signed). Write a new target position to this register in order to activate the ramp generator positioning in RAMPMODE=0.
    /// Initialize all velocity, acceleration and deceleration parameters before.
    ///
    /// Hint: The position is allowed to wrap around, thus, XTARGET value optionally can be treated as an unsigned number.
    ///
    /// Hint: The maximum possible displacement is +/-((2^31)-1).
    ///
    /// Hint: When increasing V1, D1 or DMAX during a motion, rewrite XTARGET afterwards in order to trigger a second acceleration phase, if desired.
    pub x_target: i32,
}

impl<const M: u8> Default for XTarget<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for XTarget<M> {
    fn from(data: u32) -> Self {
        Self {
            x_target: read_from_bit(data, 0, 0xffffffff) as i32,
        }
    }
}

impl<const M: u8> From<XTarget<M>> for u32 {
    fn from(data: XTarget<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.x_target as u32);
        value
    }
}

impl Register for XTarget<0> {
    fn addr() -> u8 {
        0x2d
    }
}
impl Register for XTarget<1> {
    fn addr() -> u8 {
        0x4d
    }
}

#[cfg(test)]
mod x_target {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(XTarget::<1> {
                x_target: 0x0666,
                ..Default::default()
            }),
            0x00000666
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            XTarget::<1>::from(0x00000666),
            XTarget::<1> {
                x_target: 0x0666,
                ..Default::default()
            },
        )
    }
}
