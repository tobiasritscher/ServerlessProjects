use crate::model::Info;

pub async fn handle(info: Info, addr: Option<impl AsRef<str>>) {
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
            Ok(_) => {
                log::debug!("info was transmitted to <{}>", addr.as_ref());
            }
        }
    }
}
