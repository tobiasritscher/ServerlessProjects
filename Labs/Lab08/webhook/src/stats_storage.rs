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

pub mod storage {

    use futures::FutureExt;
    use once_cell::sync::OnceCell;
    use parking_lot::{Mutex, RwLock, RwLockReadGuard};
    use std::{
        collections::VecDeque,
        ops::{Deref, DerefMut},
    };
    use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

    use super::Info;

    pub const RUNNER_TIMEOUT: u64 = 5;

    pub const DATA_MAX_AGE: std::time::Duration = std::time::Duration::from_secs(60 * 5);

    #[derive(serde::Serialize, Debug)]
    #[serde(transparent)]
    pub struct Blocks(VecDeque<Info>);

    impl Blocks {
        fn new() -> Self {
            Self(VecDeque::new())
        }
    }

    impl Deref for Blocks {
        type Target = VecDeque<Info>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Blocks {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    static INSTANCE: OnceCell<Storage> = OnceCell::new();

    struct Storage {
        data: RwLock<Blocks>,
        send: UnboundedSender<Info>,
        recv: Mutex<UnboundedReceiver<Info>>,
    }

    impl Storage {
        fn new() -> Self {
            let (send, recv) = unbounded_channel();
            let recv = Mutex::new(recv);
            let data = RwLock::new(Blocks::new());
            Self { data, send, recv }
        }

        fn store(&self, data: Info) {
            // SAFETY: unwrap is safe here as there is no way for this function
            // to fail, given that both sender and receiver are bound to this
            // struct.
            self.send
                .send(data)
                .expect("Sending the info has failed... This should never ever happen...");
        }

        async fn resv(&self) {
            // SAFETY: unwrap is safe here as there is no way for this function
            // to fail, given that both sender and receiver are bound to this
            // struct.
            let data = self.recv.lock().recv().await;
            let data =
                data.expect("Receiving the info has failed... This should never ever happen...");
            self.data.write().push_back(data);
        }

        fn pop(&self) {
            // SAFETY: unwrap is safe here as there is no way for this function
            // to fail, as we know that data_max_age works
            let max_age = chrono::Duration::from_std(DATA_MAX_AGE)
                .expect("unable to convert std duration to chrono");

            // get lock
            let mut data = self.data.write();
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

    pub fn store(data: Info) {
        get_storage().store(data);
    }

    pub fn serialized() -> RwLockReadGuard<'static, Blocks> {
        get_storage().data.read()
    }

    fn get_storage() -> &'static Storage {
        INSTANCE.get_or_init(Storage::new)
    }

    pub async fn handler() {
        let sleep_time = std::time::Duration::from_secs(RUNNER_TIMEOUT);
        let storage = get_storage();

        loop {
            // none blocking wait
            futures::select! {
                 _ = tokio::time::sleep(sleep_time).fuse() => storage.pop(),
                _ = storage.resv().fuse() => {}
            };
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
