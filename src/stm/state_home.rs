use crossterm::event::KeyCode;
use tui::{backend::Backend, Frame};

use crate::stm::{events::Event, State, States};
use crate::ui::core::draw_box;
use crate::ui::list_stateful_widget::draw_stateful_list;
use crate::{app::Context, ui::core::split_columns};

// Home State
pub struct HomeState {
    pub on_enter_first: bool,
}

impl Default for HomeState {
    fn default() -> Self {
        Self {
            on_enter_first: true,
        }
    }
}

impl HomeState {
    fn show_asset_pair_info(ctx: &mut Context) {
        ctx.model.favorites_asset_pairs_info_stateful.clear();

        let selected = ctx.model.favorites_asset_pairs_stateful.state.selected();
        if let Some(index) = selected {
            let asset_pair_opt = ctx.model.favorites_asset_pairs_stateful.items.get(index);

            if let Some(asset_pair) = asset_pair_opt {
                let pairs = ctx
                    .model
                    .asset_pairs
                    .iter()
                    .filter(|(_, pair)| pair.wsname == Some(asset_pair.clone()))
                    .map(|(_, pair)| pair)
                    .collect::<Vec<_>>();

                if !pairs.is_empty() {
                    let pair = pairs[0];
                    let stateful_list = &mut ctx.model.favorites_asset_pairs_info_stateful;

                    stateful_list.push(format!("      alt_name:{:?}", pair.alt_name));
                    stateful_list.push(format!("        wsname:{:?}", pair.wsname));
                    stateful_list.push(format!("   aclass_base:{}", pair.aclass_base));
                    stateful_list.push(format!("          base:{}", pair.base));
                    stateful_list.push(format!("  aclass_quote:{}", pair.aclass_quote));
                    stateful_list.push(format!("         quote:{}", pair.quote));
                    stateful_list.push(format!(" pair_decimals:{}", pair.pair_decimals));
                    stateful_list.push(format!("  lot_decimals:{}", pair.lot_decimals));
                    stateful_list.push(format!("lot_multiplier:{}", pair.lot_multiplier));
                    stateful_list.push("          fees:".to_owned());
                    for fee in &pair.fees {
                        stateful_list.push(format!("             - {fee:?}"));
                    }
                    stateful_list.push(format!("      ordermin:{:?}", pair.ordermin));
                }
            }
        }
    }
}

