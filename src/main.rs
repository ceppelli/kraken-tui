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
  core::{States, MainStm},
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
  let res = run_app(&mut xterm.terminal, &mut ctx, &mut stm);

  // check for errors
  if let Err(err) = res {
    println!("[main] {:?}", err)
  }

  // restore terminal state
  xterm.restore()?;

  print!("[main] app info {} completed!!!", ctx.info());

  Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, ctx: &mut AppContext, stm: &mut MainStm) -> io::Result<()> {
  // reset the state machine
  stm.switch_state(States::Home);

  loop {
    terminal.draw(|f| stm.draw(f, ctx))?;

    if let Event::Key(key) = event::read()? {
      stm.on_event(events::Event::Key { key_code: key.code }, ctx);

      if let KeyCode::Char('q') = key.code {
        return Ok(())
      }
    }
  }
}
