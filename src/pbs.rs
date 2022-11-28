use reqwest::{header, Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct DatastoreUsuage {
    pub avail: i64,
    #[serde(alias = "estimated-full-date")]
    pub estimated_full_date: i64,
    pub store: String,
    pub total: i64,
    pub used: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponse {
    pub data: Vec<DatastoreUsuage>,
}

pub fn create_client() -> Client {
    let token_id = env::var("TOKEN_ID").expect("$TOKEN_ID is not set");
    let token_secret = env::var("TOKEN_SECRET").expect("$TOKEN_SECRET is not set");
    let auth_header = format!("PBSAPIToken={}:{}", &token_id, &token_secret);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Authorization",
        header::HeaderValue::from_str(auth_header.as_str()).unwrap(),
    );
    ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .default_headers(headers)
        .build()
        .unwrap()
}

pub async fn get_status_response(client: Client) -> Result<StatusResponse, ()> {
    let base_url = env::var("BASE_URL").expect("$BASE_URL is not set");
    let result = client
        .get(format!("{}/status/datastore-usage/", base_url))
        .send()
        .await;
    match result {
        Ok(result) => {
            let text = result.text().await;
            match text {
                Ok(text) => {
                    let status_response: Result<StatusResponse, serde_json::Error> =
                        serde_json::from_str(text.as_str());
                    match status_response {
                        Ok(response) => Result::Ok(response),
                        Err(error) => {
                            println!("Failed to parse response: {}", error);
                            Err(())
                        }
                    }
                }
                Err(error) => {
                    println!("Failed to get response text: {}", error);
                    Err(())
                }
            }
        }
        Err(error) => {
            println!("Failed to get response: {}", error);
            Err(())
        }
    }
}
