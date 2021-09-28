use crate::hid;
pub mod meta;
pub mod time;
pub mod connection;
#[non_exhaustive]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, serde::Serialize, serde::Deserialize)]
#[repr(u16)]
pub enum Packet {
    Meta(meta::Packet),
    HID(hid::Packet),
}
