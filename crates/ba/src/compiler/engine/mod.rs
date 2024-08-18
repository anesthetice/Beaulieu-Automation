use std::time::Duration;

use crate::compiler::expression::adapt_expressions;

use super::{button::Button, expression::Expression};

mod watcher;
use anyhow::Context;
use oneshot::TryRecvError;
use watcher::Watcher;

pub struct Engine {
    inner: Vec<Expression>,
    watcher: Watcher,
    delay: std::time::Duration,
    buttons_in_use: Vec<Button>,
}

impl Engine {
    // 50 milliseconds
    const MAIN_THREAD_DELAY_BETWEEN_CHECKS: Duration = Duration::new(0, 50000000);

    pub fn new(
        mut expressions: Vec<Expression>,
        host_resolution: (i32, i32),
    ) -> anyhow::Result<Self> {
        let script_resolution = expressions
            .iter()
            .find_map(|expr| match expr {
                Expression::Resolution(res) => Some(*res),
                _ => None,
            })
            .ok_or_else(|| {
                tracing::error!("GLOBAL_RESOLUTION definition missing");
                anyhow::anyhow!("Failed to create engine")
            })?;
        tracing::debug!(
            "script resolution = {}x{}",
            script_resolution.0,
            script_resolution.1
        );

        let delay_between_actions = expressions
            .iter()
            .find_map(|expr| match expr {
                Expression::DelayBetweenActions(val) => {
                    Some(std::time::Duration::from_millis(*val))
                }
                _ => None,
            })
            .ok_or_else(|| {
                tracing::error!("DELAY_BETWEEN_ACTIONS definition missing");
                anyhow::anyhow!("Failed to create engine")
            })?;
        tracing::debug!(
            "delay between actions = {} ms",
            delay_between_actions.as_millis()
        );

        let global_halt_key = expressions
            .iter()
            .find_map(|expr| match expr {
                Expression::GlobalHaltKey(button) => Some(*button),
                _ => None,
            })
            .ok_or_else(|| {
                tracing::error!("GLOBAL_HALT_KEY definition missing");
                anyhow::anyhow!("Failed to create engine")
            })?;
        tracing::debug!("global halt key = {:?}", global_halt_key);

        // extract and launch binds
        let mut buttons_in_use = vec![global_halt_key];
        while let Some(idx) = expressions
            .iter()
            .position(|expr| matches!(expr, Expression::Bind(..)))
        {
            let (button, sub_expressions) = match expressions.remove(idx) {
                Expression::Bind(key, sub_expressions) => (key, sub_expressions),
                _ => unreachable!(),
            };
            let sub_expressions =
                adapt_expressions(sub_expressions, host_resolution, script_resolution);

            tracing::info!("Attempting to bind '{:?}' as a HotKey", button);
            tracing::trace!("with subexpressions {:?}", sub_expressions);
            if buttons_in_use.contains(&button) {
                Err(anyhow::anyhow!(
                    "Failed to bind the button '{:?}' as it is already in use",
                    &button
                ))?
            }
            buttons_in_use.push(button);
            button.detached_hotkey(move || {
                for expr in sub_expressions.iter() {
                    expr.execute();
                    std::thread::sleep(delay_between_actions);
                }
            })?;
        }

        let expressions = adapt_expressions(expressions, host_resolution, script_resolution);

        Ok(Self {
            inner: expressions,
            watcher: Watcher::new(global_halt_key)?,
            delay: delay_between_actions,
            buttons_in_use,
        })
    }

    pub fn start(self, nb_cycles: usize) -> anyhow::Result<()> {
        let executor_receiver =
            Self::spawn_executor(self.inner, self.delay, nb_cycles, self.buttons_in_use)
                .context("Failed to spawn executor thread")?;
        loop {
            if self.watcher.check() {
                self.watcher.post_halt();
                return Ok(());
            }
            match executor_receiver.try_recv() {
                Ok(()) => return Ok(()),
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => {
                    tracing::error!("Executor thread disconnected");
                    Err(anyhow::anyhow!(
                        "Program halted, executor thread stopped unexpectedly"
                    ))?
                }
            }
            std::thread::sleep(Self::MAIN_THREAD_DELAY_BETWEEN_CHECKS);
        }
    }

    pub fn spawn_executor(
        expressions: Vec<Expression>,
        delay: std::time::Duration,
        nb_cycles: usize,
        buttons_in_use: Vec<Button>,
    ) -> anyhow::Result<oneshot::Receiver<()>> {
        let (sender, receiver) = oneshot::channel::<()>();

        std::thread::Builder::new()
            .name(String::from("Executor"))
            .spawn(move || {
                'outer: for cycle_idx in 0..nb_cycles {
                    tracing::info!("cycle {}/{}", cycle_idx + 1, nb_cycles);
                    for expr in expressions.iter() {
                        match expr {
                            Expression::AwaitKey(button) => {
                                if buttons_in_use.contains(button) {
                                    tracing::error!(
                                        "Cannot use '{:?}' to await as it is already in use",
                                        button
                                    );
                                    break 'outer;
                                }
                            }
                            _ => (),
                        }
                        expr.execute();
                        std::thread::sleep(delay)
                    }
                }
                if let Err(err) = sender.send(()) {
                    tracing::error!("Executor failed to signal the main thread: '{err}'")
                };
            })?;

        Ok(receiver)
    }
}