impl State for HomeState {
    fn on_enter_once(&mut self, ctx: &mut Context) {
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
                    }
                    None => {
                        ctx.debug(
                            "[HomeS] on_enter_once kraken_api list_assets failed.".to_string(),
                        );
                    }
                }
            }
            Err(_) => ctx.debug("[HomeS] on_enter_once kraken_api error connection!!!".to_string()),
        }
    }

    fn on_event(&mut self, event: Event, ctx: &mut Context) -> Option<States> {
        match event {
            Event::Key {
                key_code: KeyCode::Char('f'),
            } => Some(States::Search),
            Event::Key {
                key_code: KeyCode::Down,
            } => {
                ctx.model.favorites_asset_pairs_stateful.next();
                HomeState::show_asset_pair_info(ctx);

                None
            }
            Event::Key {
                key_code: KeyCode::Up,
            } => {
                ctx.model.favorites_asset_pairs_stateful.previous();
                HomeState::show_asset_pair_info(ctx);

                None
            }
            Event::Key {
                key_code: KeyCode::Left,
            } => {
                ctx.model.favorites_asset_pairs_stateful.unselect();
                ctx.model.favorites_asset_pairs_info_stateful.clear();

                None
            }
            Event::Key {
                key_code: KeyCode::Char('d'),
            } => {
                let selected = ctx.model.favorites_asset_pairs_stateful.state.selected();
                if let Some(index) = selected {
                    ctx.model.favorites_asset_pairs_stateful.unselect();
                    if index < ctx.model.favorites_asset_pairs_stateful.items.len() {
                        ctx.model.favorites_asset_pairs_stateful.items.remove(index);
                    }
                    ctx.model.favorites_asset_pairs_info_stateful.clear();
                }

                None
            }
            _ => {
                ctx.debug(format!("[HomeS] on_event {event:?} not match"));
                None
            }
        }
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>, ctx: &mut Context) {
        let size = f.size();
        draw_box(f, size, " Home State ");

        let bboxs = split_columns(2, size);

        draw_stateful_list(
            f,
            bboxs[0],
            " favorites ",
            &mut ctx.model.favorites_asset_pairs_stateful,
            false,
        );
        draw_stateful_list(
            f,
            bboxs[1],
            " details ",
            &mut ctx.model.favorites_asset_pairs_info_stateful,
            false,
        );
    }

    fn help_text(&self) -> &str {
        r##"
        ESC    -> back
        UP     -> previous
        DOWN   -> next
        LEFT   -> previous
        d      -> remove from favorites
        f      -> search
        D      -> show Debug
        "##
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;
    use krakenrs::AssetsResponse;
    use tui::{backend::TestBackend, buffer::Buffer, Terminal};

    use crate::{kraken::client::MockRestAPI, stm::events::Event};

    use super::*;

    #[test]
    fn test_on_event() -> Result<(), String> {
        let mut ctx = Context::new_for_testing(Box::new(MockRestAPI::new()));

        let mut state = HomeState::default();

        let event = Event::Key {
            key_code: KeyCode::Char('f'),
        };
        let to_state = state.on_event(event, &mut ctx);
        assert_eq!(to_state, Some(States::Search));

        let event = Event::Key {
            key_code: KeyCode::Char('*'),
        };
        let to_state = state.on_event(event, &mut ctx);
        assert_eq!(to_state, None);

        let event = Event::Key {
            key_code: KeyCode::Down,
        };
        let to_state = state.on_event(event, &mut ctx);
        assert_eq!(to_state, None);

        let event = Event::Key {
            key_code: KeyCode::Up,
        };
        let to_state = state.on_event(event, &mut ctx);
        assert_eq!(to_state, None);

        let event = Event::Key {
            key_code: KeyCode::Left,
        };
        let to_state = state.on_event(event, &mut ctx);
        assert_eq!(to_state, None);

        let event = Event::Key {
            key_code: KeyCode::Char('d'),
        };
        let to_state = state.on_event(event, &mut ctx);
        assert_eq!(to_state, None);

        Ok(())
    }

    #[test]
    fn test_list_assets() -> Result<(), String> {
        let mut mock_client = Box::new(MockRestAPI::new());
        mock_client.expect_connect().once().returning(|| Ok(()));
        mock_client
            .expect_list_assets()
            .once()
            .returning(|| Some(AssetsResponse::new()));
        let mut ctx = Context::new_for_testing(mock_client);

        let mut state = HomeState::default();
        state.on_enter_once(&mut ctx);

        assert_eq!(state.on_enter_first, false);

        assert_eq!(ctx.model.assets.len(), 0);
        assert_eq!(ctx.model.assets_stateful.items.len(), 0);
        assert_eq!(ctx.model.assets_stateful.state.selected(), None);

        Ok(())
    }

    #[test]
    fn test_asset_pair_select() -> Result<(), String> {
        let mut ctx = Context::new_for_testing(Box::new(MockRestAPI::new()));

        ctx.model
            .favorites_asset_pairs_stateful
            .push("ETH/USDC".to_owned());

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

        let mut state = HomeState::default();

        let event = Event::Key {
            key_code: KeyCode::Down,
        };
        let to_state = state.on_event(event, &mut ctx);
        assert_eq!(to_state, None);

        assert_eq!(ctx.model.favorites_asset_pairs_stateful.items.len(), 1);
        assert_eq!(
            ctx.model.favorites_asset_pairs_stateful.state.selected(),
            Some(0)
        );

        let event = Event::Key {
            key_code: KeyCode::Enter,
        };

        let to_state = state.on_event(event, &mut ctx);
        assert_eq!(to_state, None);

        assert_eq!(
            ctx.model.favorites_asset_pairs_info_stateful.items.len(),
            13
        );
        assert_eq!(
            ctx.model
                .favorites_asset_pairs_info_stateful
                .state
                .selected(),
            None
        );

        Ok(())
    }

    #[test]
    fn test_ui() {
        let backend = TestBackend::new(7, 4);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut ctx = Context::new_for_testing(Box::new(MockRestAPI::new()));

        let state = HomeState::default();

        terminal
            .draw(|f| {
                state.ui(f, &mut ctx);
            })
            .unwrap();

        #[rustfmt::skip]
        let expected = Buffer::with_lines(vec![
            " Home─╮",
            "│ ┌┌┐ │",
            "│ └└┘ │",
            "╰─────╯"
            ]);

        terminal.backend().assert_buffer(&expected);
    }

    #[test]
    fn test_state_help() -> Result<(), String> {
        let state = HomeState::default();
        assert_eq!(state.help_text().len(), 203);

        Ok(())
    }
}
