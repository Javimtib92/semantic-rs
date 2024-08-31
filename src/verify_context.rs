use std::error::Error;

use crate::{context::Context, git::is_git_repo};

/// Checks if provided context is valid.
///

pub fn verify_context(context: &Context) -> Result<(), Box<dyn Error>> {
    let mut errors: Vec<String> = vec![];

    if !is_git_repo() {
        errors.push("ENOGITREPO".to_owned());
    } else if context.config.repository_url.is_empty() {
        errors.push("ENOREPOURL".to_owned());
    }

    // TODO: add stuff about compile git tag template
    //

    // TODO: validate branches

    if errors.is_empty() {
        return Err(errors.concat().into());
    }

    Ok(())
}
