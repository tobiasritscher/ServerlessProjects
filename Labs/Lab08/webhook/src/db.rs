use reqwest::StatusCode;

use crate::model::Info;

pub async fn handle(info: Info, addr: Option<impl AsRef<str>>) {
    log::debug!("data handle requested");
    if let Some(addr) = addr {
        let res = reqwest::Client::new()
            .post(addr.as_ref())
            .json(&info)
            .send()
            .await;

        match res {
            Err(err) => match err.status() {
                Some(code) => {
                    log::info!("Unable to send data to DB: status - <{}>", code);
                }
                None => {
                    log::warn!("Unable to send data to DB: Unexpected error <{}>", err);
                }
            },
            Ok(req) => match req.status() {
                StatusCode::OK => log::debug!("info was transmitted to <{}>", addr.as_ref()),
                _ => log::warn!(
                    "tried to send JSON(<{}>) unable to send information to DB: <{:?}>",
                    serde_json::to_string_pretty(&info).unwrap(),
                    req
                ),
            },
        }
    }
}
