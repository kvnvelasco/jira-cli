use crate::client::Client;
use crate::common::PaginatedResponse;
use crate::issue::{get_issues_for_sprint, Response as IssueResponse};
use crate::network::send_request;
use reqwest;
use std::error::Error;
use std::fmt;
pub type Response = PaginatedResponse<Sprint>;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Sprint {
  pub id: usize,
  pub state: String,
  pub name: String,
  pub startDate: Option<String>,
  pub endDate: Option<String>,
  pub completeDate: Option<String>,
  pub originBoardId: usize,
  pub goal: String,
}

impl Sprint {
  pub fn get_issues(&self, client: &Client) -> Result<IssueResponse, Box<Error>> {
    get_issues_for_sprint(&client, &self.id, &self.originBoardId, 0)
  }
}

impl fmt::Display for Sprint {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

pub fn get_sprint_for_board(client: &Client, board_id: usize) -> Result<Response, Box<Error>> {
  let uri = format!("/rest/agile/1.0/board/{}/sprint", &board_id);
  let req = reqwest::Client::new().get(&client.create_url(&uri));
  let response: Response = send_request(client.add_credentials_to_req(req)).json()?;
  Ok(response)
}
