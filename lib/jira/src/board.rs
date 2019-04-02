use crate::client::Client;
use crate::common::PaginatedResponse;
use crate::network::send_request;
use crate::sprint::{get_sprint_for_board, Response as SprintResponse};
use std::error::Error;
use std::fmt;
const URI: &'static str = "/rest/agile/1.0/board";

type BoardResponse = PaginatedResponse<Board>;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Board {
  pub id: usize,
  pub name: String,
  pub r#type: String,
  pub location: BoardLocation,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BoardLocation {
  pub displayName: String,
}

impl Board {
  pub fn get_sprints(&self, client: &Client) -> Result<SprintResponse, Box<Error>> {
    get_sprint_for_board(client, self.id)
  }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.location.displayName)
  }
}

pub fn get_boards(client: &Client) -> Result<BoardResponse, Box<Error>> {
  let req = reqwest::Client::new().get(&client.create_url(URI));
  let response: BoardResponse = send_request(client.add_credentials_to_req(req)).json()?;
  Ok(response)
}
