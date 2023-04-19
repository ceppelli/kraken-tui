use super::{events::Event, State, States};
use crate::app::Context;
use crate::ui::{
  core::{draw_box, split_columns},
  list_stateful_widget::draw_stateful_list,
};
use crossterm::event::KeyCode;
use tui::{backend::Backend, Frame};

enum ActiveColumn {
  Assets,
  AssetPairs,
}

// Search State
pub struct SearchState {
  pub on_enter_first: bool,
  avtive_column: ActiveColumn,
}

impl Default for SearchState {
  fn default() -> Self {
    Self { on_enter_first: true, avtive_column: ActiveColumn::Assets }
  }
}

impl SearchState {
  fn filter_asset_pairs(ctx: &mut Context) {
    ctx.model.asset_pairs_stateful.clear();

    let selected = ctx.model.assets_stateful.state.selected();
    if let Some(index) = selected {
      let asset_opt = ctx.model.assets_stateful.items.get(index);

      if let Some(asset) = asset_opt {
        let pairs = ctx
          .model
          .asset_pairs
          .iter()
          .filter(|(_, pair)| pair.base == *asset)
          .filter_map(|(_, pair)| pair.wsname.as_ref().map(std::borrow::ToOwned::to_owned))
          .collect::<Vec<String>>();

        for alt_name in pairs {
          ctx.model.asset_pairs_stateful.push(alt_name);
        }
      }
    }
  }
}

impl State for SearchState {
  fn on_enter_once(&mut self, ctx: &mut Context) {
    self.on_enter_first = false;
    ctx.debug("[SearchS] on_enter_once".to_string());

    for key in ctx.model.assets.keys() {
      ctx.model.assets_stateful.push(key.clone());
    }

    let result = ctx.kraken_api.list_asset_pairs();
    match result {
      Some(asset_pairs) => {
        ctx.debug("[SearchS] on_enter_once kraken_api list_asset_pairs.".to_string());
        ctx.model.asset_pairs = asset_pairs;
      },
      None => {
        ctx.debug("[SearchS] on_enter_once kraken_api list_asset_pairs failed.".to_string());
      },
    }
  }

  fn on_event(&mut self, event: Event, ctx: &mut Context) -> Option<States> {
    match event {
      Event::Key { key_code: KeyCode::Down } => {
        match self.avtive_column {
          ActiveColumn::Assets => {
            ctx.model.assets_stateful.next();
            SearchState::filter_asset_pairs(ctx);
          },
          ActiveColumn::AssetPairs => {
            ctx.model.asset_pairs_stateful.next();
          },
        }
        None
      },
      Event::Key { key_code: KeyCode::Up } => {
        match self.avtive_column {
          ActiveColumn::Assets => {
            ctx.model.assets_stateful.previous();
            SearchState::filter_asset_pairs(ctx);
          },
          ActiveColumn::AssetPairs => {
            ctx.model.asset_pairs_stateful.previous();
          },
        }
        None
      },
      Event::Key { key_code: KeyCode::Left } => {
        match self.avtive_column {
          ActiveColumn::Assets => {
            ctx.model.assets_stateful.unselect();
            ctx.model.asset_pairs_stateful.clear();
          },
          ActiveColumn::AssetPairs => {
            self.avtive_column = ActiveColumn::Assets;
            ctx.model.asset_pairs_stateful.unselect();
          },
        }
        None
      },
      Event::Key { key_code: KeyCode::Right } => {
        match self.avtive_column {
          ActiveColumn::Assets => {
            if !ctx.model.asset_pairs_stateful.items.is_empty() {
              self.avtive_column = ActiveColumn::AssetPairs;
              ctx.model.asset_pairs_stateful.next();
            }
          },
          ActiveColumn::AssetPairs => {},
        }
        None
      },
      Event::Key { key_code: KeyCode::Enter } => {
        match self.avtive_column {
          ActiveColumn::Assets => {},
          ActiveColumn::AssetPairs => {
            if let Some(index) = ctx.model.asset_pairs_stateful.state.selected() {
              let asset_pair_opt = ctx.model.asset_pairs_stateful.items.get(index);

              if let Some(asset_pair) = asset_pair_opt {
                let pairs = ctx
                  .model
                  .asset_pairs
                  .iter()
                  .filter(|(_, pair)| pair.wsname == Some(asset_pair.clone()))
                  .filter_map(|(_, pair)| pair.wsname.as_ref().map(std::borrow::ToOwned::to_owned))
                  .collect::<Vec<_>>();

                if !pairs.is_empty() {
                  let key = &pairs[0];
                  ctx
                    .model
                    .favorites_asset_pairs_stateful
                    .push(key.clone());
                }
              }
            }
          },
        }
        None
      },
      Event::Key { key_code: KeyCode::Char('h') } => Some(States::Home),
      _ => {
        ctx.debug(format!("[SearchS] on_event {event:?} not match"));
        None
      },
    }
  }

  fn ui<B: Backend>(&self, f: &mut Frame<B>, ctx: &mut Context) {
    let size = f.size();
    draw_box(f, size, " Search State ");

    let bboxs = split_columns(2, size);

    draw_stateful_list(
      f,
      bboxs[0],
      " assets ",
      &mut ctx.model.assets_stateful,
      false,
    );
    draw_stateful_list(
      f,
      bboxs[1],
      " asset pairs ",
      &mut ctx.model.asset_pairs_stateful,
      false,
    );
  }

