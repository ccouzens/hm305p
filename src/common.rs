pub enum Request {
    Read(Action),
    Write((Action, u16)),
}

pub enum Action {
    OnOff,
    CurrentmA,
    VoltagemV,
}

pub const MESSAGE_LENGTH: usize = 8;
pub const READ_RESPONSE_LENGTH: usize = 7;

pub const INDEX_ADDRESS: usize = 0;
pub const INDEX_READ_WRITE: usize = 1;
pub const INDEX_CONTROL_COMMAND_0: usize = 2;
pub const INDEX_CONTROL_COMMAND_1: usize = 3;
pub const INDEX_SET_VALUE_HIGH: usize = 4;
pub const INDEX_SET_VALUE_LOW: usize = 5;

pub const VALUE_ADDRESS: u8 = 0x01;
pub const VALUE_READ: u8 = 0x03;
pub const VALUE_WRITE: u8 = 0x06;

pub fn u8_high_low_get_u16(u8_high: u8, u8_low: u8) -> u16 {
    u8_high_get_u16(u8_high) + u8_low_get_u16(u8_low)
}

fn u8_high_get_u16(input: u8) -> u16 {
    (input as u16) << 8
}

fn u8_low_get_u16(input: u8) -> u16 {
    input as u16
}

pub fn u16_get_u8_high(input: u16) -> u8 {
    (input >> 8) as u8
}

pub fn u16_get_u8_low(input: u16) -> u8 {
    (input & 0x00ff) as u8
}

#[cfg(test)]
#[path = "./test_common.rs"]
mod test_common;
