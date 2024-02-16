#![warn(clippy::all, clippy::pedantic)]

use serde::{Deserialize, Serialize};

fn main() {
    use reqwest::blocking::Client;
    let client = Client::new();

    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("{}", request_url);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        "YayCli/0.1".parse().expect("Invalid User-Agent"),
    );

    let resp = client
        .get(&request_url)
        .headers(headers)
        .send()
        .expect("Failed to send request");

    #[derive(Serialize, Deserialize, Debug)]
    struct Stargazer {
        login: String,
        id: u64,
        node_id: String,
        avatar_url: String,
        gravatar_id: String,
        url: String,
        html_url: String,
        followers_url: String,
        following_url: String,
        gists_url: String,
        starred_url: String,
        subscriptions_url: String,
        organizations_url: String,
        repos_url: String,
        events_url: String,
        received_events_url: String,
        r#type: String,
        site_admin: bool,
    }
    let stargazers: Vec<Stargazer> = resp.json().expect("Failed to parse JSON");

    for stargazer in stargazers {
        println!("{:?}", stargazer.gists_url);
    }
}
