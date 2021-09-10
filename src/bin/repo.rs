use git2::Repository;
use git2::DiffFormat;
use std::path::PathBuf;

struct Arguments {
    path: PathBuf,
}
fn main() {
    let args = Arguments { path: ".".into() };
    let repo = match Repository::open(args.path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let mut revwalker = repo.revwalk().expect("Unable to create Revwalker.");
    revwalker.simplify_first_parent().expect("Settings error");
    revwalker.push_head().expect("Couldn't push head.");
    let oids: Vec<_> = revwalker
        .map(|oid| oid.and_then(|oid| repo.find_commit(oid).and_then(|commit| commit.tree())))
        .collect();
    for pair in oids.chunks(2) {
        if let [prev, next] = pair {
            let diff = repo
                .diff_tree_to_tree(
                    Some(prev.as_ref().unwrap()),
                    Some(next.as_ref().unwrap()),
                    None,
                )
                .expect("Couldn't diff.");
            println!("{:?}", diff.stats());
            diff.print(DiffFormat::Patch, |delta, hunk, line| {
                println!("Hunk: {:?}", String::from_utf8_lossy(hunk.map(|h| h.header()).unwrap_or_default()));
                println!("Line: {:?}", String::from_utf8_lossy(line.content()));
                true
            });
        }
    }
}