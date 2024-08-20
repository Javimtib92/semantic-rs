use semantic_release::git::{get_tag_head, get_tags};

fn main() {
    get_tag_head("v0.0.5");

    get_tags("origin/release-v0.0.15");
    get_tags("origin/release-v0.0.9");
}
