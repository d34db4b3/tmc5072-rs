//! Ramp Generator Driver Feature Control Register Set
//!
//! This register set offers registers for
//! - driver current control
//! - setting thresholds for coolStep operation
//! - setting thresholds for different chopper modes
//! - setting thresholds for dcStep operation
//! - reference switch and stallGuard2 event configuration
//! - a ramp and reference switch status register

use super::Register;
use crate::bits::{read_bool_from_bit, read_from_bit, write_bool_to_bit, write_from_bit};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// IHOLD_IRUN: Driver current control
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IHoldIRun<const M: u8> {
    /// IHOLD: Standstill current (0=1/32…31=32/32)
    ///
    /// In combination with stealthChop mode, setting IHOLD=0 allows to choose freewheeling or coil short circuit for motor stand still.
    pub i_hold: u8,
    /// IRUN: Motor run current (0=1/32…31=32/32)
    ///
    /// Hint: Choose sense resistors in a way, that normal IRUN is 16 to 31 for best microstep performance.
    pub i_run: u8,
    /// IHOLDDELAY: Controls the number of clock cycles for motor power down after a motion as soon as TZEROWAIT has expired.
    /// The smooth transition avoids a motor jerk upon power down.
    ///  - 0: instant power down
    ///  - 1..15: Delay per current reduction step in multiple of 2^18 clocks
    pub i_hold_delay: u8,
}

impl<const M: u8> Default for IHoldIRun<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for IHoldIRun<M> {
    fn from(data: u32) -> Self {
        Self {
            i_hold: read_from_bit(data, 0, 0x1f) as u8,
            i_run: read_from_bit(data, 8, 0x1f) as u8,
            i_hold_delay: read_from_bit(data, 16, 0x0f) as u8,
        }
    }
}

impl<const M: u8> From<IHoldIRun<M>> for u32 {
    fn from(data: IHoldIRun<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0x1f, data.i_hold as u32);
        write_from_bit(&mut value, 8, 0x1f, data.i_run as u32);
        write_from_bit(&mut value, 16, 0x0f, data.i_hold_delay as u32);
        value
    }
}

impl Register for IHoldIRun<0> {
    fn addr() -> u8 {
        0x30
    }
}
impl Register for IHoldIRun<1> {
    fn addr() -> u8 {
        0x50
    }
}

#[cfg(test)]
mod i_hold_i_run {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(IHoldIRun::<1> {
                i_hold: 5,
                i_run: 31,
                i_hold_delay: 1,
                ..Default::default()
            }),
            0x00011F05
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            IHoldIRun::<1>::from(0x00011F05),
            IHoldIRun::<1> {
                i_hold: 5,
                i_run: 31,
                i_hold_delay: 1,
                ..Default::default()
            },
        )
    }
}

/// VCOOLTHRS: coolStep & stallGuard lower threshold velocity (unsigned)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VCoolThrs<const M: u8> {
    /// coolStep & stallGuard lower threshold velocity (unsigned)
    ///
    /// This is the lower threshold velocity for switching on smart energy coolStep and stallGuard feature.
    /// Further it is the upper operation velocity for stealthChop. (unsigned)
    ///
    /// Set this parameter to disable coolStep at low speeds, where it cannot work reliably.
    /// The stop on stall function (enable with sg_stop when using internal motion controller) becomes enabled when exceeding this velocity.
    /// In non-dcStep mode, it becomes disabled again once the velocity falls below this threshold.
    /// This allows for homing procedures with stallGuard by blanking out the stallGuard signal at low velocities (will not work in combination with stealthChop).
    ///
    /// VHIGH ≥ |VACT| ≥ VCOOLTHRS:
    ///  - coolStep and stop on stall are enabled, if configured
    ///  - Voltage PWM mode stealthChop is switched off, if configured
    ///
    /// (Only bits 22..8 are used for value and for comparison)
    pub v_cool_thrs: u32,
}

impl<const M: u8> Default for VCoolThrs<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for VCoolThrs<M> {
    fn from(data: u32) -> Self {
        Self {
            v_cool_thrs: read_from_bit(data, 0, 0x7fffff) as u32,
        }
    }
}

impl<const M: u8> From<VCoolThrs<M>> for u32 {
    fn from(data: VCoolThrs<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0x7fffff, data.v_cool_thrs);
        value
    }
}

