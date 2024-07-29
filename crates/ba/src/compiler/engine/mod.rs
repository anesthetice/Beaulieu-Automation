use super::expression::Expression;

mod watcher;
use watcher::Watcher;

pub struct Engine {
    inner: Vec<Expression>,
    watcher: Watcher,
    delay: std::time::Duration,
}

impl Engine {
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

        let width_ratio: f64 = host_resolution.0 as f64 / script_resolution.0 as f64;
        let height_ratio: f64 = host_resolution.1 as f64 / script_resolution.1 as f64;
        let modify_positions = (width_ratio != 1.0) | (height_ratio != 1.0);

        // extract and launch binds
        let mut buttons_in_use = vec![global_halt_key.clone()];
        while let Some(idx) = expressions
            .iter()
            .position(|expr| matches!(expr, Expression::Bind(..)))
        {
            let (button, sub_expressions) = match expressions.remove(idx) {
                Expression::Bind(key, sub_expressions) => (key, sub_expressions),
                _ => unreachable!(),
            };
            tracing::info!("Attempting to bind '{:?}' as a HotKey", button);
            tracing::trace!("HotKey expressions = {:?}", sub_expressions);
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
                }
            })?;
        }

        let expressions: Vec<Expression> = expressions
            .into_iter()
            .filter(|expr| !expr.is_handled_at_init())
            .map(|expr| match expr {
                expr @ Expression::Move((x, y)) => {
                    if modify_positions {
                        Expression::Move((
                            (x as f64 * width_ratio).floor() as i32,
                            (y as f64 * height_ratio).floor() as i32,
                        ))
                    } else {
                        expr
                    }
                }
                other => other,
            })
            .collect();

        Ok(Self {
            inner: expressions,
            watcher: Watcher::new(global_halt_key)?,
            delay: delay_between_actions,
        })
    }

    pub fn start(self, nb_cycles: usize) -> anyhow::Result<()> {
        for cycle_idx in 0..nb_cycles {
            tracing::info!("cycle {}/{}", cycle_idx + 1, nb_cycles);
            for expr in self.inner.iter() {
                if self.watcher.check() {
                    self.watcher.post_halt();
                    return Err(anyhow::anyhow!("Engine manually stopped"));
                }

                if matches!(expr, Expression::Await) {
                    tracing::info!("Reached 'Await' instruction, awaiting indefinitely...");
                    loop {
                        if self.watcher.check() {
                            self.watcher.post_halt();
                            return Err(anyhow::anyhow!("Engine manually stopped"));
                        }
                        std::thread::sleep(self.delay);
                    }
                }

                expr.execute();
                std::thread::sleep(self.delay);
            }
        }

        tracing::info!("Engine finished running");

        Ok(())
    }
}
