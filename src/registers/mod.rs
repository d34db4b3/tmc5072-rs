//! TMC5072 registers

// TODO: use macro, bitfields or packed_struct for registers

pub mod encoder_registers;
pub mod general_configuration_register;
pub mod microstep_table_register;
pub mod motor_driver_register;
pub mod ramp_generator_driver_feature_control_register;
pub mod ramp_generator_register;
pub mod voltage_pwm_mode_stealth_chop;

/// Address flag for read operation
pub(crate) const READ_FLAG: u8 = 0x00;
/// Address flag for write operation
pub(crate) const WRITE_FLAG: u8 = 0x80;
/// IC Version expected
pub const IC_VERSION: u8 = 0x10;

/// Register trait
///
/// Imposes u32 conversion and addressing capabilities
pub trait Register
where
    u32: From<Self>,
    Self: From<u32>,
    Self: Copy,
{
    /// Actual address of the register
    fn addr() -> u8;
}

#[cfg(test)]
mod addresses {
    use super::*;

    #[test]
    fn encoder_registers() {
        assert_eq!(encoder_registers::EncMode::<0>::addr(), 0x38);
        assert_eq!(encoder_registers::EncMode::<1>::addr(), 0x58);
        assert_eq!(encoder_registers::XEnc::<0>::addr(), 0x39);
        assert_eq!(encoder_registers::XEnc::<1>::addr(), 0x59);
        assert_eq!(encoder_registers::EncConst::<0>::addr(), 0x3A);
        assert_eq!(encoder_registers::EncConst::<1>::addr(), 0x5A);
        assert_eq!(encoder_registers::EncStatus::<0>::addr(), 0x3B);
        assert_eq!(encoder_registers::EncStatus::<1>::addr(), 0x5B);
        assert_eq!(encoder_registers::EncLatch::<0>::addr(), 0x3C);
        assert_eq!(encoder_registers::EncLatch::<1>::addr(), 0x5C);
    }

    #[test]
    fn general_configuration_register() {
        assert_eq!(general_configuration_register::GConf::addr(), 0x00);
        assert_eq!(general_configuration_register::GStat::addr(), 0x01);
        assert_eq!(general_configuration_register::IfCnt::addr(), 0x02);
        assert_eq!(general_configuration_register::SlaveConf::addr(), 0x03);
        assert_eq!(general_configuration_register::Input::addr(), 0x04);
        assert_eq!(general_configuration_register::Output::addr(), 0x04);
        assert_eq!(general_configuration_register::XCompare::addr(), 0x05);
    }

    #[test]
    fn microstep_table_register() {
        assert_eq!(microstep_table_register::MsLut0::addr(), 0x60);
        assert_eq!(microstep_table_register::MsLut1::addr(), 0x61);
        assert_eq!(microstep_table_register::MsLut2::addr(), 0x62);
        assert_eq!(microstep_table_register::MsLut3::addr(), 0x63);
        assert_eq!(microstep_table_register::MsLut4::addr(), 0x64);
        assert_eq!(microstep_table_register::MsLut5::addr(), 0x65);
        assert_eq!(microstep_table_register::MsLut6::addr(), 0x66);
        assert_eq!(microstep_table_register::MsLut7::addr(), 0x67);
        assert_eq!(microstep_table_register::MsLutSel::addr(), 0x68);
        assert_eq!(microstep_table_register::MsLutStart::addr(), 0x69);
    }

    #[test]
    fn motor_driver_register() {
        assert_eq!(motor_driver_register::MsCnt::<0>::addr(), 0x6A);
        assert_eq!(motor_driver_register::MsCnt::<1>::addr(), 0x7A);
        assert_eq!(motor_driver_register::MsCurAct::<0>::addr(), 0x6B);
        assert_eq!(motor_driver_register::MsCurAct::<1>::addr(), 0x7B);
        assert_eq!(motor_driver_register::ChopConf::<0>::addr(), 0x6C);
        assert_eq!(motor_driver_register::ChopConf::<1>::addr(), 0x7C);
        assert_eq!(motor_driver_register::CoolConf::<0>::addr(), 0x6D);
        assert_eq!(motor_driver_register::CoolConf::<1>::addr(), 0x7D);
        assert_eq!(motor_driver_register::DcCtrl::<0>::addr(), 0x6E);
        assert_eq!(motor_driver_register::DcCtrl::<1>::addr(), 0x7E);
        assert_eq!(motor_driver_register::DrvStatus::<0>::addr(), 0x6F);
        assert_eq!(motor_driver_register::DrvStatus::<1>::addr(), 0x7F);
    }

