use crate::kraken::client::Client;
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
  pub kraken_api: Client,
}

impl AppContext {
  pub fn new(app_id: String, app_version: String) -> Self {
    Self {
      app_id,
      app_version,
      model: AppModel::new(),
      kraken_api: Client::new(String::from("https://")),
    }
  }

  pub fn info(&self) -> String {
    format!("AppId:{}, AppVersion:{}\n", self.app_id, self.app_version)
  }

  pub fn debug(&mut self, message: String) {
    self.model.debug_messages_stateful.push(message);
  }
}

// tests
#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_app_context() {
    let ctx = AppContext::new(String::from("app_id"), String::from("app_version"));
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
