use crate::{storage::Creation, timestamp};
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

impl Creation for Info {
    fn timestamp(&self) -> &timestamp::TimeStamp {
        &self.timestamp
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct Stats {
    data: Vec<Info>,
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
