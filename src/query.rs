use std::sync::mpsc::{Receiver, TryRecvError};

#[derive(Debug, Default)]
pub enum QueryState<T, E> {
    #[default]
    Idle,
    Loading,
    Success(T),
    Error(E),
}

#[derive(Debug)]
pub struct AsyncQuery<T, E> {
    pub state: QueryState<T, E>,
    receiver: Option<Receiver<Result<T, E>>>,
}

impl<T, E> Default for  AsyncQuery<T, E> {
    fn default() -> Self {
        Self { state: QueryState::Idle, receiver: None }
    }
}

impl<T: Send + 'static, E: Send + 'static> AsyncQuery<T, E> {
    pub fn fetch<F>(&mut self, fut: F)
    where
        F: std::future::Future<Output = Result<T, E>> + Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::channel();

        self.state = QueryState::Loading;
        self.receiver = Some(rx);

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        std::thread::spawn(move || {
            runtime.block_on(async {
                let result = fut.await;
                let _ = tx.send(result);
            });
        });
    }

    pub fn poll(&mut self) {
        if let Some(rx) = &self.receiver {
            match rx.try_recv() {
                Ok(Ok(data)) => {
                    self.state = QueryState::Success(data);
                    self.receiver = None;
                }
                Ok(Err(err)) => {
                    self.state = QueryState::Error(err);
                    self.receiver = None;
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    panic!("Async task disconnected")
                }
            }
        }
    }

    pub fn is_ready(&self) -> bool {
        !matches!(self.state, QueryState::Loading)
    }
}