use url::Url;

use crate::{context::Context, git::verify_auth};

pub fn get_git_auth_url(context: &Context) -> String {
    let mut url = Url::parse(&context.config.repository_url).expect("Couldn\'t parse URL");

    let protocol = url.scheme();

    if protocol.contains("http") {
        // Replace `git+https` and `git+http` with `https` or `http`
        let new_scheme = if protocol.contains("https") {
            "https"
        } else {
            "http"
        };

        url.set_scheme(new_scheme).expect("Invalid URL scheme");
    }

    // Test if push is allowed without transforming the URL (e.g. is ssh keys are set up)
    let is_verified: bool = verify_auth(&url.to_string(), &context.branch);

    if !is_verified {
        println!("SSH key auth failed, falling back to https.");

        let token_name = "GITHUB_TOKEN";

        match std::env::var(&token_name) {
            Ok(value) => {
                url.set_username(&token_name)
                    .expect("Couldn\'t set username");

                url.set_password(Some(&value))
                    .expect("Couldn\'t set password");
            }
            Err(err) => panic!("Couldn\'t set token, error: {}", err),
        }
    }

    url.to_string()
}