    #[test]
    fn ramp_generator_driver_feature_control_register() {
        assert_eq!(
            ramp_generator_driver_feature_control_register::IHoldIRun::<0>::addr(),
            0x30
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::IHoldIRun::<1>::addr(),
            0x50
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::VCoolThrs::<0>::addr(),
            0x31
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::VCoolThrs::<1>::addr(),
            0x51
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::VHigh::<0>::addr(),
            0x32
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::VHigh::<1>::addr(),
            0x52
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::VDcMin::<0>::addr(),
            0x33
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::VDcMin::<1>::addr(),
            0x53
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::SwMode::<0>::addr(),
            0x34
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::SwMode::<1>::addr(),
            0x54
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::RampStat::<0>::addr(),
            0x35
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::RampStat::<1>::addr(),
            0x55
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::XLatch::<0>::addr(),
            0x36
        );
        assert_eq!(
            ramp_generator_driver_feature_control_register::XLatch::<1>::addr(),
            0x56
        );
    }

    #[test]
    fn ramp_generator_register() {
        assert_eq!(ramp_generator_register::RampMode::<0>::addr(), 0x20);
        assert_eq!(ramp_generator_register::RampMode::<1>::addr(), 0x40);
        assert_eq!(ramp_generator_register::XActual::<0>::addr(), 0x21);
        assert_eq!(ramp_generator_register::XActual::<1>::addr(), 0x41);
        assert_eq!(ramp_generator_register::VActual::<0>::addr(), 0x22);
        assert_eq!(ramp_generator_register::VActual::<1>::addr(), 0x42);
        assert_eq!(ramp_generator_register::VStart::<0>::addr(), 0x23);
        assert_eq!(ramp_generator_register::VStart::<1>::addr(), 0x43);
        assert_eq!(ramp_generator_register::A1::<0>::addr(), 0x24);
        assert_eq!(ramp_generator_register::A1::<1>::addr(), 0x44);
        assert_eq!(ramp_generator_register::V1::<0>::addr(), 0x25);
        assert_eq!(ramp_generator_register::V1::<1>::addr(), 0x45);
        assert_eq!(ramp_generator_register::AMax::<0>::addr(), 0x26);
        assert_eq!(ramp_generator_register::AMax::<1>::addr(), 0x46);
        assert_eq!(ramp_generator_register::VMax::<0>::addr(), 0x27);
        assert_eq!(ramp_generator_register::VMax::<1>::addr(), 0x47);
        assert_eq!(ramp_generator_register::DMax::<0>::addr(), 0x28);
        assert_eq!(ramp_generator_register::DMax::<1>::addr(), 0x48);
        assert_eq!(ramp_generator_register::D1::<0>::addr(), 0x2a);
        assert_eq!(ramp_generator_register::D1::<1>::addr(), 0x4a);
        assert_eq!(ramp_generator_register::VStop::<0>::addr(), 0x2b);
        assert_eq!(ramp_generator_register::VStop::<1>::addr(), 0x4b);
        assert_eq!(ramp_generator_register::TZeroWait::<0>::addr(), 0x2c);
        assert_eq!(ramp_generator_register::TZeroWait::<1>::addr(), 0x4c);
        assert_eq!(ramp_generator_register::XTarget::<0>::addr(), 0x2d);
        assert_eq!(ramp_generator_register::XTarget::<1>::addr(), 0x4d);
    }

    #[test]
    fn voltage_pwm_mode_stealth_chop() {
        assert_eq!(voltage_pwm_mode_stealth_chop::PwmConf::<0>::addr(), 0x10);
        assert_eq!(voltage_pwm_mode_stealth_chop::PwmConf::<1>::addr(), 0x18);
        assert_eq!(voltage_pwm_mode_stealth_chop::PwmStatus::<0>::addr(), 0x11);
        assert_eq!(voltage_pwm_mode_stealth_chop::PwmStatus::<1>::addr(), 0x19);
    }
}
