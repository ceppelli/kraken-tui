mod app;
mod kraken;
mod stm;
mod terminal;
mod ui;
use std::io;

use app::AppContext;
use crossterm::event::{self, Event, KeyCode};
use stm::{events, stm_main::MainStm, States};
use tui::{backend::Backend, Terminal};

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
  stm.switch_state(States::Home, ctx);

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
  use super::*;
  use crate::kraken::client::MockClient;
  use krakenrs::AssetsResponse;
  use tui::{backend::TestBackend, Terminal};

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
    let mut terminal = Terminal::new(backend).unwrap();

    let mut mock_client = Box::new(MockClient::new());
    mock_client.expect_connect().once().returning(|| Ok(()));
    mock_client
      .expect_list_assets()
      .once()
      .returning(|| Some(AssetsResponse::new()));
    let mut ctx = AppContext::new_for_testing(mock_client);
    let mut stm = MainStm::new("stm", false);

    let result = run_app(&mut terminal, &mut ctx, &mut stm, false);

    assert!(result.is_ok());
    assert_eq!(stm.current_st, States::Home);
  }
}
