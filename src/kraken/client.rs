use krakenrs::{KrakenRestAPI, KrakenRestConfig, AssetsResponse};
use serde_json::to_string_pretty;

#[allow(unused)]
pub struct Client {
  url: String,
  api: Option<KrakenRestAPI>,
}

#[allow(unused)]
impl Client {
  pub fn new(url: String) -> Client {
    Client {
      url,
      api: None,
    }
  }

  pub fn connect(& mut self) -> Result<String, krakenrs::Error> {
    let kc_config = KrakenRestConfig::default();
    let api= KrakenRestAPI::try_from(kc_config);
    match api {
        Ok(api) => {
          self.api = Some(api);
        },
        Err(_) => todo!(),
    }

    Ok("OK".to_string())
  }

  pub fn list_assets(&self) -> AssetsResponse {
    if let Some(api) = &self.api {
      let assets = api.assets();
      let result: AssetsResponse = assets.unwrap_or_default();
      return result;
    }

    Default::default()
  }

  pub fn list_pairs(&self) -> Result<String, krakenrs::Error> {

    if let Some(api) = &self.api {

      let s = api.asset_pairs(vec![
        //String::from("BTC/USD"),
        String::from("SOL/BTC"),
      ])?;

      println!("{}", to_string_pretty(&s).unwrap());

      println!(
        "{}",
        to_string_pretty(
          &api
            .ticker(vec![String::from("BTC/USD")])
            .expect("api call failed")
        )
        .unwrap()
      );

    }

    Ok("OK".to_owned())



  }

  pub fn disconnect(&self) -> Result<(), krakenrs::Error> {
    Ok(())
  }
}
