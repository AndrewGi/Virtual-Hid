use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Time {
    pub unix_time_ms: u64
}
impl Time {
    pub fn current_time() -> Self {
        let ms = SystemTime::now().duration_since(UNIX_EPOCH).ok().unwrap_or_default().as_millis().try_into().expect("u64 overflowed for unix time");
        Self {
            unix_time_ms: ms
        }
    }
}