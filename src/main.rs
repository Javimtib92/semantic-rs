use semantic_release::{
    context::Context, get_config::get_config, get_git_auth_url::get_git_auth_url,
    verify_context::verify_context,
};

const COMMIT_NAME: &str = "semantic-release-bot";
const COMMIT_EMAIL: &str = "javimtib92@gmail.com";

fn main() {
    let github_actions = std::env::var("GITHUB_ACTIONS");
    let github_event_name = std::env::var("GITHUB_EVENT_NAME");

    let is_ci = github_actions.is_ok();

    let is_pr = github_event_name
        .map(|value| value == "pull_request" || value == "pull_request_target")
        .unwrap_or(false);

    let branch = match is_pr {
        true => std::env::var("GITHUB_HEAD_REF").expect("Couldnt\'t get GITHUB_HEAD_REF"),
        false => std::env::var("GITHUB_REF").expect("Couldnt\'t get GITHUB_REF"),
    };

    let config = get_config().expect("Couldn\'t get config file");

    let mut context = Context {
        is_ci,
        is_pr,
        branch,
        config,
    };

    run(&mut context);
}

fn run(context: &mut Context) {
    if !context.is_ci && !context.config.dry_run && !context.config.ci {
        // This run was not triggered in a known CI environment, running in dry-run mode.
        context.config.dry_run = true;
    } else {
        // When running on CI, set the commits author and committer info and prevent the `git` CLI to prompt for username/password.

        std::env::set_var("GIT_AUTHOR_NAME", COMMIT_NAME);
        std::env::set_var("GIT_AUTHOR_EMAIL", COMMIT_EMAIL);
        std::env::set_var("GIT_COMMITTER_NAME", COMMIT_NAME);
        std::env::set_var("GIT_COMMITTER_EMAIL", COMMIT_EMAIL);
        std::env::set_var("GIT_ASKPASS", "echo");
        std::env::set_var("GIT_TERMINAL_PROMPT", "0");
    }

    if context.is_ci && context.is_pr && !context.config.ci {
        // This run was triggered by a pull request and therefore a new version won't be published.
        return;
    }

    verify_context(context).expect("Context is not valid");

    context.config.repository_url = get_git_auth_url(context);

    context.config.branches = vec!["todo".to_owned(), "todo".to_owned()];

    if context.config.branches.contains(&context.branch) {
        println!(
            "This test run was triggered on the branch {}, while semantic-release is configured to only publish from {}, therefore a new version won’t be published.", context.branch, context.config.branches.join(", "));

        return;
    }

    println!("continue");
}
