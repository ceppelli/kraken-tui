use krakenrs::{AssetPairsResponse, AssetsResponse};

use crate::kraken::client::{RestAPI, RestAPIImpl};
use crate::ui::list_stateful_widget::StatefulList;

#[derive(Debug)]
pub struct Model {
    pub assets: AssetsResponse,
    pub asset_pairs: AssetPairsResponse,
    pub debug_messages_stateful: StatefulList<String>,
    pub assets_stateful: StatefulList<String>,
    pub asset_pairs_stateful: StatefulList<String>,
    pub favorites_asset_pairs_stateful: StatefulList<String>,
    pub favorites_asset_pairs_info_stateful: StatefulList<String>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            assets: AssetsResponse::new(),
            asset_pairs: AssetPairsResponse::new(),
            debug_messages_stateful: StatefulList::new(),
            assets_stateful: StatefulList::new(),
            asset_pairs_stateful: StatefulList::new(),
            favorites_asset_pairs_stateful: StatefulList::new(),
            favorites_asset_pairs_info_stateful: StatefulList::new(),
        }
    }
}

pub struct Context {
    app_id: String,
    app_version: String,

    pub model: Model,
    pub kraken_api: Box<dyn RestAPI>,
}

impl Context {
    pub fn new(app_id: String, app_version: String) -> Self {
        Self {
            app_id,
            app_version,
            model: Model::new(),
            kraken_api: Box::new(RestAPIImpl::new(String::from("https://"))),
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
    use super::{Context, Model};
    use crate::kraken::client::RestAPI;

    impl Context {
        #[allow(unused)]
        pub fn new_for_testing(kraken_api: Box<dyn RestAPI>) -> Self {
            Self {
                app_id: String::from("_app_id_"),
                app_version: String::from("_app_version_"),
                model: Model::new(),
                kraken_api,
            }
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::kraken::client::MockRestAPI;

    #[test]
    fn test_app_context() {
        let ctx = Context::new_for_testing(Box::new(MockRestAPI::new()));
        assert_eq!(ctx.app_id, String::from("_app_id_"));
        assert_eq!(ctx.app_version, String::from("_app_version_"));
        assert_eq!(ctx.model.debug_messages_stateful.items.len(), 0);
        assert_eq!(
            ctx.info(),
            String::from("AppId:_app_id_, AppVersion:_app_version_\n")
        );
    }

    // #[test]
    // fn test_app_model_debug() {
    //   let model = AppModel::new();

    //   assert_eq!(format!("{model:?}"), "AppModel { debug_messages_stateful: StatefulList { state: ListState { offset: 0, selected: None }, items: [] }, kraken_assets_stateful: StatefulList { state: ListState { offset: 0, selected: None }, items: [] } }");
    // }
}
