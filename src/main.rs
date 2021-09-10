use std::path::PathBuf;
use git2::Repository;

struct Arguments {
    path: PathBuf
}

fn main() {
    let args = Arguments {
        path: ".".into()
    };
    let repo = match Repository::open(args.path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    println!("{:?}", repo.namespace());
}
