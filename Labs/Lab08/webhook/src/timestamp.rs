use std::ops::{Deref, DerefMut};
use time::OffsetDateTime;

#[derive(Debug, serde::Serialize, PartialEq, Eq, Clone)]
#[serde(transparent)]
pub struct TimeStamp(#[serde(serialize_with = "serializer")] OffsetDateTime);

impl TimeStamp {
    fn get_timestamp() -> time::OffsetDateTime {
        cfg_if::cfg_if! {
            if #[cfg(test)] {
                use time::format_description::well_known;
                const TEST_TIMESTAMP: &str = "2021-12-25T17:16:15.095455462Z";
                OffsetDateTime::parse(TEST_TIMESTAMP, &well_known::Rfc3339).unwrap()
            } else {
                OffsetDateTime::now_utc()
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
    type Target = OffsetDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TimeStamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn serializer<S>(val: &OffsetDateTime, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use time::format_description::well_known::Rfc3339;
    // SAFETY: We can unwrap here without any problems
    // as we know that the well_known formats will
    // succed.
    let f = &val.format(&Rfc3339).unwrap();
    ser.serialize_str(f)
}
