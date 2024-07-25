use oneshot::{Receiver, TryRecvError};
use std::thread::{self, JoinHandle};

use crate::compiler::button::Button;

pub(super) struct Watcher {
    receiver: Receiver<()>,
    handle: JoinHandle<()>,
}

impl Watcher {
    pub(super) fn new(button: Button) -> Self {
        let (sender, receiver) = oneshot::channel::<()>();
        let handle = button
            .listen_once(1, move || {
                tracing::info!("Halt key pressed");
                match sender.send(()) {
                    Ok(()) => tracing::debug!("Halt message sent"),
                    Err(_) => {
                        tracing::debug!(
                            "Failed to send halt message as the receiver no longer exists"
                        )
                    }
                };
            })
            .unwrap();
        Self { receiver, handle }
    }

    pub(super) fn check(&self) -> bool {
        match self.receiver.try_recv() {
            Ok(()) => {
                tracing::info!("Halt message received");
                true
            }
            Err(TryRecvError::Disconnected) => {
                tracing::error!("Watcher thread disconnected");
                true
            }
            Err(TryRecvError::Empty) => false,
        }
    }

    pub(super) fn clean(self) {
        match self.handle.join() {
            Ok(()) => tracing::debug!("Watcher thread successfully joined"),
            Err(err) => tracing::warn!("Watcher thread panicked '{:?}'", err),
        }
    }
}
