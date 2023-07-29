use crate::endpoints;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

pub struct Auth {
    pub user: String,
    pub password: String,
}

pub struct Client {
    auth: Auth,
    reqwest_client: reqwest::Client,
}

impl Client {
    pub fn new(auth: Auth) -> Self {
        Self {
            auth,
            reqwest_client: reqwest::Client::new(),
        }
    }

    pub async fn get_me(&self) -> Result<endpoints::get_me::ResponseBody, Error> {
        let url = endpoints::get_me::BASE_URL;
        println!("Requesting me, url: {}", url);
        let response = self
            .reqwest_client
            .get(url)
            .basic_auth(&self.auth.user, Some(&self.auth.password))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}
