use super::{events::Event, State, States};
use crate::app::AppContext;
use crossterm::event::KeyCode;
use tui::{backend::Backend, Frame};

// Help State
pub struct HelpState;

impl State for HelpState {
  #[allow(clippy::let_and_return)]
  fn on_event(&mut self, event: Event, ctx: &mut AppContext) -> Option<States> {
    let to_state = match event {
      Event::Key { key_code: KeyCode::Esc } => Some(States::PreviousOne),
      _ => {
        ctx.debug(format!("[HelpS] on_event {:?} not match", event));
        None
      },
    };

    to_state
  }

  fn ui<B: Backend>(&self, _f: &mut Frame<B>, _ctx: &mut AppContext) {}
}
