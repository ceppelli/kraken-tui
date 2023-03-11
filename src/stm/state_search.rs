use super::{events::Event, State, States};
use crate::app::AppContext;
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
    Self {
      on_enter_first: true,
      avtive_column: ActiveColumn::Assets,
    }
  }
}

impl SearchState {
  fn filter_asset_pairs(&self, ctx: &mut AppContext) {
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
          .filter_map(|(_, pair)| pair.wsname.as_ref().map(|wsname| wsname.to_owned()))
          .collect::<Vec<String>>();

        ctx.debug(format!(" len: {}", pairs.len()));

        for alt_name in pairs {
          ctx.model.asset_pairs_stateful.push(alt_name);
        }
      }
    }
  }
}

impl State for SearchState {
  fn on_enter_once(&mut self, ctx: &mut AppContext) {
    self.on_enter_first = false;
    ctx.debug("[SearchS] on_enter_once".to_string());

    for key in ctx.model.assets.keys() {
      ctx.model.assets_stateful.push(key.to_owned());
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

  fn on_event(&mut self, event: Event, ctx: &mut AppContext) -> Option<States> {
    #[allow(clippy::let_and_return)]
    let to_state = match event {
      Event::Key { key_code: KeyCode::Down } => {
        match self.avtive_column {
          ActiveColumn::Assets => {
            ctx.model.assets_stateful.next();
            self.filter_asset_pairs(ctx);
          },
          ActiveColumn::AssetPairs => {
            ctx.model.asset_pairs_stateful.next();
          }
        }
        None
      },
      Event::Key { key_code: KeyCode::Up } => {
        match self.avtive_column {
          ActiveColumn::Assets => {
            ctx.model.assets_stateful.previous();
            self.filter_asset_pairs(ctx);
          },
          ActiveColumn::AssetPairs => {
            ctx.model.asset_pairs_stateful.previous();
          }
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
          }
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
            ActiveColumn::AssetPairs => {
            }
        }
        None
      },
      Event::Key { key_code: KeyCode::Char('h') } => Some(States::Home),
      _ => {
        ctx.debug(format!("[SearchS] on_event {:?} not match", event));
        None
      },
    };

    to_state
  }

  fn ui<B: Backend>(&self, f: &mut Frame<B>, ctx: &mut AppContext) {
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
      h      -> home
      D      -> show Debug
    "##
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{kraken::client::MockClient, stm::events::Event};
  use crossterm::event::KeyCode;
  use krakenrs::AssetPairsResponse;

  #[test]
  fn test_search_state() -> Result<(), String> {
    let mut ctx = AppContext::new_for_testing(Box::new(MockClient::new()));

    let mut search = SearchState::default();

    let event = Event::Key { key_code: KeyCode::Char('h') };
    let to_state = search.on_event(event, &mut ctx);
    assert_eq!(to_state, Some(States::Home));

    let event = Event::Key { key_code: KeyCode::Char('*') };
    let to_state = search.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    let event = Event::Key { key_code: KeyCode::Down };
    let to_state = search.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    let event = Event::Key { key_code: KeyCode::Up };
    let to_state = search.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    let event = Event::Key { key_code: KeyCode::Left };
    let to_state = search.on_event(event, &mut ctx);
    assert_eq!(to_state, None);

    Ok(())
  }

  #[test]
  fn test_search_state_list_assets() -> Result<(), String> {
    let mut mock_client = Box::new(MockClient::new());
    mock_client
      .expect_list_asset_pairs()
      .once()
      .returning(|| Some(AssetPairsResponse::new()));
    let mut ctx = AppContext::new_for_testing(mock_client);

    let event = Event::Key { key_code: KeyCode::Char('l') };

    let mut search = SearchState::default();
    search.on_enter_once(&mut ctx);

    assert_eq!(search.on_enter_first, false);

    let to_state = search.on_event(event, &mut ctx);

    assert_eq!(to_state, None);
    assert_eq!(ctx.model.asset_pairs.len(), 0);
    assert_eq!(ctx.model.assets_stateful.items.len(), 0);
    assert_eq!(ctx.model.assets_stateful.state.selected(), None);

    Ok(())
  }
}