impl Register for VCoolThrs<0> {
    fn addr() -> u8 {
        0x31
    }
}
impl Register for VCoolThrs<1> {
    fn addr() -> u8 {
        0x51
    }
}

#[cfg(test)]
mod v_cool_thrs {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(VCoolThrs::<1> {
                v_cool_thrs: 30000,
                ..Default::default()
            }),
            0x00007530
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            VCoolThrs::<1>::from(0x00007530),
            VCoolThrs::<1> {
                v_cool_thrs: 30000,
                ..Default::default()
            },
        )
    }
}

/// VHIGH
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VHigh<const M: u8> {
    /// VHIGH
    ///
    /// This velocity setting allows velocity dependent switching into a different chopper mode and fullstepping to maximize torque. (unsigned)
    ///
    /// |VACT| ≥ VHIGH:
    ///  - coolStep is disabled (motor runs with normal current scale)
    ///  - If vhighchm is set, the chopper switches to chm=1 with TFD=0 (constant off time with slow decay, only).
    ///  - If vhighfs is set, the motor operates in fullstep mode.
    ///  - Voltage PWM mode stealthChop is switched off, if configured
    ///
    /// (Only bits 22..8 are used for value and for comparison)
    pub v_high: u32,
}

impl<const M: u8> Default for VHigh<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for VHigh<M> {
    fn from(data: u32) -> Self {
        Self {
            v_high: read_from_bit(data, 0, 0x7fffff) as u32,
        }
    }
}

impl<const M: u8> From<VHigh<M>> for u32 {
    fn from(data: VHigh<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0x7fffff, data.v_high);
        value
    }
}

impl Register for VHigh<0> {
    fn addr() -> u8 {
        0x32
    }
}
impl Register for VHigh<1> {
    fn addr() -> u8 {
        0x52
    }
}

#[cfg(test)]
mod v_high {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(VHigh::<1> {
                v_high: 400000,
                ..Default::default()
            }),
            0x00061A80
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            VHigh::<1>::from(0x00061A80),
            VHigh::<1> {
                v_high: 400000,
                ..Default::default()
            },
        )
    }
}

/// VDCMIN: dcStep minimum velocity (unsigned)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VDcMin<const M: u8> {
    /// dcStep minimum velocity (unsigned)
    ///
    /// Automatic commutation dcStep becomes enabled above velocity VDCMIN (unsigned)
    /// In this mode, the actual position is determined by the sensorless motor commutation and becomes fed back to XACTUAL.
    /// In case the motor becomes heavily loaded, VDCMIN also is used as the minimum step velocity.
    ///  - 0: Disable, dcStep off
    ///
    /// |VACT| ≥ VDCMIN ≥ 256:
    ///  - Triggers the same actions as exceeding VHIGH.
    ///  - Switches on automatic commutation dcStep
    ///
    /// Hint: Also set bits vhighfs and vhighchm and set DCCTRL parameters in order to operate dcStep.
    ///
    /// (Only bits 22… 8 are used for value and for comparison)
    pub v_dc_min: u32,
}

impl<const M: u8> Default for VDcMin<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for VDcMin<M> {
    fn from(data: u32) -> Self {
        Self {
            v_dc_min: read_from_bit(data, 0, 0x7fffff) as u32,
        }
    }
}

impl<const M: u8> From<VDcMin<M>> for u32 {
    fn from(data: VDcMin<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0x7fffff, data.v_dc_min);
        value
    }
}

impl Register for VDcMin<0> {
    fn addr() -> u8 {
        0x33
    }
}
impl Register for VDcMin<1> {
    fn addr() -> u8 {
        0x53
    }
}

#[cfg(test)]
mod v_dc_min {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(VDcMin::<1> {
                v_dc_min: 500000,
                ..Default::default()
            }),
            0x0007A120
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            VDcMin::<1>::from(0x0007A120),
            VDcMin::<1> {
                v_dc_min: 500000,
                ..Default::default()
            },
        )
    }
}

