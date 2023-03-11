use super::{events::Event, State, States};
use crate::ui::core::{centered_rect, draw_box};
use crate::{app::AppContext, ui::list_stateful_widget::draw_stateful_list};
use crossterm::event::KeyCode;
use tui::{backend::Backend, Frame};

// Debug State
pub struct DebugState;

impl State for DebugState {
  fn on_event(&mut self, event: Event, ctx: &mut AppContext) -> Option<States> {
    match event {
      Event::Key { key_code: KeyCode::Esc } => Some(States::PreviousOne),
      Event::Key { key_code: KeyCode::Down } => {
        ctx.model.debug_messages_stateful.next();
        None
      },
      Event::Key { key_code: KeyCode::Up } => {
        ctx.model.debug_messages_stateful.previous();
        None
      },
      Event::Key { key_code: KeyCode::Left } => {
        ctx.model.debug_messages_stateful.unselect();
        None
      },
      _ => None,
    }
  }

  fn ui<B: Backend>(&self, f: &mut Frame<B>, ctx: &mut AppContext) {
    let size = f.size();
    draw_box(f, size, " Debug State ");

    let bbox = centered_rect(95, 90, size);
    draw_stateful_list(
      f,
      bbox,
      "debug messages",
      &mut ctx.model.debug_messages_stateful,
      true,
    );
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{kraken::client::MockClient, stm::events::Event};
  use crossterm::event::KeyCode;

  #[test]
  fn test_debug_state() -> Result<(), String> {
    let mut ctx = AppContext::new_for_testing(Box::new(MockClient::new()));

    let mut debug = DebugState;

    let event = Event::Key { key_code: KeyCode::Esc };
    let to_state = debug.on_event(event, &mut ctx);
    assert_eq!(to_state, Some(States::PreviousOne));

    let event = Event::Key { key_code: KeyCode::Down };
    let to_state = debug.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    let event = Event::Key { key_code: KeyCode::Up };
    let to_state = debug.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    let event = Event::Key { key_code: KeyCode::Left };
    let to_state = debug.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    Ok(())
  }
}
