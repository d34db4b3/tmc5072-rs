#[inline]
pub(crate) fn read_bool_from_bit<T>(data: T, bit: T) -> bool
where
    T: core::ops::BitAnd<Output = T> + From<u8> + core::ops::Shl<Output = T> + core::cmp::PartialEq,
{
    (data & (T::from(1u8) << bit)) != T::from(0u8)
}

#[inline]
pub(crate) fn write_bool_to_bit<T>(data: &mut T, bit: T, value: bool)
where
    T: core::ops::BitOrAssign
        + From<u8>
        + core::ops::Shl<Output = T>
        + core::ops::Not<Output = T>
        + core::ops::BitAndAssign,
{
    if value {
        // set bit
        *data |= T::from(1u8) << bit;
    } else {
        // clear bit
        *data &= !(T::from(1u8) << bit);
    }
}

#[inline]
pub(crate) fn read_from_bit<T>(data: T, from: T, mask: T) -> T
where
    T: core::ops::Shr<Output = T> + core::ops::BitAnd<Output = T>,
{
    (data >> from) & mask
}

#[inline]
pub(crate) fn write_from_bit<T>(data: &mut T, from: T, mask: T, value: T)
where
    T: core::ops::Shl<Output = T>
        + core::ops::BitAnd<Output = T>
        + core::ops::BitXorAssign
        + core::ops::BitOrAssign
        + Copy,
{
    // clears bits according to mask
    *data ^= *data & (mask << from);
    // sets bits according to value
    *data |= value << from;
}

#[inline]
pub(crate) fn convert_to_signed_n(from: u32, bits: u8) -> i32 {
    if from >> (bits - 1) & 1 == 1 {
        (!((1 << bits) - 1)) | from as i32
    } else {
        from as i32
    }
}

#[inline]
pub(crate) fn convert_from_signed_n(from: i32, bits: u8) -> u32 {
    from as u32 & ((1 << bits) - 1)
}

