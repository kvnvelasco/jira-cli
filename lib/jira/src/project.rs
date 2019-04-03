use crate::client::Client;
use reqwest::Client as RClient;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
pub const URI: &'static str = "/rest/api/2/project";

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
}

pub fn get_projects(client: &Client) -> Result<Vec<Project>, Box<Error>> {
    let mut request = RClient::new().get(&client.create_url(URI));
    request = client.add_credentials_to_req(request);

    let result: Vec<Project> = request.send()?.json()?;
    Ok(result)
}
