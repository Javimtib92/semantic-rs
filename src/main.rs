fn main() {
    let github_actions = std::env::var("GITHUB_ACTIONS");
    let github_event_name = std::env::var("GITHUB_EVENT_NAME");

    let is_ci = github_actions.is_ok();

    let is_pr = github_event_name
        .and_then(|value| Ok(value == "pull_request" || value == "pull_request_target"))
        .unwrap_or(false);

    let branch = match is_pr {
        true => std::env::var("GITHUB_HEAD_REF").ok(),
        false => std::env::var("GITHUB_REF").ok(),
    };
}
