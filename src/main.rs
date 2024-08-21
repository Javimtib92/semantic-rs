use semantic_release::git::{get_branches, get_commits, get_tag_head, get_tags, is_ref_exists};

fn main() {
    get_tag_head("v0.0.5");

    get_tags("origin/release-v0.0.15");
    get_tags("origin/release-v0.0.9");

    get_commits(
        "0779705ecc46cbced5059bcbadee7b8d254d4300",
        "3d92276063e6ebb33d63e2d20bf23d405f9d4925",
    );

    get_branches();

    let exists = is_ref_exists("origin/release-v0.0.15");

    println!("{}", exists);
}
