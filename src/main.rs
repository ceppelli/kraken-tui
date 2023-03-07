mod app;
mod kraken;
mod stm;
mod terminal;
mod ui;

use std::io;

use crossterm::event::{self, Event, KeyCode};

use tui::{backend::Backend, Terminal};

use app::AppContext;
use stm::{
  core::{MainStm, States},
  events,
};

const APP_ID: &str = "kraken";
const APP_VERSION: &str = "0.0.1+";

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // initialize terminal state
  let mut xterm = terminal::XTerminal::new()?;

  // initialize app context and state machine
  let mut ctx = AppContext::new(String::from(APP_ID), String::from(APP_VERSION));
  let mut stm = MainStm::new("stm", true);
  let res = run_app(&mut xterm.terminal, &mut ctx, &mut stm, true);

  // check for errors
  if let Err(err) = res {
    println!("[main] {:?}", err)
  }

  // restore terminal state
  xterm.restore()?;

  print!("[main] app info {} completed!!!", ctx.info());

  Ok(())
}

fn run_app<B: Backend>(
  terminal: &mut Terminal<B>,
  ctx: &mut AppContext,
  stm: &mut MainStm,
  looping: bool,
) -> io::Result<()> {
  // reset the state machine
  stm.switch_state(States::Home);

  loop {
    terminal.draw(|f| stm.draw(f, ctx))?;

    if !looping {
      return Ok(());
    }

    if let Event::Key(key) = event::read()? {
      stm.on_event(events::Event::Key { key_code: key.code }, ctx);

      if let KeyCode::Char('q') = key.code {
        return Ok(());
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use tui::{backend::TestBackend, Terminal};

  use super::*;

  #[test]
  fn test_app_id() {
    assert_eq!(APP_ID, "kraken");
  }

  #[test]
  fn test_app_version() {
    assert_eq!(APP_VERSION, "0.0.1+");
  }

  #[test]
  fn test_run_app() {
    let backend = TestBackend::new(7, 4);
    let terminal = &mut Terminal::new(backend).unwrap();

    let ctx = &mut AppContext::new(String::from(APP_ID), String::from(APP_VERSION));

    let stm = &mut MainStm::new("stm", false);

    let result = run_app(terminal, ctx, stm, false);

    assert!(result.is_ok());
    assert_eq!(stm.current_st, States::Home);
  }
}
