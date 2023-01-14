pub static LIGHT_LEVEL_MIN: i32 = 0;
pub static LIGHT_LEVEL_MAX: i32 = u16::MAX as i32;

pub static ROTATION_MIN: f32 = 0.0;
pub static ROTATION_MAX: f32 = 360.0;

pub static I2C_RANGE_MIN: u16 = 0x08;
pub static I2C_RANGE_MAX: u16 = 0x77;
pub static I2C_BYTES_PER_LIGHT: u8 = 2;
pub static I2C_LIGHT_LEVEL_START_OFFSET: u8 = 2;
