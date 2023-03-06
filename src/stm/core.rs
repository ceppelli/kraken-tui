use tui::{backend::Backend, Frame};

use crossterm::event::KeyCode;

use crate::app::AppContext;
use crate::stm::events::Event;
use crate::ui::core::{centered_rect, clear_box, draw_box, draw_paragraph};
use crate::ui::list_stateful_widget::draw_stateful_list;

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum States {
  Unknown,
  PreviousOne,
  NextOne,
  Debug,

  Home,
  Search,
  Help,
}

pub trait State {
  // this method returns an Option<States>. If it is not NONE the optional States value
  // is the new state which the State Machine will transition
  fn on_event(&self, event: Event, ctx: &mut AppContext) -> Option<States> {
    ctx.debug(format!("[ConsumeEvent] on_event {:?}", event));
    None
  }

  fn ui<B: Backend>(&self, f: &mut Frame<B>, _ctx: &mut AppContext);

  fn help_text(&self) -> &str {
    r##"
    Help
    "##
  }
}

// Unknown State
struct UnknownState;

impl State for UnknownState {
  fn on_event(&self, event: Event, ctx: &mut AppContext) -> Option<States> {
    let to_state = match event {
      Event::Key {
        key_code: KeyCode::Esc,
      } => Some(States::Home),
      _ => {
        ctx.debug(format!("[UnknownS] on_event {:?} not match", event));
        None
      }
    };

    to_state
  }

  fn ui<B: Backend>(&self, f: &mut Frame<B>, _ctx: &mut AppContext) {
    let size = f.size();
    draw_box(f, size, " Unknow State ");

    let area = centered_rect(80, 40, size);
    clear_box(f, area); //this clears out the background
    draw_box(f, area, " Popup");
  }

  fn help_text(&self) -> &str {
    r##"
    ESC   -> back
    "##
  }
}

// Help State
pub struct HelpState;

impl State for HelpState {
  fn on_event(&self, event: Event, ctx: &mut AppContext) -> Option<States> {
    let to_state = match event {
      Event::Key {
        key_code: KeyCode::Esc,
      } => Some(States::PreviousOne),
      _ => {
        ctx.debug(format!("[HelpS] on_event {:?} not match", event));
        None
      }
    };

    to_state
  }

  fn ui<B: Backend>(&self, _f: &mut Frame<B>, _ctx: &mut AppContext) {}
}

// Debug State
struct DebugState;

impl State for DebugState {
  fn on_event(&self, event: Event, ctx: &mut AppContext) -> Option<States> {
    match event {
      Event::Key {
        key_code: KeyCode::Esc,
      } => Some(States::PreviousOne),
      Event::Key {
        key_code: KeyCode::Down,
      } => {
        ctx.model.debug_messages_stateful.next();
        None
      }
      Event::Key {
        key_code: KeyCode::Up,
      } => {
        ctx.model.debug_messages_stateful.previous();
        None
      }
      Event::Key {
        key_code: KeyCode::Left,
      } => {
        ctx.model.debug_messages_stateful.unselect();
        None
      }
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
      true
    );
  }

}

// Home State
struct HomeState;

