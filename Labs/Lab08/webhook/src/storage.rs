use futures::FutureExt;
use once_cell::sync::OnceCell;
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{model::Info, timestamp};

pub const RUNNER_TIMEOUT: u64 = 5;

pub const DATA_MAX_AGE: std::time::Duration = std::time::Duration::from_secs(60 * 5);

pub trait Creation: serde::Serialize + std::fmt::Debug {
    fn timestamp(&self) -> &timestamp::TimeStamp;
}

#[derive(serde::Serialize, Debug)]
#[serde(transparent)]
pub struct Blocks<T: Creation>(VecDeque<T>);

impl<T: Creation> Blocks<T> {
    fn new() -> Self {
        Self(VecDeque::new())
    }
}

impl<T: Creation> Deref for Blocks<T> {
    type Target = VecDeque<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Creation> DerefMut for Blocks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct Storage<T: Creation> {
    data: RwLock<Blocks<T>>,
    send: UnboundedSender<T>,
    recv: Mutex<UnboundedReceiver<T>>,
}

impl<T: Creation> Storage<T> {
    fn new() -> Self {
        let (send, recv) = unbounded_channel();
        let recv = Mutex::new(recv);
        let data = RwLock::new(Blocks::new());
        Self { data, send, recv }
    }

    fn store(&self, data: T) {
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
        let data = data.expect("Receiving the info has failed... This should never ever happen...");
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
                **entry.timestamp() < chrono::Utc::now() - max_age
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

pub fn serialized() -> RwLockReadGuard<'static, Blocks<Info>> {
    get_storage().data.read()
}

fn get_storage() -> &'static Storage<Info> {
    static INSTANCE: OnceCell<Storage<Info>> = OnceCell::new();
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
