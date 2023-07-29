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

    pub async fn get_current_time_entry(
        &self,
    ) -> Result<endpoints::get_current_time_entry::ResponseBody, Error> {
        let url = endpoints::get_current_time_entry::BASE_URL;
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

    pub async fn get_projects(
        &self,
        params: endpoints::get_projects::RequestBody,
    ) -> Result<endpoints::get_projects::ResponseBody, Error> {
        let mut url = reqwest::Url::parse(endpoints::get_projects::BASE_URL).unwrap();
        if let Some(include_archived) = params.include_archived {
            url.query_pairs_mut().append_pair("include_archived", &include_archived.to_string());
        }
        if let Some(since) = params.since {
            url.query_pairs_mut().append_pair("since", &since.to_string());
        }
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
