use std::ops::{Deref, DerefMut};

#[derive(Debug, serde::Serialize, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct TimeStamp(chrono::DateTime<chrono::Utc>);

impl TimeStamp {
    fn get_timestamp() -> chrono::DateTime<chrono::Utc> {
        cfg_if::cfg_if! {
            if #[cfg(test)] {
                const TEST_TIMESTAMP: &str = "2021-12-25T17:16:15.095455462Z";
                TEST_TIMESTAMP.parse().unwrap()
            } else {
                chrono::Utc::now()
            }
        }
    }
}

impl Default for TimeStamp {
    fn default() -> Self {
        Self(Self::get_timestamp())
    }
}

impl Deref for TimeStamp {
    type Target = chrono::DateTime<chrono::Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TimeStamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
