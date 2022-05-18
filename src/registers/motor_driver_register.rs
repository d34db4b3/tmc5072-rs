//! Motor Driver Register Set
//!
//! This register set offers registers for
//! - setting / reading out microstep table and counter
//! - chopper and driver configuration
//! - coolStep and stallGuard2 configuration
//! - dcStep configuration, and
//! - reading out stallGuard2 values and driver error flags

use super::Register;
use crate::bits::{
    convert_from_signed_n, convert_to_signed_n, read_bool_from_bit, read_from_bit,
    write_bool_to_bit, write_from_bit,
};

#[derive(Debug, Clone, Copy, PartialEq)]
/// MSCNT: Microstep counter.
pub struct MsCnt<const M: u8> {
    /// Microstep counter
    ///
    /// Indicates actual position in the microstep table for CUR_A. CUR_B uses an offset of 256.
    ///
    /// Hint: Move to a position where MSCNT is zero before re-initializing MSLUTSTART or MSLUT and MSLUTSEL.
    pub ms_cnt: u16,
}

impl<const M: u8> Default for MsCnt<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for MsCnt<M> {
    fn from(data: u32) -> Self {
        Self {
            ms_cnt: read_from_bit(data, 0, 0x3ff) as u16,
        }
    }
}

impl<const M: u8> From<MsCnt<M>> for u32 {
    fn from(data: MsCnt<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0x3ff, data.ms_cnt as u32);
        value
    }
}

impl Register for MsCnt<0> {
    fn addr() -> u8 {
        0x6A
    }
}
impl Register for MsCnt<1> {
    fn addr() -> u8 {
        0x7A
    }
}

