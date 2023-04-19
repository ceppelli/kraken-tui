use krakenrs::{
    AssetPairsResponse, AssetsResponse, KrakenRestAPI, KrakenRestConfig, TickerResponse,
};

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait RestAPI {
    fn connect(&mut self) -> Result<(), krakenrs::Error>;
    fn disconnect(&self) -> Result<(), krakenrs::Error>;
    fn list_assets(&self) -> Option<AssetsResponse>;
    fn list_asset_pairs(&self) -> Option<AssetPairsResponse>;
    fn ticker(&self, asset_pair: &str) -> Option<TickerResponse>;
}

#[allow(unused)]
pub struct RestAPIImpl {
    url: String,
    api: Option<KrakenRestAPI>,
}

impl RestAPIImpl {
    pub fn new(url: String) -> RestAPIImpl {
        RestAPIImpl { url, api: None }
    }
}

#[allow(unused)]
impl RestAPI for RestAPIImpl {
    fn connect(&mut self) -> Result<(), krakenrs::Error> {
        let kc_config = KrakenRestConfig::default();
        let api = KrakenRestAPI::try_from(kc_config);
        match api {
            Ok(api) => {
                self.api = Some(api);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn list_assets(&self) -> Option<AssetsResponse> {
        if let Some(api) = &self.api {
            let result = api.assets();
            return match result {
                Ok(assets) => return Some(assets),
                Err(_) => return None,
            };
        }

        None
    }

    fn list_asset_pairs(&self) -> Option<AssetPairsResponse> {
        if let Some(api) = &self.api {
            let result = api.asset_pairs(vec![]);
            return match result {
                Ok(asset_pairs) => Some(asset_pairs),
                Err(e) => None,
            };
        }

        None
    }

    fn ticker(&self, asset_pair: &str) -> Option<TickerResponse> {
        if let Some(api) = &self.api {
            let result = api.ticker(vec![asset_pair.to_owned()]);
            return match result {
                Ok(asset_pairs) => Some(asset_pairs),
                Err(_) => None,
            };
        }

        None
    }

    fn disconnect(&self) -> Result<(), krakenrs::Error> {
        Ok(())
    }
}
