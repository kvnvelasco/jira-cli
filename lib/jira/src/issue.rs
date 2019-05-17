use crate::client::Client;
use crate::common::Paginated;
use crate::network::send_request;
use ansi_term::Colour;
use reqwest;
use serde_derive::{Deserialize, Serialize};
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
    pub description: Option<String>,
    pub status: Status,
    pub updated: String,
    pub issuetype: Type,
    pub assignee: Option<Asignee>,
    pub progress: Progress,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    name: String,
    status_category: StatusCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCategory {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    progress: Option<usize>,
    total: Option<usize>,
    percent: Option<usize>,
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
            "| ({}/{}) | {}",
            Colour::Green.dimmed().underline().paint(assignee),
            Colour::Red
                .dimmed()
                .underline()
                .paint(&self.fields.status.name),
            self.fields.summary,
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
        board_id: usize,
        sprint_id: Option<usize>,
    ) -> Result<(), Box<std::error::Error>> {
        while self.get_number_of_pages() > 0 {
            match sprint_id {
                Some(sprint_id) => {
                    let mut response =
                        get_issues_for_sprint(&client, board_id, sprint_id, self.issues.len())?;
                    self.issues.append(&mut response.issues)
                }
                None => {
                    // get issues for board
                }
            }
        }
        Ok(())
    }
}

impl Issue {
    pub fn from_yml(yaml_string: String) -> Option<Issue> {
        serde_yaml::from_str(&yaml_string).ok()
    }
    pub fn to_yml(&self) -> Option<String> {
        serde_yaml::to_string(&self).ok()
    }
}

pub fn get_issues_for_board(
    client: &Client,
    board_id: usize,
    offset: usize,
) -> Result<Response, Box<std::error::Error>> {
    let uri = format!("/rest/agile/1.0/board/{}/backlog", board_id);
    let mut req = reqwest::Client::new().get(&client.create_url(&uri));
    req = client
        .add_credentials_to_req(req)
        .query(&[("startAt", format!("{}", offset).as_str())]);

    let mut response: Response = send_request(req).json()?;
    response.exhaust(&client, board_id, None)?;
    Ok(response)
}

pub fn get_issues_for_sprint(
    client: &Client,
    board_id: usize,
    sprint_id: usize,
    offset: usize,
) -> Result<Response, Box<std::error::Error>> {
    let uri = format!(
        "/rest/agile/1.0/board/{}/sprint/{}/issue",
        board_id, sprint_id
    );
    let mut req = reqwest::Client::new().get(&client.create_url(&uri));
    req = client
        .add_credentials_to_req(req)
        .query(&[("startAt", format!("{}", offset).as_str())]);

    let mut response: Response = send_request(req).json()?;
    response.exhaust(&client, board_id, Some(sprint_id))?;
    Ok(response)
}

pub fn get_issue(client: &Client, key: &str) -> Option<Issue> {
    let uri = format!("/rest/agile/1.0/issue/{}", key);
    let mut req = reqwest::Client::new().get(&client.create_url(&uri));
    req = client.add_credentials_to_req(req);
    send_request(req).json().ok()
}
