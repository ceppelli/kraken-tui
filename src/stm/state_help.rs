use crossterm::event::KeyCode;
use tui::{backend::Backend, Frame};

use crate::app::Context;
use crate::stm::{events::Event, State, States};

// Help State
pub struct HelpState;

impl State for HelpState {
    fn on_event(&mut self, event: Event, ctx: &mut Context) -> Option<States> {
        if let Event::Key {
            key_code: KeyCode::Esc,
        } = event
        {
            Some(States::PreviousOne)
        } else {
            ctx.debug(format!("[HelpS] on_event {event:?} not match"));
            None
        }
    }

    fn ui<B: Backend>(&self, _f: &mut Frame<B>, _ctx: &mut Context) {}
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;

    use crate::{kraken::client::MockRestAPI, stm::events::Event};

    use super::*;

    #[test]
    fn test_home_state() -> Result<(), String> {
        let mut ctx = Context::new_for_testing(Box::new(MockRestAPI::new()));

        let mut help = HelpState;

        let event = Event::Key {
            key_code: KeyCode::Char('*'),
        };
        let to_state = help.on_event(event, &mut ctx);
        assert_eq!(to_state, None);

        Ok(())
    }

    #[test]
    fn test_state_help() -> Result<(), String> {
        let state = HelpState;
        assert_eq!(state.help_text().len(), 22);

        Ok(())
    }
}
