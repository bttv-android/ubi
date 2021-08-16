use std::process;
use git2::{Repository, Reference, Oid};


pub fn diff(disass_dir: &String) -> Vec<String> {
    trace!("opening disass repo");
    let repo = match Repository::open(disass_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open disass repo: {}", e),
    };
    trace!("opened disass repo, finding base tag");

    let base = get_tag(&repo, "refs/tags/base");
    let update = get_tag(&repo, "refs/tags/update");

    let diff = repo.diff_tree_to_tree(Some(&base), Some(&update), None).expect("diffing tags");

    let mut vec = vec![];
    for delta in diff.deltas() {

        for file in [delta.old_file(), delta.new_file()] {
        let string = file.path().unwrap().to_str().unwrap().to_string();
            if !string.ends_with(".smali") {
                continue;
            }
            let mut split = string.splitn(2, '/');
            split.next();
            vec.push(split.next().unwrap().to_string());
        }
    }
    return vec;
}

fn get_tag<'a>(repo: &'a Repository, name: &str) -> git2::Tree<'a> {
    trace!("get_tag() {}", name);
    let tag_ref: Reference = match repo.find_reference(name) {
        Ok(base_ref) => base_ref,
        Err(e) => panic!("failed to get ref '{}': {}", name, e),
    };
    trace!("get_tag(): found ref for {}", name);

    if !tag_ref.is_tag() {
        eprintln!("{} found, but not a tag", name);
        process::exit(1);
    }

    let tag: Oid = match tag_ref.target() {
        Some(id) => id,
        None => panic!("could not resolve {} to an oid", name),
    };

    let commit = match repo.find_commit(tag) {
        Ok(commit) => commit,
        Err(_) => panic!("tag {} does not target a commit", name),
    };
    trace!("found commit for {}", name);
    let tree = commit.tree().expect("could not get tree of commit tag");
    trace!("found tree for {}", name);
    return tree;
}