#[cfg(test)]
mod bits {
    use super::*;
    #[test]
    fn read_bool() {
        let data = 0x01234567u32;
        // 0x0=0b0000
        assert_eq!(read_bool_from_bit(data, 31), false);
        assert_eq!(read_bool_from_bit(data, 30), false);
        assert_eq!(read_bool_from_bit(data, 29), false);
        assert_eq!(read_bool_from_bit(data, 28), false);
        // 0x1=0b0001
        assert_eq!(read_bool_from_bit(data, 27), false);
        assert_eq!(read_bool_from_bit(data, 26), false);
        assert_eq!(read_bool_from_bit(data, 25), false);
        assert_eq!(read_bool_from_bit(data, 24), true);
        // 0x2=0b0010
        assert_eq!(read_bool_from_bit(data, 23), false);
        assert_eq!(read_bool_from_bit(data, 22), false);
        assert_eq!(read_bool_from_bit(data, 21), true);
        assert_eq!(read_bool_from_bit(data, 20), false);
        // 0x3=0b0011
        assert_eq!(read_bool_from_bit(data, 19), false);
        assert_eq!(read_bool_from_bit(data, 18), false);
        assert_eq!(read_bool_from_bit(data, 17), true);
        assert_eq!(read_bool_from_bit(data, 16), true);
        // 0x4=0b0100
        assert_eq!(read_bool_from_bit(data, 15), false);
        assert_eq!(read_bool_from_bit(data, 14), true);
        assert_eq!(read_bool_from_bit(data, 13), false);
        assert_eq!(read_bool_from_bit(data, 12), false);
        // 0x5=0b0101
        assert_eq!(read_bool_from_bit(data, 11), false);
        assert_eq!(read_bool_from_bit(data, 10), true);
        assert_eq!(read_bool_from_bit(data, 9), false);
        assert_eq!(read_bool_from_bit(data, 8), true);
        // 0x6=0b0110
        assert_eq!(read_bool_from_bit(data, 7), false);
        assert_eq!(read_bool_from_bit(data, 6), true);
        assert_eq!(read_bool_from_bit(data, 5), true);
        assert_eq!(read_bool_from_bit(data, 4), false);
        // 0x6=0b0111
        assert_eq!(read_bool_from_bit(data, 3), false);
        assert_eq!(read_bool_from_bit(data, 2), true);
        assert_eq!(read_bool_from_bit(data, 1), true);
        assert_eq!(read_bool_from_bit(data, 0), true);
    }
    #[test]
    fn write_bool() {
        let mut data = 0xffffffffu32;
        // 0x0=0b0000
        write_bool_to_bit(&mut data, 31, false);
        write_bool_to_bit(&mut data, 30, false);
        write_bool_to_bit(&mut data, 29, false);
        write_bool_to_bit(&mut data, 28, false);
        // 0x1=0b0001
        write_bool_to_bit(&mut data, 27, false);
        write_bool_to_bit(&mut data, 26, false);
        write_bool_to_bit(&mut data, 25, false);
        write_bool_to_bit(&mut data, 24, true);
        // 0x2=0b0010
        write_bool_to_bit(&mut data, 23, false);
        write_bool_to_bit(&mut data, 22, false);
        write_bool_to_bit(&mut data, 21, true);
        write_bool_to_bit(&mut data, 20, false);
        // 0x3=0b0011
        write_bool_to_bit(&mut data, 19, false);
        write_bool_to_bit(&mut data, 18, false);
        write_bool_to_bit(&mut data, 17, true);
        write_bool_to_bit(&mut data, 16, true);
        // 0x4=0b0100
        write_bool_to_bit(&mut data, 15, false);
        write_bool_to_bit(&mut data, 14, true);
        write_bool_to_bit(&mut data, 13, false);
        write_bool_to_bit(&mut data, 12, false);
        // 0x5=0b0101
        write_bool_to_bit(&mut data, 11, false);
        write_bool_to_bit(&mut data, 10, true);
        write_bool_to_bit(&mut data, 9, false);
        write_bool_to_bit(&mut data, 8, true);
        // 0x6=0b0110
        write_bool_to_bit(&mut data, 7, false);
        write_bool_to_bit(&mut data, 6, true);
        write_bool_to_bit(&mut data, 5, true);
        write_bool_to_bit(&mut data, 4, false);
        // 0x6=0b0111
        write_bool_to_bit(&mut data, 3, false);
        write_bool_to_bit(&mut data, 2, true);
        write_bool_to_bit(&mut data, 1, true);
        write_bool_to_bit(&mut data, 0, true);
        assert_eq!(data, 0x01234567);
    }
    #[test]
    fn write() {
        let mut data = 0xffffffffu32;
        write_from_bit(&mut data, 8, 0xff, 0x8f);
        assert_eq!(data, 0xffff8fff);
        write_from_bit(&mut data, 16, 0xff00, 0x5566);
        assert_eq!(data, 0x55ff8fff);
        write_from_bit(&mut data, 0, 0xffffffff, 0x55667788);
        assert_eq!(data, 0x55667788);
    }
    #[test]
    fn read() {
        assert_eq!(read_from_bit(0xffff8fffu32, 8, 0xff,), 0x8f);
        assert_eq!(read_from_bit(0x55ff8fffu32, 16, 0xff00), 0x5500);
        assert_eq!(read_from_bit(0x55667788u32, 0, 0xffffffff), 0x55667788);
    }

    #[test]
    fn to_signed_n() {
        assert_eq!(convert_to_signed_n(0x0000, 8), 0);
        assert_eq!(convert_to_signed_n(0x007f, 8), 127);
        assert_eq!(convert_to_signed_n(0x0080, 8), -128);
        assert_eq!(convert_to_signed_n(0x00ff, 8), -1);
        assert_eq!(convert_to_signed_n(0x0000, 9), 0);
        assert_eq!(convert_to_signed_n(0x00ff, 9), 255);
        assert_eq!(convert_to_signed_n(0x0100, 9), -256);
        assert_eq!(convert_to_signed_n(0x01ff, 9), -1);
    }
    #[test]
    fn from_signed_n() {
        assert_eq!(convert_from_signed_n(0, 8), 0x0000);
        assert_eq!(convert_from_signed_n(127, 8), 0x007f);
        assert_eq!(convert_from_signed_n(-128, 8), 0x0080);
        assert_eq!(convert_from_signed_n(-1, 8), 0x00ff);
        assert_eq!(convert_from_signed_n(0, 9), 0x0000);
        assert_eq!(convert_from_signed_n(255, 9), 0x00ff);
        assert_eq!(convert_from_signed_n(-256, 9), 0x0100);
        assert_eq!(convert_from_signed_n(-1, 9), 0x01ff);
    }
}
