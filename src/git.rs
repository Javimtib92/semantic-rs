use std::str::from_utf8;

use git2::{ObjectType, Oid, Repository};

pub fn get_tag_head(tag_name: &str) -> Oid {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    let object = match repo.revparse_single(tag_name) {
        Ok(object) => object,
        Err(e) => panic!("failed to get revwalk: {}", e),
    };

    object.id()
}

pub fn get_tags(branch: &str) -> Vec<String> {
    let mut tags = Vec::new();

    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    let branch_ref = match repo.find_branch(branch, git2::BranchType::Remote) {
        Ok(branch) => branch,
        Err(e) => panic!("failed to find branch: {}", e),
    };

    let branch_commit = branch_ref
        .get()
        .peel_to_commit()
        .expect("failed to get last commit");

    repo.tag_foreach(|tag_id, name| {
        if let Ok(object) = repo.find_object(tag_id, Some(ObjectType::Any)) {
            let name = from_utf8(name).expect("Couldn\'t parse tag name");

            if let Some(lightweight_tag) = object.as_commit() {
                if repo
                    .graph_descendant_of(branch_commit.id(), lightweight_tag.id())
                    .unwrap()
                {
                    tags.push(name.to_string());
                }
            } else if let Some(annotated_tag) = object.as_tag() {
                let target = annotated_tag
                    .target()
                    .expect("Couldn\'t obtain tag's target");

                if target.id() == branch_commit.id() {
                    tags.push(name.to_string());
                }
            }
        }
        true
    })
    .expect("Couldn\'t parse tags in the repository");

    tags
}

pub fn get_commits(from: &str, to: &str) -> Vec<String> {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    let mut revwalk = repo.revwalk().expect("Couldn\'t retrieve revwalk");

    let from =
        Oid::from_str(from).expect(&format!("from parameter \"{:?}\" is not a valid SHA", from));

    let to = Oid::from_str(to).expect(&format!("to parameter \"{:?}\" is not a valid SHA", to));

    revwalk.push(to).expect(&format!(
        "Couldn\'t set revwalk root to commit \"{:?}\"",
        to
    ));

    revwalk
        .hide(from)
        .expect(&format!("Couldn\'t hide commit \"{:?}\"", from));

    let mut commits = Vec::new();

    for oid in revwalk {
        if let Ok(oid) = oid {
            let commit = repo.find_commit(oid).expect("Couldn\'t find commit");
            if let Some(message) = commit.message() {
                commits.push(message.to_string());
            }
        }
    }

    commits
}