  fn help_text(&self) -> &str {
    r##"
      ESC    -> back
      UP     -> previous
      DOWN   -> next
      LEFT   -> previous
      RIGHT  -> select
      ENTER  -> add to favorites
      h      -> home
      D      -> show Debug
    "##
  }
}

#[cfg(test)]
mod tests {
  use crossterm::event::KeyCode;
  use krakenrs::AssetPairsResponse;
  use tui::{backend::TestBackend, buffer::Buffer, Terminal};

  use crate::{kraken::client::MockRestAPI, stm::events::Event};

  use super::*;

  #[test]
  fn test_on_event() -> Result<(), String> {
    let mut ctx = Context::new_for_testing(Box::new(MockRestAPI::new()));

    let mut state = SearchState::default();

    let event = Event::Key { key_code: KeyCode::Char('h') };
    let to_state = state.on_event(event, &mut ctx);
    assert_eq!(to_state, Some(States::Home));

    let event = Event::Key { key_code: KeyCode::Char('*') };
    let to_state = state.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    let event = Event::Key { key_code: KeyCode::Down };
    let to_state = state.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    let event = Event::Key { key_code: KeyCode::Up };
    let to_state = state.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    let event = Event::Key { key_code: KeyCode::Left };
    let to_state = state.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    let event = Event::Key { key_code: KeyCode::Right };
    let to_state = state.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    let event = Event::Key { key_code: KeyCode::Enter };
    let to_state = state.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    Ok(())
  }

  #[test]
  fn test_list_asset_pairs() -> Result<(), String> {
    let mut mock_client = Box::new(MockRestAPI::new());
    mock_client
      .expect_list_asset_pairs()
      .once()
      .returning(|| Some(AssetPairsResponse::new()));
    let mut ctx = Context::new_for_testing(mock_client);

    ctx.model.assets_stateful.push("key_0".to_owned());
    ctx.model.assets_stateful.push("key_1".to_owned());
    ctx.model.assets_stateful.push("key_2".to_owned());

    let event = Event::Key { key_code: KeyCode::Down };

    let mut state = SearchState::default();
    state.on_enter_once(&mut ctx);

    assert_eq!(state.on_enter_first, false);

    let to_state = state.on_event(event, &mut ctx);

    assert_eq!(to_state, None);
    assert_eq!(ctx.model.asset_pairs.len(), 0);
    assert_eq!(ctx.model.assets_stateful.items.len(), 3);
    assert_eq!(ctx.model.assets_stateful.state.selected(), Some(0));

    Ok(())
  }

  #[test]
  fn test_asset_pair_select() -> Result<(), String> {
    let mut ctx = Context::new_for_testing(Box::new(MockRestAPI::new()));

    ctx.model.asset_pairs_stateful.push("ETH/USDC".to_owned());
    ctx.model.asset_pairs_stateful.push("pair_1".to_owned());
    ctx.model.asset_pairs_stateful.push("pair_2".to_owned());

    let asset_pairs_json = r#"{
      "ETHUSDC": {
        "alt_name": null,
        "wsname": "ETH/USDC",
        "aclass_base": "currency",
        "base": "XETH",
        "aclass_quote": "currency",
        "quote": "USDC",
        "pair_decimals": 2,
        "lot_decimals": 8,
        "lot_multiplier": 1,
        "fees": [
          [
            "0",
            "0.26"
          ],
          [
            "50000",
            "0.24"
          ]
        ],
        "ordermin": "0.01"
      }
    }"#;

    ctx.model.asset_pairs = serde_json::from_str(asset_pairs_json).unwrap();
    assert_eq!(ctx.model.asset_pairs.len(), 1);

    let mut state = SearchState::default();
    state.avtive_column = ActiveColumn::AssetPairs;

    let event = Event::Key { key_code: KeyCode::Down };
    let to_state = state.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    assert_eq!(ctx.model.asset_pairs.len(), 1);
    assert_eq!(ctx.model.asset_pairs_stateful.items.len(), 3);
    assert_eq!(ctx.model.asset_pairs_stateful.state.selected(), Some(0));

    let event = Event::Key { key_code: KeyCode::Enter };

    let to_state = state.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    assert_eq!(ctx.model.favorites_asset_pairs_stateful.items.len(), 1);
    assert_eq!(
      ctx.model.favorites_asset_pairs_stateful.state.selected(),
      None
    );

    Ok(())
  }

  #[test]
  fn test_ui() {
    let backend = TestBackend::new(7, 4);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut ctx = Context::new_for_testing(Box::new(MockRestAPI::new()));

    let state = SearchState::default();

    terminal
      .draw(|f| {
        state.ui(f, &mut ctx);
      })
      .unwrap();

    #[rustfmt::skip]
    let expected = Buffer::with_lines(vec![
      " Sear─╮",
      "│ ┌┌┐ │",
      "│ └└┘ │",
      "╰─────╯"
      ]);

    terminal.backend().assert_buffer(&expected);
  }

  #[test]
  fn test_state_help() -> Result<(), String> {
    let state = SearchState::default();
    assert_eq!(state.help_text().len(), 201);

    Ok(())
  }
}
