use reqwest::RequestBuilder;
use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    jira_host: String,
    api_key: String,
    email: String,
}

impl Client {
    pub fn new(jira_host: &str, api_key: &str, email: &str) -> Self {
        // TODO: URL Parsing and validation
        Client {
            jira_host: jira_host.to_owned(),
            api_key: api_key.to_owned(),
            email: email.to_owned(),
        }
    }

    fn base_url(&self) -> String {
        format!("https://{}", self.jira_host)
    }

    pub fn create_url(&self, uri: &str) -> String {
        format!("{}{}", &self.base_url(), uri)
    }

    pub fn add_credentials_to_req(&self, request: RequestBuilder) -> RequestBuilder {
        request.basic_auth(&self.email, Some(&self.api_key))
    }
}
