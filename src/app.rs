use crate::kraken::client::{Client, ClientImpl};
use crate::ui::list_stateful_widget::StatefulList;

#[derive(Debug)]
pub struct AppModel {
  pub debug_messages_stateful: StatefulList<String>,
  pub kraken_assets_stateful: StatefulList<String>,
}

impl AppModel {
  pub fn new() -> AppModel {
    AppModel {
      debug_messages_stateful: StatefulList::new(),
      kraken_assets_stateful: StatefulList::new(),
    }
  }
}

pub struct AppContext {
  app_id: String,
  app_version: String,

  pub model: AppModel,
  pub kraken_api: Box<dyn Client>,
}

impl AppContext {
  pub fn new(app_id: String, app_version: String) -> Self {
    Self {
      app_id,
      app_version,
      model: AppModel::new(),
      kraken_api: Box::new(ClientImpl::new(String::from("https://"))),
    }
  }

  pub fn info(&self) -> String {
    format!("AppId:{}, AppVersion:{}\n", self.app_id, self.app_version)
  }

  pub fn debug(&mut self, message: String) {
    self.model.debug_messages_stateful.push(message);
  }
}

#[cfg(test)]
mod mock_test {
  use super::{AppContext, AppModel};
  use crate::kraken::client::Client;

  impl AppContext {
    #[allow(unused)]
    pub fn new_for_testing(kraken_api: Box<dyn Client>) -> Self {
      Self {
        app_id: String::from("_app_id_"),
        app_version: String::from("_app_version_"),
        model: AppModel::new(),
        kraken_api,
      }
    }
  }
}

// tests
#[cfg(test)]
mod tests {

  use super::*;
  use crate::kraken::client::MockClient;

  #[test]
  fn test_app_context() {
    let ctx = AppContext::new_for_testing(Box::new(MockClient::new()));
    assert_eq!(ctx.app_id, String::from("app_id"));
    assert_eq!(ctx.app_version, String::from("app_version"));
    assert_eq!(ctx.model.debug_messages_stateful.items.len(), 0);
    assert_eq!(
      ctx.info(),
      String::from("AppId:app_id, AppVersion:app_version\n")
    );
  }

  #[test]
  fn test_app_model_debug() {
    let model = AppModel::new();

    assert_eq!(format!("{model:?}"), "AppModel { debug_messages_stateful: StatefulList { state: ListState { offset: 0, selected: None }, items: [] }, kraken_assets_stateful: StatefulList { state: ListState { offset: 0, selected: None }, items: [] } }");
  }
}