/// SW_MODE: Reference Switch & stallGuard2 Event Configuration
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwMode<const M: u8> {
    /// stop_l_enable:
    ///  - true: Enables automatic motor stop during active left reference switch input
    ///
    /// Hint: The motor restarts in case the stop switch becomes released.
    pub stop_l_enable: bool,
    /// stop_r_enable:
    ///  - true: Enables automatic motor stop during active right reference switch input
    ///
    /// Hint: The motor restarts in case the stop switch becomes released.
    pub stop_r_enable: bool,
    /// pol_stop_l: Sets the active polarity of the left reference switch input
    ///  - false: non-inverted, high active: a high level on REFL stops the motor
    ///  - true: inverted, low active: a low level on REFL stops the motor
    pub pol_stop_l: bool,
    /// pol_stop_r: Sets the active polarity of the right reference switch input
    ///  - false: non-inverted, high active: a high level on REFR stops the motor
    ///  - true: inverted, low active: a low level on REFR stops the motor
    pub pol_stop_r: bool,
    /// swap_lr:
    ///  - true: Swap the left and the right reference switch input REFL and REFR
    pub swap_lr: bool,
    /// latch_l_active:
    ///  - true: Activates latching of the position to XLATCH upon an active going edge on the left reference switch input REFL.
    ///
    /// Hint: Activate latch_l_active to detect any spurious stop event by reading status_latch_l.
    pub latch_l_active: bool,
    /// latch_l_inactive:
    ///  - true: Activates latching of the position to XLATCH upon an inactive going edge on the left reference switch input REFL.
    ///
    /// The active level is defined by pol_stop_l.
    pub latch_l_inactive: bool,
    /// latch_r_active:
    ///  - true: Activates latching of the position to XLATCH upon an active going edge on the right reference switch input REFR.
    ///
    /// Hint: Activate latch_r_active to detect any spurious stop event by reading status_latch_r
    pub latch_r_active: bool,
    /// latch_r_inactive:
    ///  - true: Activates latching of the position to XLATCH upon an inactive going edge on the right reference switch input REFR.
    ///
    /// The active level is defined by pol_stop_r.
    pub latch_r_inactive: bool,
    /// en_latch_encoder:
    ///  - true: Latch encoder position to ENC_LATCH upon reference switch event.
    pub en_latch_encoder: bool,
    /// sg_stop:
    ///  - true: Enable stop by stallGuard2. Disable to release motor after stop event.
    ///
    /// Attention: Do not enable during motor spin-up, wait until the motor velocity exceeds a certain value,
    /// where stallGuard2 delivers a stable result, or set VCOOLTHRS to a suitable value
    pub sg_stop: bool,
    /// en_softstop:
    ///  - false: Hard stop
    ///  - true: Soft stop
    ///
    /// The soft stop mode always uses the deceleration ramp settings DMAX, V1, D1, VSTOP and TZEROWAIT for stopping the motor.
    /// A stop occurs when the velocity sign matches the reference switch position (REFL for negative velocities, REFR for positive velocities)
    /// and the respective switch stop function is enabled.
    ///
    /// A hard stop also uses TZEROWAIT before the motor becomes released.
    ///
    /// Attention: Do not use soft stop in combination with stallGuard2.
    pub en_softstop: bool,
}

impl<const M: u8> Default for SwMode<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for SwMode<M> {
    fn from(data: u32) -> Self {
        Self {
            stop_l_enable: read_bool_from_bit(data, 0),
            stop_r_enable: read_bool_from_bit(data, 1),
            pol_stop_l: read_bool_from_bit(data, 2),
            pol_stop_r: read_bool_from_bit(data, 3),
            swap_lr: read_bool_from_bit(data, 4),
            latch_l_active: read_bool_from_bit(data, 5),
            latch_l_inactive: read_bool_from_bit(data, 6),
            latch_r_active: read_bool_from_bit(data, 7),
            latch_r_inactive: read_bool_from_bit(data, 8),
            en_latch_encoder: read_bool_from_bit(data, 9),
            sg_stop: read_bool_from_bit(data, 10),
            en_softstop: read_bool_from_bit(data, 11),
        }
    }
}

impl<const M: u8> From<SwMode<M>> for u32 {
    fn from(data: SwMode<M>) -> Self {
        let mut value = 0;
        write_bool_to_bit(&mut value, 0, data.stop_l_enable);
        write_bool_to_bit(&mut value, 1, data.stop_r_enable);
        write_bool_to_bit(&mut value, 2, data.pol_stop_l);
        write_bool_to_bit(&mut value, 3, data.pol_stop_r);
        write_bool_to_bit(&mut value, 4, data.swap_lr);
        write_bool_to_bit(&mut value, 5, data.latch_l_active);
        write_bool_to_bit(&mut value, 6, data.latch_l_inactive);
        write_bool_to_bit(&mut value, 7, data.latch_r_active);
        write_bool_to_bit(&mut value, 8, data.latch_r_inactive);
        write_bool_to_bit(&mut value, 9, data.en_latch_encoder);
        write_bool_to_bit(&mut value, 10, data.sg_stop);
        write_bool_to_bit(&mut value, 11, data.en_softstop);
        value
    }
}

