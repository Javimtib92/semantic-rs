use std::str::from_utf8;

use git2::{Cred, Direction, ObjectType, Oid, PushOptions, RemoteCallbacks, Repository};

/// Get the commit **SHA** for a given tag.
///
/// # Panics
///
/// Will panic if no repository is found in current directory or any of the parents
/// or it fails to get the reference to the provided tag.
///
/// # Example
///
/// ```
/// get_tag_head("v0.0.5");
/// ```
pub fn get_tag_head(tag_name: &str) -> Oid {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    let object = match repo.revparse_single(tag_name) {
        Ok(object) => object,
        Err(e) => panic!("failed to get reference: {}", e),
    };

    object.id()
}

/// Get all the tags for a given branch.
///
/// # Panics
///
/// Will panic if no repository is found in current directory or any of the parents
/// or it fails to get the branch reference or traverse the repository tags.
///
/// # Example
///
/// ```
/// get_tags("origin/release-v0.0.15");
/// ```
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

/// Retrieve a range of commits.
///
/// # Panics
///
/// Will panic if no repository is found in current directory or any of the parents
/// or it fails to get revwalk the repository or if any of the provided arguments is
/// not a valid SHA commit.
///
/// # Example
///
/// ```
/// get_commits(
///    "0779705ecc46cbced5059bcbadee7b8d254d4300",
///    "3d92276063e6ebb33d63e2d20bf23d405f9d4925",
/// );
/// ```
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

/// Get all the repository branches.
///
/// # Panics
///
/// Will panic if no repository is found in current directory or any of the parents
/// or it fails to get remote branches.
///
/// # Example
///
/// ```
/// get_branches();
/// ```
pub fn get_branches() -> Vec<String> {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    let branches = repo
        .branches(Some(git2::BranchType::Remote))
        .expect("Couldn\'t retrieve any branches for this repository.");

    let branch_names = branches
        .filter_map(|branch_result| {
            match branch_result {
                Ok((branch, _)) => branch
                    .name()
                    .unwrap_or(None)
                    .map(|branch_name| branch_name.to_string()),
                Err(_) => None, // Skip branches that result in an error
            }
        })
        .collect::<Vec<String>>();

    branch_names
}

/// Verify if the `ref` exits
///
/// # Panics
///
/// Will panic if no repository is found in current directory or any of the parents
///
/// # Example
///
/// ```
/// is_ref_exists("origin/release-v0.0.15");
/// ```
pub fn is_ref_exists(reference: &str) -> bool {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    let exists = repo.revparse_single(reference).is_ok();

    exists
}

/// Get the HEAD sha.
///
/// # Panics
///
/// Will panic if no repository is found in current directory or any of the parents, cannot get the HEAD ref or it cannot get the target it points to.
///
/// # Example
///
/// ```
/// get_git_head();
/// ```
pub fn get_git_head() -> Oid {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    let head = repo
        .head()
        .expect("Couldn\'t get HEAD ref for this repository");

    head.target().expect("Couldn't get the target OID")
}

/// Get the repository remote URL.
///
/// # Panics
///
/// Will panic if no repository is found in current directory or any of the parents, cannot get config or cannot get the remote origin url.
///
/// # Example
///
/// ```
/// get_repository_url();
/// ```
pub fn get_repository_url() -> String {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    let config = repo.config().expect("Couldn\'t get the repository config");

    config
        .get_string("remote.origin.url")
        .expect("Couldn\'t get the remote origin url")
}

/// Test if the current working directory is a Git repository.
///
/// # Example
///
/// ```
/// is_git_repo();
/// ```
pub fn is_git_repo() -> bool {
    Repository::open_from_env().is_ok()
}

/// Verify write access authorization to remote repository with
/// push dry-run.
/// # Panics
///
/// Will panic if no repository is found in current directory or any of the parents, cannot get config or cannot create a remote or cannot connect.
///
/// # Example
///
/// ```
/// verify_auth("git@github.com:Javimtib92/papyrust.git", "main");
/// ```
pub fn verify_auth(repository_url: &str, branch: &str) -> bool {
    // Open the repository in the current directory
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    let mut remote = repo
        .remote_anonymous(repository_url)
        .expect("Couldn\'t create a remote with the provided URL");

    // Set up callbacks for authentication
    let mut callbacks = RemoteCallbacks::new();

    let mut ssh_attempts_count = 0;

    callbacks.credentials(|_url, username, allowed| {
        if allowed.contains(git2::CredentialType::SSH_KEY) {
            // If ssh-agent authentication fails, libgit2 will keep
            // calling this callback asking for other authentication
            // methods to try. Make sure we only try ssh-agent once.
            ssh_attempts_count += 1;
            // dbg!(self.ssh_attempts_count);
            let u = username.unwrap_or("git");
            return if ssh_attempts_count == 1 {
                Cred::ssh_key_from_agent(u)
            } else {
                Err(git2::Error::from_str("try with an other username"))
            };
        }

        if allowed.contains(git2::CredentialType::DEFAULT) {
            return git2::Cred::default();
        }

        Err(git2::Error::from_str("no valid authentication available"))
    });

    let mut push_options = PushOptions::new();

    remote
        .connect_auth(Direction::Push, Some(callbacks), None)
        .expect("Couldn't connect");

    remote
        .push(
            &[format!("HEAD:refs/heads/{}", branch)],
            Some(&mut push_options),
        )
        .is_ok()
}
