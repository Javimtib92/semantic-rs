use std::collections::HashMap;

use url::Url;

fn is_github_shorthand(arg: &str) -> bool {
    let first_hash = arg.find('#');
    let first_slash = arg.find('/');
    let second_slash = arg[first_slash.unwrap_or(0) + 1..]
        .find('/')
        .map(|s| s + first_slash.unwrap_or(0) + 1);
    let first_colon = arg.find(':');
    let first_space = arg.find(char::is_whitespace);
    let first_at = arg.find('@');

    let space_only_after_hash = first_space.is_none()
        || (first_hash.is_some() && first_space.unwrap() > first_hash.unwrap());
    let at_only_after_hash =
        first_at.is_none() || (first_hash.is_some() && first_at.unwrap() > first_hash.unwrap());
    let colon_only_after_hash = first_colon.is_none()
        || (first_hash.is_some() && first_colon.unwrap() > first_hash.unwrap());
    let second_slash_only_after_hash = second_slash.is_none()
        || (first_hash.is_some() && second_slash.unwrap() > first_hash.unwrap());
    let has_slash = first_slash.is_some() && first_slash.unwrap() > 0;
    let does_not_end_with_slash = if let Some(fh) = first_hash {
        arg.chars().nth(fh - 1) != Some('/')
    } else {
        !arg.ends_with('/')
    };
    let does_not_start_with_dot = !arg.starts_with('.');

    space_only_after_hash
        && has_slash
        && does_not_end_with_slash
        && does_not_start_with_dot
        && at_only_after_hash
        && colon_only_after_hash
        && second_slash_only_after_hash
}

fn parse_url(giturl: &str, protocols: &HashMap<String, Protocol>) -> Option<Url> {
    Url::from_file_path(giturl).ok()
}

#[derive(Debug, Clone)]
struct Protocol {
    auth: bool,
    name: Option<String>,
}

fn main_function(
    giturl: &str,
    opts: HashMap<String, String>,
    git_hosts: &GitHosts,
    protocols: &HashMap<String, Protocol>,
) -> Option<(
    String,
    Option<String>,
    Option<String>,
    HashMap<String, String>,
)> {
    if giturl.is_empty() {
        return None;
    }

    let corrected_url = if is_github_shorthand(giturl) {
        format!("github:{}", giturl)
    } else {
        giturl.to_string()
    };

    let parsed = parse_url(&corrected_url, protocols)?;
    let parsed_protocol = parsed.scheme();
    let parsed_host = parsed.host_str().unwrap_or("").to_string();
    let git_host_shortcut = git_hosts.by_shortcut.get(parsed_protocol);
    let git_host_domain =
        git_hosts
            .by_domain
            .get(if let Some(stripped) = parsed_host.strip_prefix("www.") {
                stripped
            } else {
                &parsed_host
            });

    let git_host_name = git_host_shortcut.or(git_host_domain)?;

    let git_host_info = git_hosts.data.get(git_host_name)?;

    let mut auth = None;
    if let Some(protocol) = protocols.get(parsed_protocol) {
        if protocol.auth && (!parsed.username().is_empty() || parsed.password().is_some()) {
            auth = Some(format!(
                "{}{}",
                parsed.username(),
                parsed
                    .password()
                    .map_or("".to_string(), |p| format!(":{}", p))
            ));
        }
    }

    let default_representation = if git_host_shortcut.is_some() {
        Some("shortcut".to_string())
    } else {
        if !git_host_info
            .protocols
            .contains(&parsed_protocol.to_string())
        {
            return None;
        }

        protocols
            .get(parsed_protocol)
            .and_then(|p| p.name.clone())
            .or(Some(
                parsed_protocol[..parsed_protocol.len() - 1].to_string(),
            ))
    };

    Some((git_host_name.clone(), auth, default_representation, opts))
}

struct GitHosts {
    by_shortcut: HashMap<String, String>,
    by_domain: HashMap<String, String>,
    data: HashMap<String, GitHostInfo>,
}

struct GitHostInfo {
    protocols: Vec<String>,
    extract: fn(Url) -> Option<UrlSegments>,
}

struct UrlSegments {
    user: Option<String>,
    project: Option<String>,
    committish: Option<String>,
}