impl Register for SwMode<0> {
    fn addr() -> u8 {
        0x34
    }
}
impl Register for SwMode<1> {
    fn addr() -> u8 {
        0x54
    }
}

#[cfg(test)]
mod sw_mode {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(SwMode::<1> {
                stop_r_enable: true,
                latch_r_inactive: true,
                sg_stop: true,
                ..Default::default()
            }),
            0x00000502
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            SwMode::<1>::from(0x00000502),
            SwMode::<1> {
                stop_r_enable: true,
                latch_r_inactive: true,
                sg_stop: true,
                ..Default::default()
            },
        )
    }
}

/// RAMP_STAT: Ramp and Reference Switch Status
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RampStat<const M: u8> {
    /// status_stop_l: Reference switch left status (true=active)
    pub status_stop_l: bool,
    /// status_stop_r: Reference switch right status (true=active)
    pub status_stop_r: bool,
    /// status_latch_l:
    ///  - true: Latch left ready (enable position latching using SWITCH_MODE settings latch_l_active or latch_l_inactive)
    ///
    /// (Flag is cleared upon reading)
    pub status_latch_l: bool,
    /// status_latch_r:
    ///  - true: Latch right ready (enable position latching using SWITCH_MODE settings latch_r_active or latch_r_inactive)
    ///
    /// (Flag is cleared upon reading)
    pub status_latch_r: bool,
    /// event_stop_l:
    ///  - true: Signals an active stop left condition due to stop switch.
    ///
    /// The stop condition and the interrupt condition can be removed by setting RAMP_MODE to hold mode
    /// or by commanding a move to the opposite direction.
    /// In soft_stop mode, the condition will remain active until the motor has stopped motion into the direction of the stop switch.
    /// Disabling the stop switch or the stop function also clears the flag, but the motor will continue motion.
    ///
    /// This bit is ORed to the interrupt output signal.
    pub event_stop_l: bool,
    ///event_stop_r:
    ///  - true: Signals an active stop right condition due to stop switch.
    ///
    /// The stop condition and the interrupt condition can be removed by setting RAMP_MODE to hold mode
    /// or by commanding a move to the opposite direction.
    /// In soft_stop mode, the condition will remain active until the motor has stopped motion into the direction of the stop switch.
    /// Disabling the stop switch or the stop function also clears the flag, but the motor will continue motion.
    ///
    /// This bit is ORed to the interrupt output signal.
    pub event_stop_r: bool,
    /// event_stop_sg:
    ///  - true: Signals an active StallGuard2 stop event.
    ///
    /// Reading the register will clear the stall condition and the motor may re-start motion, unless the motion controller has been stopped.
    ///
    /// (Flag and interrupt condition are cleared upon reading)
    ///
    /// This bit is ORed to the interrupt output signal.
    pub event_stop_sg: bool,
    /// event_pos_reached:
    ///  - true: Signals, that the target position has been reached (position_reached becoming active).
    ///
    /// (Flag and interrupt condition are cleared upon reading)
    ///
    /// This bit is ORed to the interrupt output signal.
    pub event_pos_reached: bool,
    /// velocity_reached:
    ///  - true: Signals, that the target velocity is reached.
    ///
    /// This flag becomes set while VACTUAL and VMAX match
    pub velocity_reached: bool,
    /// position_reached:
    ///  - true: Signals, that the target position is reached.
    ///
    /// This flag becomes set while XACTUAL and XTARGET match.
    pub position_reached: bool,
    /// vzero :
    ///  - true: Signals, that the actual velocity is 0.
    pub vzero: bool,
    /// t_zerowait_active:
    ///  - true: Signals, that TZEROWAIT is active after a motor stop. During this time, the motor is in standstill.
    pub t_zerowait_active: bool,
    /// second_move:
    ///  - true: Signals that the automatic ramp required moving back in the opposite direction, e.g. due to on-the-fly parameter change
    ///
    /// (Flag is cleared upon reading)
    pub second_move: bool,
    /// status_sg:
    ///  - true: Signals an active stallGuard2 input from the coolStep driver or from the dcStep unit, if enabled.
    ///
    /// Hint: When polling this flag, stall events may be missed – activate sg_stop to be sure not to miss the stall event.
    pub status_sg: bool,
}

