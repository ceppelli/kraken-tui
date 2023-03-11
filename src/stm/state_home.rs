use super::{events::Event, State, States};
use crate::app::AppContext;
use crate::ui::core::draw_box;
use crossterm::event::KeyCode;
use tui::{backend::Backend, Frame};

// Home State
pub struct HomeState {
  pub on_enter_first: bool,
}

impl State for HomeState {
  fn on_enter_once(&mut self, ctx: &mut AppContext) {
    self.on_enter_first = false;
    let result = ctx.kraken_api.connect();
    match result {
      Ok(_) => {
        ctx.debug("[HomeS] on_enter_once kraken_api connected.".to_string());

        let result = ctx.kraken_api.list_assets();
        match result {
          Some(assets) => {
            ctx.debug("[HomeS] on_enter_once kraken_api list_assets.".to_string());
            ctx.model.assets = assets;
          },
          None => {
            ctx.debug("[HomeS] on_enter_once kraken_api list_assets failed.".to_string());
          },
        }

        /*let result = ctx.kraken_api.list_asset_pairs();
        match result {
          Some(asset_pairs) => {
            ctx.debug("[HomeS] on_enter_once kraken_api list_asset_pairs.".to_string());
            ctx.model.asset_pairs = asset_pairs;
          },
          None => {
            ctx.debug("[HomeS] on_enter_once kraken_api list_asset_pairs failed.".to_string());
          },
        }*/
      },
      Err(_) => ctx.debug("[HomeS] on_enter_once kraken_api error connection!!!".to_string()),
    }
  }

  fn on_event(&mut self, event: Event, ctx: &mut AppContext) -> Option<States> {
    #[allow(clippy::let_and_return)]
    let to_state = match event {
      Event::Key { key_code: KeyCode::Char('f') } => Some(States::Search),
      _ => {
        ctx.debug(format!("[HomeS] on_event {:?} not match", event));
        None
      },
    };

    to_state
  }

  fn ui<B: Backend>(&self, f: &mut Frame<B>, _ctx: &mut AppContext) {
    let size = f.size();
    draw_box(f, size, " Home State ");
  }

  fn help_text(&self) -> &str {
    r##"
      ESC   -> back
      f     -> search
      D     -> show Debug
    "##
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{kraken::client::MockClient, stm::events::Event};
  use crossterm::event::KeyCode;

  #[test]
  fn test_home_state() -> Result<(), String> {
    let mut ctx = AppContext::new_for_testing(Box::new(MockClient::new()));

    let mut home = HomeState { on_enter_first: true };

    let event = Event::Key { key_code: KeyCode::Char('f') };
    let to_state = home.on_event(event, &mut ctx);
    assert_eq!(to_state, Some(States::Search));

    let event = Event::Key { key_code: KeyCode::Char('*') };
    let to_state = home.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    Ok(())
  }
}
