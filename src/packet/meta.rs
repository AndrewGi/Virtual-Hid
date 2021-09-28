
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub enum Packet {
    Ping,
    Pong,
    TimeAsk,
    Time(super::time::Time)
}