impl State for HomeState {
  fn on_event(&self, event: Event, ctx: &mut AppContext) -> Option<States> {
    let to_state = match event {
      Event::Key {
        key_code: KeyCode::Char('f'),
      } => {
        ctx.debug(format!(
          "[HomeS] on_event {:?} key_code: {:?}",
          event, "e"
        ));
        Some(States::Search)
      }
      Event::Key { key_code } => {
        ctx.debug(format!(
          "[HomeS] on_event {:?} key_code: {:?}",
          event, key_code
        ));
        Some(States::Unknown)
      }
      _ => {
        ctx.debug(format!("[HomeS] on_event {:?} not match", event));
        None
      }
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

// Search State
struct SearchState;

impl State for SearchState {
  fn on_event(&self, event: Event, ctx: &mut AppContext) -> Option<States> {
    let to_state = match event {
      Event::Key { key_code: KeyCode::Char('l') } => {
        let result = ctx.kraken_api.connect();
        match result {
          Ok(_) => {
            let result = ctx.kraken_api.list_assets();

            for (key, info) in &result {
              ctx.model.kraken_assets_stateful.push(format!(" {} {} {} {} {}", key, info.altname, info.aclass, info.decimals, info.display_decimals));
            }
            
            /*let assests = result.iter()
              .map(|(key, info)|  format!(" {} {} {} {} {}", key, info.altname, info.aclass, info.decimals, info.display_decimals))
              .collect::<Vec<String>>();

            assests.iter().for_each(|item| {
              ctx.model.kraken_assets_stateful.push(item.to_string());
            });*/

          },
          Err(error_code) => println!("[main] Error:{:?}", error_code)
        }

        None
      },
      Event::Key {
        key_code: KeyCode::Down,
      } => {
        ctx.model.kraken_assets_stateful.next();
        None
      }
      Event::Key {
        key_code: KeyCode::Up,
      } => {
        ctx.model.kraken_assets_stateful.previous();
        None
      }
      Event::Key {
        key_code: KeyCode::Left,
      } => {
        ctx.model.kraken_assets_stateful.unselect();
        None
      }
      Event::Key { key_code: KeyCode::Char('h') } => {
        Some(States::Home)
      },
      _ => {
        ctx.debug(format!("[SearchS] on_event {:?} not match", event));
        None
      }
    };

    to_state
  }

  fn ui<B: Backend>(&self, f: &mut Frame<B>, ctx: &mut AppContext) {
    let size = f.size();
    draw_box(f, size, " Search State ");

    let bbox = centered_rect(95, 90, size);
    draw_stateful_list(
      f,
      bbox,
      "assets",
      &mut ctx.model.kraken_assets_stateful,
      false
    );
  }

  fn help_text(&self) -> &str {
    r##"
      ESC   -> back
      l     -> list assets
      h     -> home
      D     -> show Debug
    "##
  }
}
#[allow(unused)]
pub struct MainStm<'a> {
  name: &'a str,
  pub current_st: States,
  previous_st: Option<States>,
  next_st: Option<States>,

  unknow_st: UnknownState,
  debug_st: DebugState,
  help_st: HelpState,
  home_st: HomeState,
  search_st: SearchState,

  trace: bool,
}

impl MainStm<'_> {
  pub fn new(name: &'_ str, trace: bool) -> MainStm {
    MainStm {
      name,
      current_st: States::Unknown,
      previous_st: None,
      next_st: None,

      unknow_st: UnknownState,
      debug_st: DebugState,
      help_st: HelpState,
      home_st: HomeState,
      search_st: SearchState,

      trace,
    }
  }

  pub fn switch_state(&mut self, to_state: States) {

    match to_state {
      States::PreviousOne => {
        if let Some(prev_state) = self.previous_st {
          self.current_st = prev_state;
          self.previous_st = None;
        }
      },
      _ => {
        self.previous_st = Some(self.current_st);
        self.current_st = to_state;
      }
    }
  }

  pub fn on_event(&mut self, event: Event, ctx: &mut AppContext) {
    if self.current_st != States::Debug {
      ctx.debug(format!("[STM] on_event {:?} state current:{:?} prev:{:?}", event, self.current_st, self.previous_st));
    }

    match (self.current_st, event.clone()) {
      (States::Unknown, Event::Key { key_code: KeyCode::Char('D') })
      | (States::Home, Event::Key { key_code: KeyCode::Char('D') })
      | (States::Search, Event::Key { key_code: KeyCode::Char('D') }) => {
         self.switch_state(States::Debug)
      },
      (States::Home, Event::Key { key_code: KeyCode::Char('?') })
      | (States::Search, Event::Key { key_code: KeyCode::Char('?') }) => {
        self.switch_state(States::Help)
      },
      (States::Unknown, _) => {
        if let Some(to_state) = self.unknow_st.on_event(event, ctx) {
          self.switch_state(to_state);
        }
      },
      (States::Debug, _) => {
        if let Some(to_state) = self.debug_st.on_event(event, ctx) {
          self.switch_state(to_state);
        }
      },
      (States::Home, Event::Key { key_code: KeyCode::Char('f') }) => {
        if let Some(to_state) = self.home_st.on_event(event, ctx) {
          self.switch_state(to_state);
        }
      },
      (States::Search, _) => {
        if let Some(to_state) = self.search_st.on_event(event, ctx) {
          self.switch_state(to_state);
        }
      },
      (States::Help, _) => {
        if let Some(to_state) = self.help_st.on_event(event, ctx) {
          self.switch_state(to_state);
        }
      },
      _ => ctx.debug(format!("[STM] on_event {:?} not match", event)),
    }
  }

  pub fn draw<B: Backend>(&self, f: &mut Frame<B>, ctx: &mut AppContext) {
    // it Help is the current state, overlay to the previous state ui the help view.
    if self.current_st == States::Help {

      let text = match self.previous_st {
        Some(States::Unknown) => {
          self.unknow_st.ui(f, ctx);
          self.unknow_st.help_text()
        },
        Some(States::Debug) => {
          self.debug_st.ui(f, ctx);
          self.debug_st.help_text()
        },
        Some(States::Home) => {
          self.home_st.ui(f, ctx);
          self.home_st.help_text()
        },
        Some(States::Search) => {
          self.search_st.ui(f, ctx);
          self.search_st.help_text()
        },
        _ => {
          ctx.debug(format!(
          "[STM] draw for current:{:?} not match",
          self.current_st
          ));
          "None"
        }
      };

      // overlay the help popup
      let bbox = centered_rect(75, 70, f.size());
      let bbox_inner = centered_rect(95, 90, bbox);
      clear_box(f, bbox);
      draw_box(f, bbox, " Help State ");
      draw_paragraph(f, bbox_inner, text);
    } else {
      match self.current_st {
        States::Unknown => self.unknow_st.ui(f, ctx),
        States::Debug => self.debug_st.ui(f, ctx),
        States::Home => self.home_st.ui(f, ctx),
        States::Search => self.search_st.ui(f, ctx),
        _ => ctx.debug(format!(
          "[STM] draw for current:{:?} not match",
          self.current_st
        )),
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::stm::events::Event;
  use crossterm::event::KeyCode;

  #[test]
  fn test_home_st() -> Result<(), String> {
    let mut ctx = AppContext::new(String::from("APP_ID"), String::from("APP_VERSION"));

    let event = Event::Key {
      key_code: KeyCode::Esc,
    };

    let home = HomeState;
    home.on_event(event, &mut ctx);

    Ok(())
  }

  #[test]
  fn test_stm() -> Result<(), String> {
    let mut ctx = AppContext::new(String::from("APP_ID"), String::from("APP_VERSION"));

    let mut stm = MainStm::new("my_stm", false);
    assert_eq!(stm.name, "my_stm");
    assert!(matches!(stm.current_st, States::Unknown));

    stm.switch_state(States::Search);
    assert!(matches!(stm.current_st, States::Search));

    let e = Event::Key {
      key_code: KeyCode::Esc,
    };
    stm.on_event(e, &mut ctx);

    Ok(())
  }
}