impl<const M: u8> Default for RampStat<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for RampStat<M> {
    fn from(data: u32) -> Self {
        Self {
            status_stop_l: read_bool_from_bit(data, 0),
            status_stop_r: read_bool_from_bit(data, 1),
            status_latch_l: read_bool_from_bit(data, 2),
            status_latch_r: read_bool_from_bit(data, 3),
            event_stop_l: read_bool_from_bit(data, 4),
            event_stop_r: read_bool_from_bit(data, 5),
            event_stop_sg: read_bool_from_bit(data, 6),
            event_pos_reached: read_bool_from_bit(data, 7),
            velocity_reached: read_bool_from_bit(data, 8),
            position_reached: read_bool_from_bit(data, 9),
            vzero: read_bool_from_bit(data, 10),
            t_zerowait_active: read_bool_from_bit(data, 11),
            second_move: read_bool_from_bit(data, 12),
            status_sg: read_bool_from_bit(data, 13),
        }
    }
}

impl<const M: u8> From<RampStat<M>> for u32 {
    fn from(data: RampStat<M>) -> Self {
        let mut value = 0;
        write_bool_to_bit(&mut value, 0, data.status_stop_l);
        write_bool_to_bit(&mut value, 1, data.status_stop_r);
        write_bool_to_bit(&mut value, 2, data.status_latch_l);
        write_bool_to_bit(&mut value, 3, data.status_latch_r);
        write_bool_to_bit(&mut value, 4, data.event_stop_l);
        write_bool_to_bit(&mut value, 5, data.event_stop_r);
        write_bool_to_bit(&mut value, 6, data.event_stop_sg);
        write_bool_to_bit(&mut value, 7, data.event_pos_reached);
        write_bool_to_bit(&mut value, 8, data.velocity_reached);
        write_bool_to_bit(&mut value, 9, data.position_reached);
        write_bool_to_bit(&mut value, 10, data.vzero);
        write_bool_to_bit(&mut value, 11, data.t_zerowait_active);
        write_bool_to_bit(&mut value, 12, data.second_move);
        write_bool_to_bit(&mut value, 13, data.status_sg);
        value
    }
}

impl Register for RampStat<0> {
    fn addr() -> u8 {
        0x35
    }
}
impl Register for RampStat<1> {
    fn addr() -> u8 {
        0x55
    }
}

#[cfg(test)]
mod ramp_stat {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(RampStat::<1> {
                event_stop_l: true,
                event_pos_reached: true,
                t_zerowait_active: true,
                ..Default::default()
            }),
            0x00000890
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            RampStat::<1>::from(0x00000890),
            RampStat::<1> {
                event_stop_l: true,
                event_pos_reached: true,
                t_zerowait_active: true,
                ..Default::default()
            },
        )
    }
}

/// XLATCH: Ramp generator latch position
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct XLatch<const M: u8> {
    /// Ramp generator latch position
    ///
    /// Ramp generator latch position, latches XACTUAL upon a programmable switch event (see SW_MODE).
    ///
    /// Hint: The encoder position can be latched to ENC_LATCH together with XLATCH to allow consistency checks.
    pub x_latch: u32,
}

impl<const M: u8> Default for XLatch<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for XLatch<M> {
    fn from(data: u32) -> Self {
        Self {
            x_latch: read_from_bit(data, 0, 0xffffffff) as u32,
        }
    }
}

impl<const M: u8> From<XLatch<M>> for u32 {
    fn from(data: XLatch<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xffffffff, data.x_latch);
        value
    }
}

impl Register for XLatch<0> {
    fn addr() -> u8 {
        0x36
    }
}
impl Register for XLatch<1> {
    fn addr() -> u8 {
        0x56
    }
}

#[cfg(test)]
mod x_latch {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(XLatch::<1> {
                x_latch: 0x0666,
                ..Default::default()
            }),
            0x00000666
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            XLatch::<1>::from(0x00000666),
            XLatch::<1> {
                x_latch: 0x0666,
                ..Default::default()
            },
        )
    }
}
