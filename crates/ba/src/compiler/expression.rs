use super::button::Button;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Resolution((i32, i32)),
    DelayBetweenActions(u64),
    GlobalHaltKey(Button),
    Move((i32, i32)),
    Tap(Button),
    Press(Button),
    Release(Button),
    Sleep(f64),
    Type(String),
    Await,
    AwaitKey(Button),
    Bind(Button, Vec<Expression>),
}

impl Expression {
    pub(super) fn execute(&self) {
        match self {
            // Handled during engine creation
            Self::Resolution(_) => (),
            Self::DelayBetweenActions(_) => (),
            Self::GlobalHaltKey(_) => (),
            Self::Bind(..) => (),

            // Handled directly
            Self::Move(pos) => inputbot::MouseCursor::move_abs(pos.0, pos.1),
            Self::Tap(button) => button.tap(),
            Self::Press(button) => button.press(),
            Self::Release(button) => button.release(),
            Self::Sleep(float) => std::thread::sleep(std::time::Duration::from_secs_f64(*float)),
            Self::Type(string) => inputbot::send_sequence(string),
            Self::Await => loop { std::thread::sleep(std::time::Duration::from_secs(5)) },
            Self::AwaitKey(button) => {
                if let Err(err) = button.await_in_place() {
                    tracing::warn!("{}", err);
                }
            }
        }
    }

    pub(super) fn is_handled_at_init(&self) -> bool {
        matches!(
            self,
            Self::Resolution(_)
                | Self::DelayBetweenActions(_)
                | Self::GlobalHaltKey(_)
                | Self::Bind(..)
        )
    }
}

pub fn adapt_expressions(
    input: Vec<Expression>,
    host_resolution: (i32, i32),
    script_resolution: (i32, i32),
) -> Vec<Expression> {
    let width_ratio: f64 = host_resolution.0 as f64 / script_resolution.0 as f64;
    let height_ratio: f64 = host_resolution.1 as f64 / script_resolution.1 as f64;
    let modify_positions = (width_ratio != 1.0) | (height_ratio != 1.0);

    input
        .into_iter()
        .filter(|expr| !expr.is_handled_at_init())
        .map(|expr| match expr {
            expr @ Expression::Move((x, y)) => {
                if modify_positions {
                    let new_x = (x as f64 * width_ratio).floor() as i32;
                    let new_y = (y as f64 * height_ratio).floor() as i32;
                    tracing::trace!("Adapted 'Move' expression, ({x}, {y}) → ({new_x}, {new_y})");
                    Expression::Move((new_x, new_y))
                } else {
                    expr
                }
            }
            other => other,
        })
        .collect()
}
