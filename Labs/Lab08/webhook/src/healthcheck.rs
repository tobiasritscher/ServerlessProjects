use std::process;

use reqwest::StatusCode;

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
enum DockerHealthcheck {
    Success = 0,
    Unhealthy = 1,
}

pub async fn check(addr: &str) -> ! {
    log::debug!("checking request");
    let res = reqwest::get(addr).await;
    log::debug!("reqest response <{:?}>", res);

    let result = match res {
        Ok(req) => match req.status() {
            StatusCode::OK => {
                log::debug!("Status is a 200");
                DockerHealthcheck::Success
            }
            _ => {
                log::debug!("Status is not a 200");
                DockerHealthcheck::Unhealthy
            }
        },
        Err(_) => DockerHealthcheck::Unhealthy,
    };

    log::debug!("reqest result <{:?}> - <{}>", result, result as i32);
    process::exit(result as _);
}
