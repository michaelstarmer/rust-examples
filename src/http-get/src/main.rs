/*
 * http-get
 * 
 * To enable all logging: RUST_LOG=trace cargo run
 */
use std::collections::HashMap;

mod logger;

extern crate reqwest;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   logger::init_application();
   
   let URL: &str = "https://redutv-api.vg.no/corona/v1/sheets/norway-region-data?exclude=cases";

   let resp = reqwest::get(URL)
      .await?
      .json::<HashMap<String, String>>()
      // .text_with_charset("utf-8")
      .await?;

   info!("text {:#?}", resp);
   Ok(())

}

   // match reqwest::get(URL) {
   //    Ok(mut response) => {
   //       if response.status() == reqwest::StatusCode::OK {
   //          match response.text() {
   //             Ok(text) => info!("Response text: {}", text),
   //          }
   //       }
   //       else {
   //          warn!("Error in response.");
   //       }
   //    },
   // }
   // }
