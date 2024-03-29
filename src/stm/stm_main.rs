use crossterm::event::KeyCode;
use tui::{backend::Backend, Frame};

use crate::app::Context;
use crate::stm::events::Event;
use crate::stm::state_debug::DebugState;
use crate::stm::state_help::HelpState;
use crate::stm::state_home::HomeState;
use crate::stm::state_search::SearchState;
use crate::stm::state_unknown::UnknownState;
use crate::stm::{State, States};
use crate::ui::core::{centered_rect, clear_box, draw_box, draw_paragraph};

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
            home_st: HomeState::default(),
            search_st: SearchState::default(),

            trace,
        }
    }

    pub fn switch_state(&mut self, to_state: States, ctx: &mut Context) {
        if to_state == States::PreviousOne {
            if let Some(prev_state) = self.previous_st {
                self.current_st = prev_state;
                self.previous_st = None;
            }
        } else {
            self.previous_st = Some(self.current_st);
            self.current_st = to_state;

            match self.current_st {
                States::Home => {
                    if self.home_st.on_enter_first {
                        self.home_st.on_enter_once(ctx);
                    }
                }
                States::Search => {
                    if self.search_st.on_enter_first {
                        self.search_st.on_enter_once(ctx);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn on_event(&mut self, event: Event, ctx: &mut Context) {
        if self.current_st != States::Debug {
            ctx.debug(format!(
                "[STM] on_event {:?} state current:{:?} prev:{:?}",
                event, self.current_st, self.previous_st
            ));
        }

        match (self.current_st, event.clone()) {
            (
                States::Unknown | States::Home | States::Search,
                Event::Key {
                    key_code: KeyCode::Char('D'),
                },
            ) => self.switch_state(States::Debug, ctx),
            (
                States::Home | States::Search,
                Event::Key {
                    key_code: KeyCode::Char('?'),
                },
            ) => {
                self.switch_state(States::Help, ctx);
            }
            (States::Unknown, _) => {
                if let Some(to_state) = self.unknow_st.on_event(event, ctx) {
                    self.switch_state(to_state, ctx);
                }
            }
            (States::Debug, _) => {
                if let Some(to_state) = self.debug_st.on_event(event, ctx) {
                    self.switch_state(to_state, ctx);
                }
            }
            (States::Home, _) => {
                if let Some(to_state) = self.home_st.on_event(event, ctx) {
                    self.switch_state(to_state, ctx);
                }
            }
            (States::Search, _) => {
                if let Some(to_state) = self.search_st.on_event(event, ctx) {
                    self.switch_state(to_state, ctx);
                }
            }
            (States::Help, _) => {
                if let Some(to_state) = self.help_st.on_event(event, ctx) {
                    self.switch_state(to_state, ctx);
                }
            }
            _ => ctx.debug(format!("[STM] on_event {event:?} not match")),
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, ctx: &mut Context) {
        // it Help is the current state, overlay to the previous state ui the help view.
        if self.current_st == States::Help {
            let text = match self.previous_st {
                Some(States::Unknown) => {
                    self.unknow_st.ui(f, ctx);
                    self.unknow_st.help_text()
                }
                Some(States::Debug) => {
                    self.debug_st.ui(f, ctx);
                    self.debug_st.help_text()
                }
                Some(States::Home) => {
                    self.home_st.ui(f, ctx);
                    self.home_st.help_text()
                }
                Some(States::Search) => {
                    self.search_st.ui(f, ctx);
                    self.search_st.help_text()
                }
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
    use crossterm::event::KeyCode;
    use krakenrs::{AssetPairsResponse, AssetsResponse};

    use crate::{kraken::client::MockRestAPI, stm::events::Event};

    use super::*;

    #[test]
    fn test_stm() -> Result<(), String> {
        let mut mock_client = Box::new(MockRestAPI::new());
        mock_client.expect_connect().once().returning(|| Ok(()));
        mock_client
            .expect_list_assets()
            .once()
            .returning(|| Some(AssetsResponse::new()));
        mock_client
            .expect_list_asset_pairs()
            .once()
            .returning(|| Some(AssetPairsResponse::new()));
        let mut ctx = Context::new_for_testing(mock_client);

        let mut stm = MainStm::new("my_stm", false);
        assert_eq!(stm.name, "my_stm");
        assert!(matches!(stm.current_st, States::Unknown));

        // home
        let e = Event::Key {
            key_code: KeyCode::Esc,
        };
        stm.on_event(e, &mut ctx);
        assert!(matches!(stm.current_st, States::Home));

        // help
        let e = Event::Key {
            key_code: KeyCode::Char('?'),
        };
        stm.on_event(e, &mut ctx);
        assert!(matches!(stm.current_st, States::Help));

        // home
        let e = Event::Key {
            key_code: KeyCode::Esc,
        };
        stm.on_event(e, &mut ctx);
        assert!(matches!(stm.current_st, States::Home));

        // search
        let e = Event::Key {
            key_code: KeyCode::Char('f'),
        };
        stm.on_event(e, &mut ctx);
        assert!(matches!(stm.current_st, States::Search));

        // help
        let e = Event::Key {
            key_code: KeyCode::Char('?'),
        };
        stm.on_event(e, &mut ctx);
        assert!(matches!(stm.current_st, States::Help));

        // search
        let e = Event::Key {
            key_code: KeyCode::Esc,
        };
        stm.on_event(e, &mut ctx);
        assert!(matches!(stm.current_st, States::Search));

        // debug
        let e = Event::Key {
            key_code: KeyCode::Char('D'),
        };
        stm.on_event(e, &mut ctx);
        assert!(matches!(stm.current_st, States::Debug));

        // search
        let e = Event::Key {
            key_code: KeyCode::Esc,
        };
        stm.on_event(e, &mut ctx);
        assert!(matches!(stm.current_st, States::Search));

        Ok(())
    }
}
