use futures::FutureExt;
use std::{collections::VecDeque, sync::Arc};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    oneshot,
};

use crate::timestamp;

pub const RUNNER_TIMEOUT: u64 = 5;

pub const DATA_MAX_AGE: std::time::Duration = std::time::Duration::from_secs(60 * 5);

pub trait Storable: serde::Serialize + std::fmt::Debug + std::clone::Clone {
    fn timestamp(&self) -> &timestamp::TimeStamp;
}
#[derive(serde::Serialize, Debug)]
#[serde(transparent)]
pub struct Data<T>(Vec<Arc<T>>);

struct InnerStorage<T> {
    data: VecDeque<Arc<T>>,
}

impl<T: Storable> InnerStorage<T> {
    fn new() -> Self {
        let data = VecDeque::new();
        Self { data }
    }

    fn store(&mut self, data: T) {
        // SAFETY: unwrap is safe here as there is no way for this function
        // to fail, given that both sender and receiver are bound to this
        // struct.
        self.data.push_back(Arc::new(data));
    }

    fn pop(&mut self) {
        // get lock
        while !self.data.is_empty() {
            let be_dropped = if let Some(entry) = self.data.front() {
                **entry.timestamp() < time::OffsetDateTime::now_utc() - DATA_MAX_AGE
            } else {
                false
            };
            if !be_dropped {
                break;
            }
            // we now drop the front
            let _ = self.data.pop_front();
        }
    }

    fn inner(&self) -> Data<T> {
        // This is cheap, although there is some allocation needed
        Data(self.data.iter().cloned().collect())
    }
}

#[derive(Clone)]
struct SeriSend<T>(UnboundedSender<oneshot::Sender<Data<T>>>);

impl<T: std::fmt::Debug> SeriSend<T> {
    async fn send(&self) -> Result<Data<T>, anyhow::Error> {
        let (send, recv) = oneshot::channel();
        self.0
            .send(send)
            .map_err(|err| anyhow::anyhow!(format!("Unable to send the data because <{}>", err)))?;
        let data = recv.await?;
        Ok(data)
    }
}

struct SeriRecv<T>(UnboundedReceiver<oneshot::Sender<Data<T>>>);

impl<T> SeriRecv<T> {
    async fn recv(&mut self) -> Option<oneshot::Sender<Data<T>>> {
        self.0.recv().await
    }
}

fn new_seri<T>() -> (SeriSend<T>, SeriRecv<T>) {
    let (s, r) = unbounded_channel();
    (SeriSend(s), SeriRecv(r))
}

#[derive(Clone)]
pub struct Storage<T> {
    send: UnboundedSender<T>,
    seri_req: SeriSend<T>,
}

impl<T: std::fmt::Debug> Storage<T> {
    fn new(send: UnboundedSender<T>) -> (Self, SeriRecv<T>) {
        let (s, r) = new_seri();
        (Self { send, seri_req: s }, r)
    }

    pub async fn store(&self, data: T) {
        // SAFETY: unwrap is safe here as there is no way for this function
        // to fail, given that both sender and receiver are bound to this
        // struct.
        self.send
            .send(data)
            .expect("Sending the info has failed... This should never ever happen...");
    }

    pub async fn data(&self) -> Data<T> {
        self.seri_req
            .send()
            .await
            .expect("Sending back the requested data failed")
    }
}

pub struct StorageHandler<T> {
    storage: InnerStorage<T>,
    storage_sender: Storage<T>,
    data_recv: UnboundedReceiver<T>,
    seri_recv: SeriRecv<T>,
}

impl<T: Storable> StorageHandler<T> {
    pub fn new() -> Self {
        let (send, data_recv) = unbounded_channel();
        let (storage_sender, seri_recv) = Storage::new(send);
        let storage = InnerStorage::new();

        Self {
            storage,
            storage_sender,
            seri_recv,
            data_recv,
        }
    }

    pub fn get_storage(&self) -> Storage<T> {
        self.storage_sender.clone()
    }

    fn handle_data_req(&self, snd: Option<oneshot::Sender<Data<T>>>) {
        if let Some(snd) = snd {
            snd.send(self.storage.inner())
                .expect("Unable to send data, as receiver died...");
        }
    }

    async fn handle_data<F, Fut>(&mut self, data: Option<T>, info_handler: &F)
    where
        F: Fn(T) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        match data {
            Some(data) => {
                log::debug!("inserting data into local storage");
                self.storage.store(data.clone());
                log::debug!("calling callback function");
                info_handler(data).await;
            }
            None => panic!("Something went wrong, with receving the data..."),
        }
    }

    pub fn handle_pop(&mut self) {
        self.storage.pop()
    }

    pub async fn handler<F, Fut>(&mut self, info_handler: F)
    where
        F: Fn(T) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let sleep_time = std::time::Duration::from_secs(RUNNER_TIMEOUT);

        loop {
            // none blocking wait
            futures::select! {
                _ = tokio::time::sleep(sleep_time).fuse() => self.handle_pop(),
                data = self.data_recv.recv().fuse() => self.handle_data(data, &info_handler).await,
                send = self.seri_recv.recv().fuse() => self.handle_data_req(send)
            }
        }
    }
}
