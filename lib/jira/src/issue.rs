use crate::client::Client;
use crate::common::Paginated;
use crate::network::send_request;
use ansi_term::Colour;
use reqwest;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
  start_at: usize,
  max_results: usize,
  total: usize,
  pub issues: Vec<Issue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
  pub id: String,
  pub key: String,
  pub fields: IssueFields,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueFields {
  pub summary: String,
  pub status: Status,
  pub issuetype: Type,
  pub assignee: Option<Asignee>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
  name: String,
  status_category: StatusCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCategory {
  name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
  name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asignee {
  display_name: String,
  name: String,
  key: String,
  // #[serde(alias = "48x48")]
  // avatar_url: String,
}

impl fmt::Display for Issue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let assignee = match &self.fields.assignee {
      Some(assignee) => assignee.display_name.as_str(),
      _ => "",
    };
    write!(
      f,
      "{:<10} {} {}\n{}",
      Colour::Blue.underline().paint(&self.key),
      Colour::Green.dimmed().underline().paint(assignee),
      Colour::Red.dimmed().underline().paint(&self.fields.status.name),
      self.fields.summary
    )
  }
}

impl Paginated for Response {
  fn get_distance_from_top(&self) -> usize {
    let total = self.start_at + self.issues.len();
    if self.total > total {
      self.total - total
    } else {
      0
    }
  }

  fn get_number_of_pages(&self) -> usize {
    match self.get_distance_from_top() {
      0 => 0,
      _ => {
        let distance = self.get_distance_from_top();
        let page_size = self.max_results;
        if distance % page_size == 0 {
          (distance / page_size)
        } else {
          (distance / page_size) + 1
        }
      }
    }
  }
}

impl Response {
  fn exhaust(
    &mut self,
    client: &Client,
    sprint_id: &usize,
    board_id: &usize,
  ) -> Result<(), Box<std::error::Error>> {
    while self.get_number_of_pages() > 0 {
      let mut response = get_issues_for_sprint(
        &client,
        &sprint_id,
        &board_id,
        &self.start_at + &self.max_results,
      )?;
      self.start_at = response.start_at;
      self.issues.append(&mut response.issues)
    }
    Ok(())
  }
}

pub fn get_issues_for_sprint(
  client: &Client,
  sprint_id: &usize,
  board_id: &usize,
  offset: usize,
) -> Result<Response, Box<std::error::Error>> {
  let uri = format!(
    "/rest/agile/1.0/board/{}/sprint/{}/issue",
    board_id, sprint_id
  );
  let mut req = reqwest::Client::new().get(&client.create_url(&uri));
  req = client.add_credentials_to_req(req).query(&[
    ("startAt", format!("{}", offset).as_str()),
    ("jql", "issuetype in (Bug, Sub-task)"),
  ]);

  let mut response: Response = send_request(req).json()?;
  response.exhaust(&client, sprint_id, &board_id)?;
  Ok(response)
}
