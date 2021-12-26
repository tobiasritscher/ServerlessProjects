use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Info {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_id: Option<String>,
    data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_data: Option<String>,
    #[serde(skip_deserializing)]
    timestamp: timestamp::TimeStamp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct Stats {
    data: Vec<Info>,
}

impl Stats {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

pub mod runner {

    use once_cell::sync::OnceCell;
    use parking_lot::RwLock;
    use std::{
        collections::VecDeque,
        ops::{Deref, DerefMut},
    };

    use super::Info;

    pub const RUNNER_TIMEOUT: u64 = 5;

    pub const DATA_MAX_AGE: std::time::Duration = std::time::Duration::from_secs(60 * 5);

    #[derive(serde::Serialize, Debug)]
    #[serde(transparent)]
    pub struct Blocks {
        data: VecDeque<Info>,
    }

    impl Deref for Blocks {
        type Target = VecDeque<Info>;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl DerefMut for Blocks {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
        }
    }

    pub fn get_stats_storage() -> &'static RwLock<Blocks> {
        static INSTANCE: OnceCell<RwLock<Blocks>> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            RwLock::new(Blocks {
                data: VecDeque::new(),
            })
        })
    }

    pub async fn runner() {
        let max_age = chrono::Duration::from_std(DATA_MAX_AGE)
            .expect("unable to convert std duration to chrono");
        let sleep_time = std::time::Duration::from_secs(RUNNER_TIMEOUT);
        loop {
            // not blocking wait
            tokio::time::sleep(sleep_time).await;

            let mut data = get_stats_storage().write();
            while !data.is_empty() {
                let be_dropped = if let Some(entry) = data.front() {
                    *entry.timestamp < chrono::Utc::now() - max_age
                } else {
                    false
                };
                if !be_dropped {
                    break;
                }
                // we now drop the front
                let _ = data.pop_front();
            }
        }
    }
}

mod timestamp {
    use std::ops::{Deref, DerefMut};

    #[derive(Debug, serde::Serialize, PartialEq, Eq, Clone)]
    #[repr(transparent)]
    pub struct TimeStamp(chrono::DateTime<chrono::Utc>);

    impl Default for TimeStamp {
        fn default() -> Self {
            cfg_if::cfg_if! {
                if #[cfg(test)] {
                    let time = get_timestamp();
                } else {
                    let time = chrono::Utc::now();
                }
            }
            Self(time)
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

    #[cfg(test)]
    const TEST_TIMESTAMP: &str = "2021-12-25T17:16:15.095455462Z";

    #[cfg(test)]
    fn get_timestamp() -> chrono::DateTime<chrono::Utc> {
        TEST_TIMESTAMP.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WEBHOOK: &str = r#"{"id": "test1", "data": "test2", "device_data": "test3"}"#;

    const STATS: &str = r#"[{"id":"test1","data":"test2","device_data":"test3","timestamp":"2021-12-25T17:16:15.095455462Z"}]"#;

    fn get_serialized_webhook() -> Info {
        Info {
            id: "test1".to_string(),
            region_id: None,
            data: "test2".to_string(),
            device_data: Some("test3".to_string()),
            timestamp: timestamp::TimeStamp::default(),
        }
    }

    fn get_serialized_stats() -> Stats {
        Stats {
            data: vec![get_serialized_webhook()],
        }
    }

    #[test]
    fn test_serialize_webhook() {
        let tested: Info = serde_json::from_str(WEBHOOK).unwrap();
        let serialized = get_serialized_webhook();

        assert_eq!(tested, serialized);
    }

    #[test]
    fn test_serialize_stats() {
        let exp = get_serialized_stats();
        let got = serde_json::to_string(&exp).expect("unable to convert");
        assert_eq!(got, STATS)
    }
}