#[cfg(test)]
mod ms_cnt {
    use super::*;

    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(MsCnt::<1> {
                ms_cnt: 600,
                ..Default::default()
            }),
            0x00000258
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            MsCnt::<1>::from(0x00000258),
            MsCnt::<1> {
                ms_cnt: 600,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// MSCURACT
pub struct MsCurAct<const M: u8> {
    /// CUR_A (signed): Actual microstep current for motor phase A as read from MSLUT (not scaled by current)
    pub cur_a: i16,
    /// CUR_B (signed): Actual microstep current for motor phase B as read from MSLUT (not scaled by current)
    pub cur_b: i16,
}

impl<const M: u8> Default for MsCurAct<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for MsCurAct<M> {
    fn from(data: u32) -> Self {
        Self {
            cur_a: convert_to_signed_n(read_from_bit(data, 0, 0x1ff) as u32, 9) as i16,
            cur_b: convert_to_signed_n(read_from_bit(data, 16, 0x1ff) as u32, 9) as i16,
        }
    }
}

impl<const M: u8> From<MsCurAct<M>> for u32 {
    fn from(data: MsCurAct<M>) -> Self {
        let mut value = 0;
        write_from_bit(
            &mut value,
            0,
            0x1ff,
            convert_from_signed_n(data.cur_a as i32, 9) as u32,
        );
        write_from_bit(
            &mut value,
            16,
            0x1ff,
            convert_from_signed_n(data.cur_b as i32, 9) as u32,
        );
        value
    }
}

impl Register for MsCurAct<0> {
    fn addr() -> u8 {
        0x6B
    }
}
impl Register for MsCurAct<1> {
    fn addr() -> u8 {
        0x7B
    }
}

#[cfg(test)]
mod ms_cur_act {
    use super::*;

    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(MsCurAct::<1> {
                cur_a: -256,
                cur_b: 255,
                ..Default::default()
            }),
            0x00ff0100
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            MsCurAct::<1>::from(0x00ff0100),
            MsCurAct::<1> {
                cur_a: -256,
                cur_b: 255,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// CHOPCONF: Chopper and driver configuration
pub struct ChopConf<const M: u8> {
    /// TOFF off time and driver enable
    ///
    /// Off time setting controls duration of slow decay phase
    ///
    /// NCLK= 12 + 32*TOFF
    /// - %0000: Driver disable, all bridges off
    /// - %0001: 1 – use only with TBL ≥ 36 clocks
    /// - %0010..%1111: 2..15
    pub toff: u8,
    /// chm=false:
    ///
    /// HSTRT: hysteresis start value added to HEND
    ///
    /// %000..%111: Add 1, 2, …, 8 to hysteresis low value HEND (1/512 of this setting adds to current setting)
    ///
    /// Attention: Effective HEND+HSTRT ≤ 16.
    ///
    /// Hint: Hysteresis decrement is done each 16 clocks
    ///
    /// chm=true:
    ///
    /// TFD \[2..0\]: fast decay time setting
    ///
    /// Fast decay time setting (MSB: fd3):
    ///
    /// %0000..%1111: Fast decay time setting TFD with NCLK= 32*HSTRT (%0000: slow decay only)
    pub hstrt: u8,
    /// chm=false:
    ///
    /// HEND: hysteresis low value
    ///
    /// %0000..%1111: Hysteresis is -3, -2, -1, 0, 1, …, 12 (1/512 of this setting adds to current setting)
    ///
    /// This is the hysteresis value which becomes used for the hysteresis chopper.
    ///
    /// chm=true:
    ///
    /// OFFSET sine wave offset
    ///
    /// %0000..%1111: Offset is -3, -2, -1, 0, 1, …, 12
    ///
    /// This is the sine wave offset and 1/512 of the value becomes added to the absolute value of each sine wave entry.
    pub hend: u8,
    /// TFD \[3\]
    ///
    /// chm=true: MSB of fast decay time setting TFD
    pub fd3: bool,
    /// disfdcc: fast decay mode
    ///
    /// chm=true: disfdcc=1 disables current comparator usage for termination of the fast decay cycle
    pub disfdcc: bool,
    /// rndtf: random TOFF time
    /// - false: Chopper off time is fixed as set by TOFF
    /// - true: Random mode, TOFF is random modulated by dNCLK= -12..+3 clocks.
    pub rndtf: bool,
    /// chm: chopper mode
    /// - false: Standard mode (spreadCycle)
    /// - true: Constant off time with fast decay time.
    ///
    /// Fast decay time is also terminated when the negative nominal current is reached. Fast decay is after on time.
    pub chm: bool,
    /// TBL: blank time select
    ///
    /// %00..%11: Set comparator blank time to 16, 24, 36 or 54 clocks
    ///
    /// Hint: %01 or %10 recommended for most applications
    pub tbl: u8,
    /// vsense: sense resistor voltage based current scaling
    /// - false: Low sensitivity, high sense resistor voltage
    /// - true: High sensitivity, low sense resistor voltage
    pub vsense: bool,
    /// vhighfs: high velocity fullstep selection
    ///
    /// This bit enables switching to fullstep, when VHIGH is exceeded. Switching takes place only at 45° position.
    /// The fullstep target current uses the current value from the microstep table at the 45° position.
    pub vhighfs: bool,
    /// vhighchm: high velocity chopper mode
    ///
    /// This bit enables switching to chm=true and fd=0, when VHIGH is exceeded. This way, a higher velocity can be achieved.
    /// Can be combined with vhighfs=true. If set, the TOFF setting automatically becomes doubled during high velocity operation in order to avoid doubling of the chopper frequency
    pub vhighchm: bool,
    /// MRES: micro step resolution
    ///
    /// %0000: Native 256 microstep setting. Use this setting when the IC is operated with the internal ramp generator.
    ///
    /// %0001..%1000: 128, 64, 32, 16, 8, 4, 2, FULLSTEP
    /// Reduced microstep resolution for Step/Dir operation. The resolution gives the number of microstep entries per sine quarter wave.
    /// Especially when switching to a low resolution of 8 microsteps and below, take care to switch at certain microstep positions.
    /// The switching position determines the sequence of patterns.
    ///
    /// step width=2^MRES \[microsteps\]
    ///
    /// Hint: Reduced microstep resolutions are also useful in special cases to extend the acceleration or position range
    pub mres: u8,
    /// intpol16: 16 microsteps with interpolation
    /// - true: In 16 microstep mode with Step/Dir interface, the microstep resolution becomes extrapolated to 256 microsteps for smoothest motor operation
    pub intpol16: bool,
    /// dedge: enable double edge step pulses
    /// - true: Enable step impulse at each step edge to reduce step frequency requirement.
    ///
    /// Attention: Use only in Step/Dir mode.
    pub dedge: bool,
    /// diss2g short to GND protection disable
    /// - false: Short to GND protection is on
    /// - true: Short to GND protection is disabled
    pub diss2g: bool,
}

impl<const M: u8> Default for ChopConf<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for ChopConf<M> {
    fn from(data: u32) -> Self {
        Self {
            toff: read_from_bit(data, 0, 0x0f) as u8,
            hstrt: read_from_bit(data, 4, 0x07) as u8,
            hend: read_from_bit(data, 7, 0x0f) as u8,
            fd3: read_bool_from_bit(data, 11),
            disfdcc: read_bool_from_bit(data, 12),
            rndtf: read_bool_from_bit(data, 13),
            chm: read_bool_from_bit(data, 14),
            tbl: read_from_bit(data, 15, 0x03) as u8,
            vsense: read_bool_from_bit(data, 17),
            vhighfs: read_bool_from_bit(data, 18),
            vhighchm: read_bool_from_bit(data, 19),
            mres: read_from_bit(data, 24, 0x0f) as u8,
            intpol16: read_bool_from_bit(data, 28),
            dedge: read_bool_from_bit(data, 29),
            diss2g: read_bool_from_bit(data, 30),
        }
    }
}

impl<const M: u8> From<ChopConf<M>> for u32 {
    fn from(data: ChopConf<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0x0f, data.toff as u32);
        write_from_bit(&mut value, 4, 0x07, data.hstrt as u32);
        write_from_bit(&mut value, 7, 0x0f, data.hend as u32);
        write_bool_to_bit(&mut value, 11, data.fd3);
        write_bool_to_bit(&mut value, 12, data.disfdcc);
        write_bool_to_bit(&mut value, 13, data.rndtf);
        write_bool_to_bit(&mut value, 14, data.chm);
        write_from_bit(&mut value, 15, 0x03, data.tbl as u32);
        write_bool_to_bit(&mut value, 17, data.vsense);
        write_bool_to_bit(&mut value, 18, data.vhighfs);
        write_bool_to_bit(&mut value, 19, data.vhighchm);
        write_from_bit(&mut value, 24, 0x0f, data.mres as u32);
        write_bool_to_bit(&mut value, 28, data.intpol16);
        write_bool_to_bit(&mut value, 29, data.dedge);
        write_bool_to_bit(&mut value, 30, data.diss2g);
        value
    }
}

impl Register for ChopConf<0> {
    fn addr() -> u8 {
        0x6C
    }
}
impl Register for ChopConf<1> {
    fn addr() -> u8 {
        0x7C
    }
}

#[cfg(test)]
mod chop_conf {
    use super::*;

    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(ChopConf::<1> {
                chm: false,
                toff: 5,
                hstrt: 4,
                hend: 1,
                tbl: 2,
                ..Default::default()
            }),
            0x000100C5
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            ChopConf::<1>::from(0x000100C5),
            ChopConf::<1> {
                chm: false,
                toff: 5,
                hstrt: 4,
                hend: 1,
                tbl: 2,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// coolStep smart current control register and stallGuard2 configuration
pub struct CoolConf<const M: u8> {
    /// semin: minimum stallGuard2 value for smart current control and smart current enable
    ///
    /// If the stallGuard2 result falls below SEMIN*32, the motor current becomes increased to reduce motor load angle.
    /// - %0000: smart current control coolStep off
    /// - %0001..%1111: 1..15
    pub semin: u8,
    /// seup: current up step width
    ///
    /// Current increment steps per measured stallGuard2 value
    ///
    /// %00..%11: 1, 2, 4, 8
    pub seup: u8,
    /// semax: stallGuard2 hysteresis value for smart current control
    ///
    /// If the stallGuard2 result is equal to or above (SEMIN+SEMAX+1)*32, the motor current becomes decreased to save energy.
    ///
    /// %0000..%1111: 0..15
    pub semax: u8,
    /// sedn: current down step speed
    /// - %00: For each 32 stallGuard2 values decrease by one
    /// - %01: For each 8 stallGuard2 values decrease by one
    /// - %10: For each 2 stallGuard2 values decrease by one
    /// - %11: For each stallGuard2 value decrease by one
    pub sedn: u8,
    /// seimin: minimum current for smart current control
    /// - false: 1/2 of current setting (IRUN)
    /// - true: 1/4 of current setting (IRUN)
    pub seimin: bool,
    /// sgt: stallGuard2 threshold value
    ///
    /// This signed value controls stallGuard2 level for stall output and sets the optimum measurement range for readout.
    /// A lower value gives a higher sensitivity. Zero is the starting value working with most motors.
    ///
    /// -64 to +63: A higher value makes stallGuard2 less sensitive and requires more torque to indicate a stall.
    pub sgt: i8,
    /// sfilt: stallGuard2 filter enable
    /// - false: Standard mode, high time resolution for stallGuard2
    /// - true: Filtered mode, stallGuard2 signal updated for each four fullsteps only to compensate for motor pole tolerances
    pub sfilt: bool,
}

impl<const M: u8> Default for CoolConf<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for CoolConf<M> {
    fn from(data: u32) -> Self {
        let sgt = read_from_bit(data, 16, 0x7f) as u8;
        Self {
            semin: read_from_bit(data, 0, 0x0f) as u8,
            seup: read_from_bit(data, 5, 0x03) as u8,
            semax: read_from_bit(data, 8, 0x0f) as u8,
            sedn: read_from_bit(data, 13, 0x03) as u8,
            seimin: read_bool_from_bit(data, 15),
            sgt: if sgt >> 6 & 1 == 1 {
                -(((!sgt) & 0x3f) as i8 + 1)
            } else {
                sgt as i8
            },
            sfilt: read_bool_from_bit(data, 24),
        }
    }
}

impl<const M: u8> From<CoolConf<M>> for u32 {
    fn from(data: CoolConf<M>) -> Self {
        let mut value = 0;
        let corrected_sgt = if data.sgt.is_negative() {
            ((!(-(data.sgt + 1)) & 0x3f) as u8) | 1 << 6
        } else {
            data.sgt as u8
        };
        write_from_bit(&mut value, 0, 0x0f, data.semin as u32);
        write_from_bit(&mut value, 5, 0x03, data.seup as u32);
        write_from_bit(&mut value, 8, 0x0f, data.semax as u32);
        write_from_bit(&mut value, 13, 0x03, data.sedn as u32);
        write_bool_to_bit(&mut value, 15, data.seimin);
        write_from_bit(&mut value, 16, 0x7f, corrected_sgt as u32);
        write_bool_to_bit(&mut value, 24, data.sfilt);
        value
    }
}

impl Register for CoolConf<0> {
    fn addr() -> u8 {
        0x6D
    }
}
impl Register for CoolConf<1> {
    fn addr() -> u8 {
        0x7D
    }
}

#[cfg(test)]
mod cool_conf {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(CoolConf::<1> {
                sgt: -64,
                seup: 3,
                semin: 5,
                sfilt: true,
                ..Default::default()
            }),
            0x01400065
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            CoolConf::<1>::from(0x01400065),
            CoolConf::<1> {
                sgt: -64,
                seup: 3,
                semin: 5,
                sfilt: true,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// DCCTRL: dcStep (DC) automatic commutation configuration
pub struct DcCtrl<const M: u8> {
    /// DC_TIME: Upper PWM on time limit for commutation (DC_TIME * 1/fCLK).
    ///
    /// Set slightly above effective blank time TBL.
    pub dc_time: u8,
    /// DC_SG: Max. PWM on time for step loss detection using dcStep stallGuard2 in dcStep mode (DC_SG * 16/fCLK)
    ///
    /// Set slightly higher than DC_TIME/16
    ///
    /// 0=disable
    pub dc_sg: u8,
}

impl<const M: u8> Default for DcCtrl<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for DcCtrl<M> {
    fn from(data: u32) -> Self {
        Self {
            dc_time: read_from_bit(data, 0, 0xff) as u8,
            dc_sg: read_from_bit(data, 8, 0xff) as u8,
        }
    }
}

impl<const M: u8> From<DcCtrl<M>> for u32 {
    fn from(data: DcCtrl<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0xff, data.dc_time as u32);
        write_from_bit(&mut value, 8, 0xff, data.dc_sg as u32);
        value
    }
}

impl Register for DcCtrl<0> {
    fn addr() -> u8 {
        0x6E
    }
}
impl Register for DcCtrl<1> {
    fn addr() -> u8 {
        0x7E
    }
}

#[cfg(test)]
mod dc_ctrl {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(DcCtrl::<1> {
                dc_time: 66,
                dc_sg: 6,
                ..Default::default()
            }),
            0x00000642
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            DcCtrl::<1>::from(0x00000642),
            DcCtrl::<1> {
                dc_time: 66,
                dc_sg: 6,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// stallGuard2 value and driver error flags
pub struct DrvStatus<const M: u8> {
    /// SG_ RESULT: stallGuard2 result respectively PWM on time for coil A in stand still for motor temperature detection
    ///
    /// Mechanical load measurement: The stallGuard2 result gives a means to measure mechanical motor load.
    /// A higher value means lower mechanical load. A value of 0 signals highest load.
    /// With optimum SGT setting, this is an indicator for a motor stall. The stall detection compares SG_RESULT to 0 in order to detect a stall.
    /// SG_RESULT is used as a base for coolStep operation, by comparing it to a programmable upper and a lower limit.
    /// It is not applicable in stealthChop mode.
    ///
    /// SG_RESULT is also applicable when dcStep is active. stallGuard2 works best with microstep operation.
    ///
    /// Temperature measurement: In standstill, no stallGuard2 result can be obtained. SG_RESULT shows the chopper on-time for motor coil A instead.
    /// If the motor is moved to a determined microstep position at a certain current setting,
    /// a comparison of the chopper on-time can help to get a rough estimation of motor temperature.
    /// As the motor heats up, its coil resistance rises and the chopper on-time increases.
    pub sg_result: u16,
    /// fsactive: full step active indicator
    /// - true: Indicates that the driver has switched to fullstep as defined by chopper mode settings and velocity thresholds.
    pub fsactive: bool,
    /// CS_ACTUAL: actual motor current / smart energy current
    ///
    /// Actual current control scaling, for monitoring smart energy current scaling controlled via settings in register COOLCONF,
    /// or for monitoring the function of the automatic current scaling.
    pub cs_actual: u8,
    /// stallGuard: stallGuard2 status
    /// - true: Motor stall detected (SG_RESULT=0) or dcStep stall in dcStep mode.
    pub stall_guard: bool,
    /// ot: overtemperature flag
    ///  - true: Overtemperature limit has been reached. Drivers become disabled until otpw is also cleared due to cooling down of the IC.
    ///
    /// The overtemperature flag is common for both drivers.
    pub ot: bool,
    /// otpw: overtemperature prewarning flag
    /// - true: Overtemperature pre-warning threshold is exceeded.
    ///
    /// The overtemperature pre-warning flag is common for both drivers.
    pub otpw: bool,
    /// s2ga short to ground indicator phase A
    /// - true: Short to GND detected on phase A. The driver becomes disabled.
    /// The flags stay active, until the driver is disabled by software (TOFF=0) or by the ENN input.
    pub s2ga: bool,
    /// s2gb short to ground indicator phase B
    /// - true: Short to GND detected on phase B. The driver becomes disabled.
    /// The flags stay active, until the driver is disabled by software (TOFF=0) or by the ENN input.
    pub s2gb: bool,
    /// ola open load indicator phase A
    /// - true: Open load detected on phase A.
    ///
    /// Hint: This is just an informative flag. The driver takes no action upon it.
    /// False detection may occur in fast motion and standstill. Check during slow motion or after a motion, only.
    pub ola: bool,
    /// olb open load indicator phase B
    /// - true: Open load detected on phase B.
    ///
    /// Hint: This is just an informative flag. The driver takes no action upon it.
    /// False detection may occur in fast motion and standstill. Check during slow motion or after a motion, only.
    pub olb: bool,
    /// stst: standstill indicator
    ///
    /// This flag indicates motor stand still in each operation mode. It is especially useful for step & dir mode.
    pub stst: bool,
}

impl<const M: u8> Default for DrvStatus<M> {
    fn default() -> Self {
        Self::from(0u32)
    }
}

impl<const M: u8> From<u32> for DrvStatus<M> {
    fn from(data: u32) -> Self {
        Self {
            sg_result: read_from_bit(data, 0, 0x3ff) as u16,
            fsactive: read_bool_from_bit(data, 15),
            cs_actual: read_from_bit(data, 16, 0x1f) as u8,
            stall_guard: read_bool_from_bit(data, 24),
            ot: read_bool_from_bit(data, 25),
            otpw: read_bool_from_bit(data, 26),
            s2ga: read_bool_from_bit(data, 27),
            s2gb: read_bool_from_bit(data, 28),
            ola: read_bool_from_bit(data, 29),
            olb: read_bool_from_bit(data, 30),
            stst: read_bool_from_bit(data, 31),
        }
    }
}

impl<const M: u8> From<DrvStatus<M>> for u32 {
    fn from(data: DrvStatus<M>) -> Self {
        let mut value = 0;
        write_from_bit(&mut value, 0, 0x3ff, data.sg_result as u32);
        write_bool_to_bit(&mut value, 15, data.fsactive);
        write_from_bit(&mut value, 16, 0x1f, data.cs_actual as u32);
        write_bool_to_bit(&mut value, 24, data.stall_guard);
        write_bool_to_bit(&mut value, 25, data.ot);
        write_bool_to_bit(&mut value, 26, data.otpw);
        write_bool_to_bit(&mut value, 27, data.s2ga);
        write_bool_to_bit(&mut value, 28, data.s2gb);
        write_bool_to_bit(&mut value, 29, data.ola);
        write_bool_to_bit(&mut value, 30, data.olb);
        write_bool_to_bit(&mut value, 31, data.stst);
        value
    }
}

impl Register for DrvStatus<0> {
    fn addr() -> u8 {
        0x6F
    }
}
impl Register for DrvStatus<1> {
    fn addr() -> u8 {
        0x7F
    }
}

#[cfg(test)]
mod drv_status {
    use super::*;
    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(DrvStatus::<1> {
                sg_result: 666,
                stall_guard: true,
                ola: true,
                olb: true,
                ..Default::default()
            }),
            0x6100029A
        )
    }
    #[test]
    fn from_u32() {
        assert_eq!(
            DrvStatus::<1>::from(0x6100029A),
            DrvStatus::<1> {
                sg_result: 666,
                stall_guard: true,
                ola: true,
                olb: true,
                ..Default::default()
            },
        )
    }
}
