use std::thread::{self, JoinHandle};
use oneshot::{Receiver, SendError, TryRecvError};

use crate::compiler::button::Button;

pub(super) struct Watcher {
    receiver: Receiver<()>,
    handle: JoinHandle<()>,
}

impl Watcher {
    pub(super) fn new(button: Button) -> Self {
        let (sender, receiver) = oneshot::channel::<()>();
        let handle = thread::spawn(move || {
            button.bind(move || {
                tracing::info!("Global halt button pressed");
                button.unbind();
            });
            inputbot::handle_input_events(true);
            tracing::debug!("Global halt button unbind");

            match sender.send(()) {
                Ok(()) => tracing::debug!("Halt message sent"),
                Err(_) => tracing::debug!("Failed to send halt message as the receiver no longer exists")
            };
        });
        Self { receiver, handle }
    }

    pub(super) fn check(&self) -> bool {
        match self.receiver.try_recv() {
            Ok(()) => {
                tracing::info!("Halt message received");
                true
            },
            Err(TryRecvError::Disconnected) => {
                tracing::error!("Watcher thread disconnected");
                true
            }
            Err(TryRecvError::Empty) => {
                false
            }
        }
    }

    pub(super) fn clean(self) {
        match self.handle.join() {
            Ok(()) => tracing::debug!("Watcher thread successfully joined"),
            Err(err) => tracing::warn!("Watcher thread panicked '{:?}'", err)
        }
    }
}