use super::expression::Expression;

mod watcher;
use watcher::Watcher;

pub struct Engine {
    inner: Vec<Expression>,
    watcher: Watcher,
    delay: std::time::Duration,
}

impl Engine {
    pub fn new(expressions: Vec<Expression>, host_resolution: (i32, i32)) -> anyhow::Result<Self> {
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

        let global_halt_button = expressions
            .iter()
            .find_map(|expr| match expr {
                Expression::GlobalHaltButton(button) => Some(*button),
                _ => None,
            })
            .ok_or_else(|| {
                tracing::error!("GLOBAL_HALT_BUTTON definition missing");
                anyhow::anyhow!("Failed to create engine")
            })?;
        tracing::debug!("global halt button = {:?}", global_halt_button);

        let width_ratio: f64 = host_resolution.0 as f64 / script_resolution.0 as f64;
        let height_ratio: f64 = host_resolution.1 as f64 / script_resolution.1 as f64;
        let modify_positions = (width_ratio != 1.0) | (height_ratio != 1.0);

        let expressions: Vec<Expression> = expressions
            .into_iter()
            .filter(|expr| !expr.is_definition())
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
            watcher: Watcher::new(global_halt_button),
            delay: delay_between_actions,
        })
    }

    pub fn start(self, nb_cycles: usize) -> anyhow::Result<()> {
        for cycle_idx in 0..nb_cycles {
            tracing::info!("cycle {}/{}", cycle_idx + 1, nb_cycles);
            for expr in self.inner.iter() {
                if self.watcher.check() {
                    self.watcher.clean();
                    return Err(anyhow::anyhow!("Engine manually stopped"));
                }
                expr.execute();
                std::thread::sleep(self.delay);
            }
        }

        tracing::info!("Engine finished running");

        Ok(())
    }
}
