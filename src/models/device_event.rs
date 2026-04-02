
#[derive(Debug)]
pub enum DeviceEvent {
    Key(u8),
    Wheel(u8, u8), // Wheel ID, Value
}
