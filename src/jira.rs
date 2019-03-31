use crate::utils::config::{Config, Session};
use crate::utils::fs::{check_file_exists, file_to_string, write_file};
use serde_yaml;
use std::path::Path;
use url::Url;

fn create_url(session: &Session, resource: Option<&str>) -> String {
  let mut base_url = format!("https://{}", session.domain,);

  base_url = match resource {
    Some(uri) => format!("{}{}", base_url, uri),
    None => base_url,
  };

  Url::parse(&base_url).expect("Invalid URL passed to create_url");

  base_url
}

fn create_get_request(session: &Session, uri: Option<&str>) -> reqwest::RequestBuilder {
  let url = create_url(session, uri);
  reqwest::Client::new()
    .get(&url)
    .basic_auth(&session.email, Some(&session.api_key))
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BoardResponse {
  pub maxResults: usize,
  pub values: Vec<Board>,
}

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

pub fn get_boards(session: &Session) -> BoardResponse {
  let request = create_get_request(session, Some("/rest/agile/1.0/board "));

  let mut response = request.send().expect("Unable to make get boards request");

  validate_api(&response);

  let json: BoardResponse = response.json().expect("Unable to deseralize boards");

  println!("Fetched JIRA Boards: ");
  println!("ID\tType\tName");
  for board in &json.values {
    println!(
      "{}\t{}\t{}",
      board.id, board.r#type, board.location.displayName
    );
  }
  println!("You may set the default board for any future requests with set board default <id>");
  json
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct IssuesResponse {
  pub maxResults: usize,
  pub total: usize,
  pub issues: Vec<Issue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
  pub id: String,
  pub key: String,
  pub fields: IssueFields,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueFields {
  pub summary: String,
}

impl Issue {
  pub fn load(path: &Path) -> Option<Issue> {
    // attempt to deserialize the issue with serde;
    if let false = check_file_exists(&path) {
      return None;
    };

    let file = file_to_string(&path);

    match serde_yaml::from_str::<Issue>(&file) {
      Err(_) => None,
      Ok(issue) => Some(issue),
    }
  }
  pub fn persist(&self, path: &Path) {
    match serde_yaml::to_string(&self) {
      Ok(contents) => write_file(&path, contents),
      Err(_) => {
        println!("Unable to write file");
      }
    }
  }
}

pub fn get_issues(
  config: &Config,
  days_in_past: Option<&usize>,
) -> Result<IssuesResponse, &'static str> {
  let active_board = match &config.context.active_board {
    Some(active) => active,
    None => return Err("No active board selected"),
  };

  let jql = match days_in_past {
    Some(days) => format!("created > -{}d", days),
    None => "created > -14d".to_owned(),
  };

  let url = format!("/rest/agile/1.0/board/{}/issue", &active_board);

  let mut response = create_get_request(&config.session, Some(&url))
    .query(&[("jql", jql)])
    .send()
    .expect("Unable to make network request.");

  validate_api(&response);

  let json: IssuesResponse = response.json().expect("Unable to deserialze issues");

  Ok(json)
}

pub fn validate_api(response: &reqwest::Response) {
  if response.status() == 200 {
    return;
  };
  if response.status() == 401 {
    println!(
      "You are not authorized to query the JIRA API. Please check your session file or run init"
    );
    std::process::exit(2)
  }
  println!(
    "Unable to query the JIRA API. API returned status code: {}",
    response.status()
  );
  std::process::exit(2);
}
